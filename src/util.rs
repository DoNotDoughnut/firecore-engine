pub use crate::{HEIGHT, WIDTH};

pub trait Entity {
    fn spawn(&mut self);

    fn despawn(&mut self);

    fn alive(&self) -> bool;
}

pub trait Reset {
    fn reset(&mut self);
}

pub trait Completable: Reset {
    fn finished(&self) -> bool;
}

pub fn date() -> u64 {
    use std::time::SystemTime;
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|dur| dur.as_secs())
        .unwrap_or_default()
        % 1000
}
