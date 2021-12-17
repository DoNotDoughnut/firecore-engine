pub extern crate firecore_battle as battle;
pub extern crate firecore_pokedex_engine as pokedex;

use std::{collections::VecDeque, fmt::Debug, hash::Hash, ops::Deref, rc::Rc};

use context::BattleGuiContext;

use pokedex::engine::log::{self, debug, warn};

use pokedex::{
    engine::{
        graphics::Color,
        math::{vec2, Vec2},
        utils::{Completable, Entity, HashMap, Reset},
        Context,
    },
    gui::{bag::BagGui, party::PartyGui},
    item::{bag::OwnedBag, usage::ItemUsageKind, Item},
    moves::{Move, MoveTarget},
    pokemon::{owned::OwnedPokemon, party::Party, Pokemon},
    types::Effective,
    Dex, Initializable, PokedexClientData,
};

use battle::{
    data::{BattleData, BattleType},
    endpoint::{MpscClient, MpscEndpoint},
    message::{ClientMessage, ServerMessage},
    moves::{damage::ClientDamage, BattleMove, ClientMove, ClientMoveAction},
    party::PlayerParty,
    pokemon::{Indexed, PokemonIdentifier},
    prelude::{FailedAction, StartableAction},
};
use ui::view::ActivePlayer;
use view::GuiPokemonView;

use self::{
    ui::{
        panels::BattlePanels,
        view::{GuiLocalPlayer, GuiRemotePlayer},
        BattleGui,
    },
    view::PlayerView,
};

pub mod action;
pub mod context;
pub mod transition;
pub mod ui;
pub mod view;

use action::*;

use self::transition::TransitionState;

pub struct BattlePlayerGui<
    ID: Default + Eq + Hash,
    P: Deref<Target = Pokemon> + Clone,
    M: Deref<Target = Move> + Clone,
    I: Deref<Target = Item> + Clone,
> {
    party: Rc<PartyGui>,
    bag: Rc<BagGui>,
    pub gui: BattleGui<M>,

    state: BattlePlayerState<ID, M>,
    should_select: bool,

    pub data: BattleData,

    pub local: GuiLocalPlayer<ID, P, M, I>,
    pub remotes: HashMap<ID, GuiRemotePlayer<ID, P>>,

    client: MpscClient<ID>,
    endpoint: MpscEndpoint<ID>,
}

#[derive(Debug)]
struct MoveQueue<ID, M: Deref<Target = Move>> {
    actions: VecDeque<Indexed<ID, BattleClientGuiAction<ID, M>>>,
    current: Option<Indexed<ID, BattleClientGuiCurrent<ID>>>,
}

#[derive(Debug)]
enum BattlePlayerState<ID, M: Deref<Target = Move>> {
    WaitToStart,
    Opening(TransitionState),
    Introduction(TransitionState),
    WaitToSelect,
    /// Current, Max
    Select(usize, usize),
    Moving(MoveQueue<ID, M>),
    PlayerEnd,
    GameEnd(Option<ID>),
}

impl<
        ID: Default + Clone + Debug + Hash + Eq,
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    > BattlePlayerGui<ID, P, M, I>
{
    pub fn new(
        ctx: &mut Context,
        btl: &BattleGuiContext,
        party: Rc<PartyGui>,
        bag: Rc<BagGui>,
    ) -> Self
    where
        ID: Default,
    {
        let (client, endpoint) = battle::endpoint::create();

        Self {
            party,
            bag,
            gui: BattleGui::new(ctx, btl),
            state: BattlePlayerState::WaitToStart,
            should_select: false,
            data: Default::default(),
            local: ActivePlayer::new(PlayerParty::new(
                Default::default(),
                None,
                Default::default(),
                Default::default(),
            )),
            remotes: Default::default(),
            client,
            endpoint,
        }
    }

    pub fn endpoint(&self) -> &MpscEndpoint<ID> {
        &self.endpoint
    }

    pub fn winner(&self) -> Option<Option<&ID>> {
        if let BattlePlayerState::GameEnd(w) = &self.state {
            Some(w.as_ref())
        } else {
            None
        }
    }

    pub fn battling(&self) -> bool {
        !matches!(
            self.state,
            BattlePlayerState::WaitToStart
                | BattlePlayerState::Opening(..)
                | BattlePlayerState::Introduction(..)
        )
    }

    pub fn start(&mut self, transition: bool) {
        self.state = match transition {
            true => BattlePlayerState::Opening(TransitionState::default()),
            false => BattlePlayerState::WaitToSelect,
        };
    }

    pub fn forfeit(&mut self) {
        self.client.send(ClientMessage::Forfeit);
    }

    pub fn process<'d>(
        &mut self,
        random: &mut impl rand::Rng,
        data: &PokedexClientData,
        btl: &BattleGuiContext,
        pokedex: &'d dyn Dex<'d, Pokemon, P>,
        movedex: &'d dyn Dex<'d, Move, M>,
        itemdex: &'d dyn Dex<'d, Item, I>,
        party: &mut Party<OwnedPokemon<P, M, I>>,
    ) {
        while let Ok(message) = self.client.receiver.try_recv() {
            match message {
                ServerMessage::Begin(client) => {
                    self.local.player = battle::party::PlayerParty {
                        name: client.name,
                        id: client.id,
                        active: client.active,
                        pokemon: party.clone(),
                    };
                    self.remotes = client
                        .remotes
                        .into_iter()
                        .map(|player| {
                            (
                                player.id.clone(),
                                ActivePlayer::new(PlayerParty {
                                    id: player.id,
                                    name: player.name,
                                    active: player.active,
                                    pokemon: player
                                        .pokemon
                                        .into_iter()
                                        .flat_map(|u| u.map(|u| u.init(pokedex)))
                                        .collect(),
                                }),
                            )
                        })
                        .collect();
                    self.data = client.data;
                    self.local.init(btl, data);
                    for remote in self.remotes.values_mut() {
                        remote.init(btl, data);
                    }
                }
                ServerMessage::Start(action) => match action {
                    StartableAction::Selecting => {
                        self.should_select = true;
                        self.gui.panel.despawn();
                    }
                    StartableAction::Turns(queue) => {
                        self.state = BattlePlayerState::Moving(MoveQueue {
                            actions: queue
                                .into_iter()
                                .map(|a| Indexed(a.0, BattleClientGuiAction::Action(a.1)))
                                .collect(),
                            current: None,
                        });
                        self.gui.text.clear();
                        self.gui.text.spawn();
                    }
                },
                ServerMessage::Replace(Indexed(pokemon, new)) => {
                    dbg!(&pokemon);
                    dbg!(new);
                    match &mut self.state {
                        BattlePlayerState::Moving(queue) => {
                            queue.actions.push_back(Indexed(
                                pokemon,
                                BattleClientGuiAction::Replace(Some(new)),
                            ));
                        }
                        _ => {
                            if let Some((renderer, pokemon)) =
                                match pokemon.team() == self.local.player.id() {
                                    true => {
                                        self.local.player.replace(pokemon.index(), Some(new));
                                        let renderer = &mut self.local.renderer[pokemon.index()];
                                        let pokemon = self.local.player.active(pokemon.index());
                                        let id = pokemon.map(|p| p.pokemon.id);
                                        renderer.status.update_gui(pokemon, None, true);
                                        Some((renderer, id))
                                    }
                                    false => {
                                        if let Some(remote) = self.remotes.get_mut(pokemon.team()) {
                                            remote.player.replace(pokemon.index(), Some(new));
                                            let renderer = &mut remote.renderer[pokemon.index()];
                                            let pokemon = remote
                                                .player
                                                .active(pokemon.index())
                                                .map(|u| u as &dyn GuiPokemonView<P, M, I>);
                                            let id = pokemon.map(|v| v.pokemon().id);
                                            renderer.status.update_gui_view(pokemon, None, true);
                                            Some((renderer, id))
                                        } else {
                                            None
                                        }
                                    }
                                }
                            {
                                renderer.pokemon.new_pokemon(data, pokemon);
                            }
                        }
                    }
                }
                ServerMessage::AddRemote(Indexed(target, unknown)) => {
                    if let Some(party) = self.remotes.get_mut(target.team()) {
                        party.player.add(target.index(), unknown.init(pokedex));
                    }
                }
                // ServerMessage::Winner(player) => {
                //     self.state = BattlePlayerState::Winner(player);
                //     for (index, pokemon) in self.local.party.pokemon.iter().enumerate() {
                //         party[index] = pokemon.clone();
                //     }
                // }
                ServerMessage::Ping(p) => log::warn!("TODO: server ping message ({:?})", p),
                ServerMessage::Fail(action) => match action {
                    FailedAction::Move(i) | FailedAction::Switch(i) => match &self.state {
                        BattlePlayerState::Select(..) => {
                            self.gui.panel.despawn();
                            self.state =
                                BattlePlayerState::Select(0, self.local.player.active.len());
                        }
                        _ => self.state = BattlePlayerState::Select(i, i + 1),
                    },
                    FailedAction::Replace(index) => {
                        debug!("cannot replace pokemon at active index {}", index);
                    }
                },
                ServerMessage::Catch(instance) => {
                    match instance.init(random, pokedex, movedex, itemdex) {
                        Some(instance) => if let Ok(_) = party.try_push(instance) {},
                        None => warn!("Could not initialize caught pokemon."),
                    }
                }
                ServerMessage::PlayerEnd(..) => {
                    self.state = BattlePlayerState::PlayerEnd;
                }
                ServerMessage::GameEnd(winner) => {
                    self.state = BattlePlayerState::GameEnd(winner);
                } // ServerMessage::AddMove(pokemon, index, move_ref) => if pokemon.team == self.local.party.id {
                  //     if let Some(pokemon) = self.local.party.pokemon.get_mut(pokemon.index) {
                  //         debug!("to - do: set move to its index.");
                  //         if let Err(err) = pokemon.moves.try_push(MoveInstance::new(move_ref)) {
                  //             warn!("Cannot add moves to {} because it has maximum number of moves. error: {}", pokemon.name(), err)
                  //         }
                  //     }
                  // }
            }
        }
    }

    pub fn update<'d>(
        &mut self,
        ctx: &Context,
        dex: &PokedexClientData,
        pokedex: &'d dyn Dex<'d, Pokemon, P>,
        movedex: &'d dyn Dex<'d, Move, M>,
        itemdex: &'d dyn Dex<'d, Item, I>,
        delta: f32,
        bag: &mut OwnedBag<I>,
    ) {
        match &mut self.state {
            BattlePlayerState::WaitToStart => (),
            BattlePlayerState::Opening(state) => match state {
                TransitionState::Begin => {
                    self.gui.opener.begin(
                        dex,
                        state,
                        self.data.type_,
                        &self.remotes.values().next().unwrap(),
                    );
                    if !matches!(self.data.type_, BattleType::Wild) {
                        self.gui.trainer.spawn(
                            self.local.player.pokemon.len(),
                            self.remotes.values().next().unwrap().player.pokemon.len(),
                        );
                    }
                    self.update(ctx, dex, pokedex, movedex, itemdex, delta, bag);
                }
                TransitionState::Run => self.gui.opener.update::<ID, P>(state, delta),
                TransitionState::End => {
                    self.state = BattlePlayerState::Introduction(TransitionState::default());
                    self.update(ctx, dex, pokedex, movedex, itemdex, delta, bag);
                }
            },
            BattlePlayerState::Introduction(state) => match state {
                TransitionState::Begin => {
                    self.gui.introduction.begin(
                        dex,
                        state,
                        self.data.type_,
                        &self.local,
                        &self.remotes.values().next().unwrap(),
                        &mut self.gui.text,
                    );
                    self.update(ctx, dex, pokedex, movedex, itemdex, delta, bag);
                }
                TransitionState::Run => {
                    self.gui.introduction.update(
                        state,
                        ctx,
                        delta,
                        &mut self.local,
                        &mut self.remotes.values_mut().next().unwrap(),
                        &mut self.gui.text,
                    );
                    self.gui.trainer.update(delta);
                    if self.gui.text.page() > 0
                        && !self.gui.trainer.ending()
                        && !matches!(self.data.type_, BattleType::Wild)
                    {
                        self.gui.trainer.end();
                    }
                }
                TransitionState::End => {
                    self.gui.introduction.end(&mut self.gui.text);
                    self.gui.trainer.despawn();
                    self.state = BattlePlayerState::WaitToSelect;
                    self.update(ctx, dex, pokedex, movedex, itemdex, delta, bag);
                }
            },
            BattlePlayerState::WaitToSelect => {
                if self.should_select {
                    self.should_select = false;
                    self.state = BattlePlayerState::Select(0, self.local.player.active.len());
                }
            }
            BattlePlayerState::Select(current, max) => {
                self.gui.bounce.update(delta);
                match current < max {
                    true => match self.local.player.active.get(*current) {
                        Some(index) => match index {
                            Some(index) => {
                                let pokemon = &self.local.player.pokemon[*index];
                                match self.gui.panel.alive() {
                                    true => {
                                        // Checks if a move is queued from an action done in the GUI

                                        if self.bag.alive() {
                                            self.bag.input(ctx, &mut bag.items);
                                            if let Some(item) =
                                                self.bag.take_selected_despawn(&mut bag.items)
                                            {
                                                match &item.usage.kind {
                                                    ItemUsageKind::Actions(..) => todo!(),
                                                    ItemUsageKind::Script => {
                                                        todo!("user targeting")
                                                    }
                                                    ItemUsageKind::Pokeball => {
                                                        self.gui.panel.active = BattlePanels::Target(
                                                            MoveTarget::Opponent,
                                                            Some(item.id),
                                                        )
                                                    }
                                                    ItemUsageKind::None => {
                                                        todo!("make item unusable")
                                                    }
                                                    // ItemUsageKind::Pokeball => ,
                                                    // ItemUsageKind::Script(..) => ,
                                                    // ItemUsageKind::None => ,
                                                }
                                            }
                                        } else if self.party.alive() {
                                            self.party.input(
                                                ctx,
                                                dex,
                                                self.local.player.pokemon.as_mut_slice(),
                                            );
                                            self.party.update(delta);
                                            if let Some(selected) = self.party.take_selected() {
                                                self.party.despawn();
                                                self.client.send(ClientMessage::Move(
                                                    *current,
                                                    BattleMove::Switch(selected),
                                                ));
                                                *current += 1;
                                                self.gui.panel.despawn();
                                            }
                                        } else if let Some(panels) =
                                            self.gui.panel.input(ctx, pokemon)
                                        {
                                            match panels {
                                                BattlePanels::Main => {
                                                    match self.gui.panel.battle.cursor {
                                                        0 => self.gui.panel.active = BattlePanels::Fight,
                                                        1 => self.bag.spawn(),
                                                        2 => self.party.spawn(dex, &self.local.player.pokemon, Some(false), true),
                                                        3 => if matches!(self.data.type_, BattleType::Wild) {
                                                            self.client.send(ClientMessage::Forfeit);
                                                        },
                                                        _ => unreachable!(),
                                                    }
                                                }
                                                BattlePanels::Fight => match pokemon.moves.get(self.gui.panel.fight.moves.cursor) {
                                                    Some(instance) => match instance.try_use() {
                                                        Some(move_ref) => {
                                                            match move_ref.target {
                                                                MoveTarget::Opponent | MoveTarget::Any => {
                                                                    let p = &self.remotes.values().next().unwrap().player;
                                                                    self.gui.panel.target(p as &dyn PlayerView<ID, P, M, I>);
                                                                    self.gui.panel.active = BattlePanels::Target(move_ref.target, None);
                                                                },
                                                                MoveTarget::Ally | MoveTarget::UserOrAlly => {
                                                                    self.gui.panel.target(&self.local.player);
                                                                    self.gui.panel.active = BattlePanels::Target(move_ref.target, None);
                                                                }
                                                                _ => {
                                                                    self.client.send(
                                                                        ClientMessage::Move(
                                                                            *current,
                                                                            BattleMove::Move(
                                                                                self.gui.panel.fight.moves.cursor,
                                                                                None,
                                                                            )
                                                                        )
                                                                    );
                                                                    *current += 1;
                                                                    self.gui.panel.despawn();
                                                                }
                                                            }
                                                        }
                                                        None => warn!("Pokemon is out of Power Points for this move!"),
                                                    }
                                                    None => warn!("Could not get move at cursor!"),
                                                }
                                                BattlePanels::Target(target, item) => {
                                                    self.client.send(
                                                        ClientMessage::Move(
                                                            *current,
                                                            match item {
                                                                Some(item) => BattleMove::UseItem(Indexed(
                                                                    match target {
                                                                        MoveTarget::Opponent => PokemonIdentifier(self.remotes.keys().next().unwrap().clone(), self.gui.panel.targets.cursor),
                                                                        _ => unreachable!(),
                                                                    },
                                                                    item,
                                                                )
                                                                ),
                                                                None => BattleMove::Move(self.gui.panel.fight.moves.cursor, Some(PokemonIdentifier(self.remotes.keys().next().unwrap().clone(), self.gui.panel.targets.cursor))),
                                                            }
                                                        )
                                                    );
                                                    *current += 1;
                                                    self.gui.panel.despawn();
                                                }
                                            }
                                        }
                                    }
                                    false => {
                                        self.gui.panel.user(pokemon);
                                        self.gui.panel.spawn();
                                    }
                                }
                            }
                            None => *current += 1,
                        },
                        None => {
                            self.gui.panel.despawn();
                        }
                    },
                    false => self.gui.panel.despawn(),
                }
            }
            BattlePlayerState::Moving(queue) => {
                match &mut queue.current {
                    None => {
                        match queue.actions.pop_front() {
                            None => {
                                // self.messages.send(ClientMessage::FinishedTurnQueue);
                                self.state = BattlePlayerState::WaitToSelect;
                            }
                            Some(Indexed(user_id, action)) => {
                                // log::trace!("set current");

                                self.gui.text.clear();
                                self.gui.text.reset();

                                let user = match user_id.team() == self.local.player.id() {
                                    true => Some((
                                        &mut self.local.player as &mut dyn PlayerView<ID, P, M, I>,
                                        &mut self.local.renderer,
                                    )),
                                    false => self
                                        .remotes
                                        .get_mut(user_id.team())
                                        .map(|p| (&mut p.player as _, &mut p.renderer)),
                                };

                                match user {
                                    Some((user, user_ui)) => {
                                        if user.active(user_id.index()).is_some()
                                            || !action.requires_user()
                                        {
                                            if let Some(action) = match action {
                                                BattleClientGuiAction::Action(action) => {
                                                    match action {
                                                        ClientMove::<ID>::Move(
                                                            pokemon_move,
                                                            pp,
                                                            targets,
                                                        ) => {
                                                            // log::trace!("set current: client move");

                                                            match movedex.try_get(&pokemon_move) {
                                                                Some(pokemon_move) => {
                                                                    {
                                                                        let user_active = user
                                                                            .active_mut(
                                                                                user_id.index(),
                                                                            )
                                                                            .unwrap();

                                                                        user_active.decrement_pp(
                                                                            &pokemon_move,
                                                                            pp,
                                                                        );

                                                                        ui::text::on_move(
                                                                            &mut self.gui.text,
                                                                            &pokemon_move,
                                                                            user_active.name(),
                                                                        );

                                                                        // user_active.decrement_pp(pp);
                                                                    }

                                                                    drop(user);
                                                                    drop(user_ui);

                                                                    let mut faint = Vec::new();

                                                                    for Indexed(
                                                                        target_id,
                                                                        action,
                                                                    ) in &targets
                                                                    {
                                                                        let userui = &mut self
                                                                            .local
                                                                            .renderer
                                                                            [target_id.index()];

                                                                        let target = match target_id.team() == self.local.player.id() {
                                                                        true => self.local.player.active_mut(target_id.index()).map(|p| (p as &mut dyn GuiPokemonView<P, M, I>, userui)),
                                                                        false => self.remotes.get_mut(target_id.team()).map(|remote| {
                                                                            let ui = &mut remote.renderer[target_id.index()];
                                                                            remote.player.active_mut(target_id.index()).map(|p| (p as _, ui))
                                                                        }).flatten(),
                                                                    };

                                                                        if let Some((
                                                                            target,
                                                                            target_ui,
                                                                        )) = target
                                                                        {
                                                                            match *action {
                                                                                ClientMoveAction::SetHP(result) => {
                                                                                    target.set_hp(result.damage());
                                                                                    if let ClientDamage::Result(result) = result {
                                                                                        match result.damage > 0.0 {
                                                                                            true => target_ui.pokemon.flicker(),
                                                                                            false => faint.push(target_id),
                                                                                        }
                                                                                        if result.effective != Effective::Effective {
                                                                                            ui::text::on_effective(&mut self.gui.text, &result.effective)
                                                                                        }
                                                                                        if result.crit {
                                                                                            ui::text::on_crit(&mut self.gui.text);
                                                                                        }
                                                                                    }
                                                                                },
                                                                                ClientMoveAction::Error => ui::text::on_fail(&mut self.gui.text, vec![format!("{} cannot use move", target.name()), format!("{}, as there was an error.", pokemon_move.name)]),
                                                                                ClientMoveAction::Miss => ui::text::on_miss(&mut self.gui.text, target.name()),
                                                                                ClientMoveAction::SetExp(experience, level) => {
                                                                                    let previous = target.level();
                                                                                    target.set_level(level);
                                                                                    target.set_exp(experience);
                                                                                    if let Some(user_pokemon) = target.instance() {
                                                                                        let moves = user_pokemon.on_level_up(movedex, previous).flat_map(|id| movedex.try_get(&id)).collect();
                                                                                        queue.actions.push_front(Indexed(target_id.clone(), BattleClientGuiAction::SetExp(previous, experience, moves)));
                                                                                    }
                                                                                }
                                                                                ClientMoveAction::Flinch => ui::text::on_flinch(&mut self.gui.text, target.name()),
                                                                                ClientMoveAction::AddStat(stat, stage) => ui::text::on_stat_stage(&mut self.gui.text, target.name(), stat, stage),
                                                                                ClientMoveAction::Ailment(ailment) => {
                                                                                    target.set_ailment(ailment);
                                                                                    ui::text::on_status(&mut self.gui.text, target.name(), ailment.ailment);
                                                                                }
                                                                            }

                                                                            match target.instance() {
                                                                            Some(i) => target_ui.status.update_gui(Some(i), None, false),
                                                                            None => target_ui.status.update_gui_view(Some(target as _), None, false),
                                                                        }
                                                                        } else {
                                                                            // target_ui.status.update_gui(None, None, false);
                                                                        }
                                                                    }

                                                                    for target_id in faint {
                                                                        queue.actions.push_front(
                                                                        Indexed(target_id.clone(), BattleClientGuiAction::Faint)
                                                                    )
                                                                    }

                                                                    Some(BattleClientGuiCurrent::Move(targets))
                                                                }
                                                                None => None,
                                                            }
                                                        }
                                                        ClientMove::UseItem(Indexed(
                                                            target,
                                                            item,
                                                        )) => {
                                                            if let Some(item) =
                                                                itemdex.try_get(&item)
                                                            {
                                                                if let Some(pokemon) = match &item
                                                                    .usage
                                                                    .kind
                                                                {
                                                                    ItemUsageKind::Script
                                                                    | ItemUsageKind::Actions(..) => {
                                                                        user.active(target.index())
                                                                    }
                                                                    ItemUsageKind::Pokeball => self
                                                                        .remotes
                                                                        .get(target.team())
                                                                        .map(|p| {
                                                                            p.player.active(
                                                                                target.index(),
                                                                            )
                                                                        })
                                                                        .flatten()
                                                                        .map(|p| p as _),
                                                                    ItemUsageKind::None => None,
                                                                } {
                                                                    if let ItemUsageKind::Pokeball =
                                                                        &item.usage.kind
                                                                    {
                                                                        // self.messages.push(ClientMessage::RequestPokemon(index));
                                                                        queue.actions.push_front(Indexed(target.clone(), BattleClientGuiAction::Catch));
                                                                    }
                                                                    ui::text::on_item(
                                                                        &mut self.gui.text,
                                                                        pokemon.name(),
                                                                        &item,
                                                                    );
                                                                }
                                                                Some(
                                                                    BattleClientGuiCurrent::UseItem(
                                                                        target,
                                                                    ),
                                                                )
                                                            } else {
                                                                None
                                                            }
                                                        }
                                                        ClientMove::Switch(index) => {
                                                            let coming = user
                                                                .pokemon(index)
                                                                .map(|v| v.name())
                                                                .unwrap_or("Unknown");
                                                            ui::text::on_switch(
                                                                &mut self.gui.text,
                                                                user.active(user_id.index())
                                                                    .map(|v| v.name())
                                                                    .unwrap_or("Unknown"),
                                                                coming,
                                                            );
                                                            Some(BattleClientGuiCurrent::Switch(
                                                                index,
                                                            ))
                                                        }
                                                    }
                                                }
                                                BattleClientGuiAction::Faint => {
                                                    let is_player = user_id.team() == user.id();
                                                    let target =
                                                        user.active_mut(user_id.index()).unwrap();
                                                    target.set_hp(0.0);
                                                    ui::text::on_faint(
                                                        &mut self.gui.text,
                                                        matches!(self.data.type_, BattleType::Wild),
                                                        is_player,
                                                        target.name(),
                                                    );
                                                    user_ui[user_id.index()].pokemon.faint();
                                                    Some(BattleClientGuiCurrent::Faint)
                                                }
                                                BattleClientGuiAction::Catch => {
                                                    match self.remotes.get_mut(user_id.team()) {
                                                        Some(remote) => {
                                                            if let Some(pokemon) = remote
                                                                .player
                                                                .active(user_id.index())
                                                            {
                                                                ui::text::on_catch(
                                                                    &mut self.gui.text,
                                                                    view::BasePokemonView::name(
                                                                        pokemon,
                                                                    ),
                                                                );
                                                            }
                                                            // if let Some(pokemon) = pokemon {
                                                            remote
                                                                .player
                                                                .replace(user_id.index(), None);
                                                            let renderer = &mut remote.renderer
                                                                [user_id.index()];
                                                            renderer
                                                                .status
                                                                .update_gui_view::<P, M, I>(
                                                                    None, None, false,
                                                                );
                                                            renderer.pokemon.new_pokemon(dex, None);
                                                            // }
                                                            Some(BattleClientGuiCurrent::Catch)
                                                        }
                                                        None => None,
                                                    }
                                                }
                                                BattleClientGuiAction::Replace(new) => {
                                                    ui::text::on_replace(
                                                        &mut self.gui.text,
                                                        user.name(),
                                                        new.map(|index| {
                                                            user.pokemon(index).map(|v| v.name())
                                                        })
                                                        .flatten(),
                                                    );
                                                    user.replace(user_id.index(), new);
                                                    Some(BattleClientGuiCurrent::Replace(
                                                        user_id.index(),
                                                        false,
                                                    ))
                                                }
                                                // To - do: experience spreading
                                                BattleClientGuiAction::SetExp(
                                                    previous,
                                                    experience,
                                                    moves,
                                                ) => match user.active_mut(user_id.index()) {
                                                    Some(pokemon) => {
                                                        ui::text::on_gain_exp(
                                                            &mut self.gui.text,
                                                            pokemon.name(),
                                                            experience,
                                                            pokemon.level(),
                                                        );
                                                        let status =
                                                            &mut user_ui[user_id.index()].status;
                                                        match pokemon.instance() {
                                                            Some(p) => status.update_gui(
                                                                Some(p),
                                                                Some(previous),
                                                                false,
                                                            ),
                                                            None => status.update_gui_view(
                                                                Some(pokemon),
                                                                Some(previous),
                                                                false,
                                                            ),
                                                        }
                                                        queue.actions.push_front(Indexed(
                                                            user_id.clone(),
                                                            BattleClientGuiAction::LevelUp(moves),
                                                        ));
                                                        Some(BattleClientGuiCurrent::SetExp)
                                                    }
                                                    None => None,
                                                },
                                                BattleClientGuiAction::LevelUp(moves) => match user
                                                    .active_mut(user_id.index())
                                                    .map(|v| v.instance())
                                                    .flatten()
                                                {
                                                    Some(instance) => match moves.is_empty() {
                                                        false => {
                                                            self.gui.level_up.spawn(
                                                                instance,
                                                                &mut self.gui.text,
                                                                moves,
                                                            );
                                                            Some(BattleClientGuiCurrent::LevelUp)
                                                        }
                                                        true => None,
                                                    },
                                                    None => None,
                                                }, // ClientMove::Catch(index) => {
                                                   //     if let Some(target) = match index.team {
                                                   //         Team::Player => &user.active[index.active],
                                                   //         Team::Opponent => &other.active[index.active],
                                                   //     }.pokemon.as_ref() {
                                                   //         ui::text::on_catch(text, target);
                                                   //     }
                                                   // }
                                            } {
                                                queue.current = Some(Indexed(user_id, action));
                                            } else {
                                                self.update(
                                                    ctx, dex, pokedex, movedex, itemdex, delta, bag,
                                                );
                                            }
                                        }
                                    }
                                    None => log::warn!("cannot get user {:?}", user_id),
                                }
                            }
                        }
                    }
                    Some(Indexed(user_id, action)) => {
                        // log::trace!("update current");

                        let user = if user_id.team() == self.local.player.id() {
                            Some((
                                &mut self.local.player as &mut dyn PlayerView<ID, P, M, I>,
                                &mut self.local.renderer,
                            ))
                        } else {
                            self.remotes
                                .get_mut(user_id.team())
                                .map(|p| (&mut p.player as _, &mut p.renderer))
                        };

                        match user {
                            Some((user, user_ui)) => match action {
                                BattleClientGuiCurrent::Move(targets) => {
                                    // log::trace!("update current: client move");

                                    match self.gui.text.finished() {
                                        false => self.gui.text.update(ctx, delta),
                                        true =>
                                        /*if self.gui.text.page() > 0 || self.gui.text.waiting() */
                                        {
                                            //&& user_ui[instance.pokemon.index].renderer.moves.finished() {

                                            // run through hp update and flicker

                                            let mut not_done = false;

                                            for Indexed(location, ..) in targets {
                                                if let Some(target_ui) =
                                                    if location.team() == self.local.player.id() {
                                                        Some(&mut self.local.renderer)
                                                    } else {
                                                        self.remotes
                                                            .get_mut(location.team())
                                                            .map(|p| &mut p.renderer)
                                                    }
                                                {
                                                    let ui = &mut target_ui[location.index()];

                                                    // while ui.pokemon.flicker.flickering() || ui.status.health_moving() {
                                                    ui.pokemon.flicker.update(delta);
                                                    ui.status.update_hp(delta);
                                                    // }

                                                    if ui.pokemon.flicker.flickering()
                                                        || ui.status.health_moving()
                                                    {
                                                        not_done = true;
                                                    }
                                                }
                                            }

                                            if !not_done {
                                                queue.current = None;
                                            }
                                        }
                                    }
                                }
                                BattleClientGuiCurrent::Switch(new) => {
                                    match self.gui.text.finished() {
                                        false => {
                                            self.gui.text.update(ctx, delta);

                                            if self.gui.text.page() == 1
                                                && !user.active_eq(user_id.index(), Some(*new))
                                            {
                                                user.replace(user_id.index(), Some(*new));
                                                let renderer = &mut user_ui[user_id.index()];
                                                let id = match user.active_mut(user_id.index()) {
                                                    Some(user) => Some(match user.instance() {
                                                        Some(i) => {
                                                            renderer.status.update_gui(
                                                                Some(i),
                                                                None,
                                                                true,
                                                            );
                                                            i.pokemon.id
                                                        }
                                                        None => {
                                                            renderer.status.update_gui_view(
                                                                Some(user as _),
                                                                None,
                                                                true,
                                                            );
                                                            user.pokemon().id
                                                        }
                                                    }),
                                                    None => None,
                                                };
                                                renderer.pokemon.new_pokemon(dex, id);
                                            }
                                        }
                                        true => queue.current = None,
                                    }
                                }
                                BattleClientGuiCurrent::UseItem(target) => {
                                    if !self.gui.text.finished() {
                                        self.gui.text.update(ctx, delta)
                                    } else if let Some(p_ui) =
                                        match target.team() == self.local.player.id() {
                                            true => Some(&mut self.local.renderer),
                                            false => self
                                                .remotes
                                                .get_mut(target.team())
                                                .map(|p| &mut p.renderer),
                                        }
                                    {
                                        let target = &mut p_ui[target.index()].status;
                                        if target.health_moving() {
                                            target.update_hp(delta);
                                        } else {
                                            queue.current = None;
                                        }
                                    } else {
                                        queue.current = None;
                                    }
                                }
                                BattleClientGuiCurrent::Faint => {
                                    let ui = &mut user_ui[user_id.index()];
                                    if ui.pokemon.faint.fainting() {
                                        ui.pokemon.faint.update(delta);
                                    } else if !self.gui.text.finished() {
                                        self.gui.text.update(ctx, delta);
                                    } else {
                                        drop(user);
                                        match user_id.team() == self.local.player.id()
                                            && self.local.player.any_inactive()
                                        {
                                            true => match self.party.alive() {
                                                true => {
                                                    self.party.input(
                                                        ctx,
                                                        dex,
                                                        self.local.player.pokemon.as_mut_slice(),
                                                    );
                                                    self.party.update(delta);
                                                    if let Some(selected) =
                                                        self.party.take_selected()
                                                    {
                                                        if !self.local.player.pokemon[selected]
                                                            .fainted()
                                                        {
                                                            // user.queue_replace(index, selected);
                                                            self.party.despawn();
                                                            self.client.send(
                                                                ClientMessage::ReplaceFaint(
                                                                    user_id.index(),
                                                                    selected,
                                                                ),
                                                            );
                                                            self.local.player.replace(
                                                                user_id.index(),
                                                                Some(selected),
                                                            );
                                                            let pokemon = self
                                                                .local
                                                                .player
                                                                .active(user_id.index());
                                                            ui.status
                                                                .update_gui(pokemon, None, true);
                                                            ui.pokemon.new_pokemon(
                                                                dex,
                                                                pokemon.map(|p| p.pokemon.id),
                                                            );
                                                            queue.current = None;
                                                        }
                                                    }
                                                }
                                                false => self.party.spawn(
                                                    dex,
                                                    &self.local.player.pokemon,
                                                    Some(false),
                                                    false,
                                                ),
                                            },
                                            false => {
                                                let remote =
                                                    self.remotes.get_mut(user_id.team()).unwrap();
                                                remote.player.replace(user_id.index(), None);
                                                let ui = &mut remote.renderer[user_id.index()];
                                                ui.status.update_gui::<P, M, I>(None, None, true);
                                                ui.pokemon.new_pokemon(dex, None);
                                                queue.current = None;
                                            }
                                        }
                                    }
                                }
                                BattleClientGuiCurrent::Replace(index, replaced) => {
                                    if self.gui.text.waiting()
                                        || self.gui.text.finished() && !*replaced
                                    {
                                        if let Some(ui) = user_ui.get_mut(*index) {
                                            let id = match user.active_mut(user_id.index()) {
                                                Some(v) => Some(match v.instance() {
                                                    Some(i) => {
                                                        ui.status.update_gui(Some(i), None, true);
                                                        i.pokemon.id
                                                    }
                                                    None => {
                                                        ui.status.update_gui_view(
                                                            Some(v as _),
                                                            None,
                                                            true,
                                                        );
                                                        v.pokemon().id
                                                    }
                                                }),
                                                None => None,
                                            };
                                            ui.pokemon.new_pokemon(dex, id);
                                            *replaced = true;
                                        }
                                    }
                                    match self.gui.text.finished() {
                                        false => self.gui.text.update(ctx, delta),
                                        true => queue.current = None,
                                    }
                                }
                                BattleClientGuiCurrent::Catch => match self.gui.text.finished() {
                                    false => self.gui.text.update(ctx, delta),
                                    true => queue.current = None,
                                },
                                BattleClientGuiCurrent::SetExp => {
                                    match !self.gui.text.finished()
                                        || self.local.renderer[user_id.index()].status.exp_moving()
                                    {
                                        true => {
                                            self.gui.text.update(ctx, delta);
                                            match self.local.player.active(user_id.index()) {
                                                Some(pokemon) => self.local.renderer
                                                    [user_id.index()]
                                                .status
                                                .update_exp(delta, pokemon),
                                                None => {
                                                    warn!(
                                                        "Could not get pokemon gaining exp at {:?}",
                                                        user_id
                                                    );
                                                    queue.current = None;
                                                }
                                            }
                                        }
                                        false => queue.current = None,
                                    }
                                }
                                BattleClientGuiCurrent::LevelUp => {
                                    match self.gui.level_up.alive() {
                                        true => {
                                            match self.local.player.pokemon.get_mut(user_id.index())
                                            {
                                                Some(pokemon) => {
                                                    if let Some((index, move_ref)) =
                                                        self.gui.level_up.update(
                                                            ctx,
                                                            &mut self.gui.text,
                                                            delta,
                                                            pokemon,
                                                        )
                                                    {
                                                        self.client.send(ClientMessage::LearnMove(
                                                            user_id.index(),
                                                            move_ref.id,
                                                            Some(index),
                                                        ));
                                                    }
                                                }
                                                None => {
                                                    warn!("Could not get user's active pokemon at {:?}", user_id);
                                                    queue.current = None;
                                                }
                                            }
                                        }
                                        false => queue.current = None,
                                    }
                                }
                            },
                            None => queue.current = None,
                        }
                    }
                }
            }
            BattlePlayerState::PlayerEnd | BattlePlayerState::GameEnd(..) => (),
        }
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        dex: &PokedexClientData,
        party: &Party<OwnedPokemon<P, M, I>>,
        bag: &OwnedBag<I>,
    ) {
        if !matches!(self.state, BattlePlayerState::WaitToStart) {
            self.gui.background.draw(ctx, 0.0);
            self.remotes
                .values()
                .for_each(|remote| remote.renderer.iter().for_each(|active| active.draw(ctx)));
            match &self.state {
                BattlePlayerState::WaitToStart => unreachable!(),
                BattlePlayerState::Opening(..) => {
                    self.gui
                        .background
                        .draw(ctx, self.gui.opener.offset::<ID, P>());
                    self.gui.opener.draw_below_panel::<ID, P>(
                        ctx,
                        &self.local.renderer,
                        &self.remotes.values().next().unwrap().renderer,
                    );
                    self.gui.trainer.draw(ctx);
                    self.gui.draw_panel(ctx);
                    self.gui.opener.draw::<ID, P>(ctx);
                }
                BattlePlayerState::Introduction(..) => {
                    self.gui.background.draw(ctx, 0.0);
                    self.gui.introduction.draw::<ID, P, M, I>(
                        ctx,
                        &self.local.renderer,
                        &self.remotes.values().next().unwrap().renderer,
                    );
                    self.gui.trainer.draw(ctx);
                    self.gui.draw_panel(ctx);
                    self.gui.text.draw(ctx);
                }
                BattlePlayerState::Select(index, ..) => {
                    if self.party.alive() {
                        self.party.draw(ctx, &party);
                    } else if self.bag.alive() {
                        self.bag.draw(ctx, dex, &bag.items);
                    } else {
                        for (current, active) in self.local.renderer.iter().enumerate() {
                            if &current == index {
                                active.pokemon.draw(
                                    ctx,
                                    Vec2::new(0.0, self.gui.bounce.offset),
                                    Color::WHITE,
                                );
                                active.status.draw(ctx, 0.0, -self.gui.bounce.offset);
                            } else {
                                active.pokemon.draw(ctx, vec2(0.0, 0.0), Color::WHITE);
                                active.status.draw(ctx, 0.0, 0.0);
                            }
                        }
                        self.gui.draw_panel(ctx);
                        self.gui.panel.draw(ctx);
                    }
                }
                // BattlePlayerState::Faint(..) => if self.party.alive() {
                //     self.party.draw(ctx)
                // },
                BattlePlayerState::WaitToSelect | BattlePlayerState::Moving(..) => {
                    self.local
                        .renderer
                        .iter()
                        .for_each(|active| active.draw(ctx));
                    self.gui.draw_panel(ctx);
                    self.gui.text.draw(ctx);
                    self.gui.level_up.draw(ctx);
                    if self.party.alive() {
                        self.party.draw(ctx, party)
                    }
                }
                BattlePlayerState::GameEnd(..) | BattlePlayerState::PlayerEnd => {
                    self.local
                        .renderer
                        .iter()
                        .for_each(|active| active.draw(ctx));
                    self.gui.draw_panel(ctx);
                    self.gui.text.draw(ctx);
                }
            }
        }
    }
}
