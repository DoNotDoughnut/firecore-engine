use std::ops::DerefMut;

use crate::Context;

#[allow(unused_variables)]
pub trait State<C: DerefMut<Target = Context>> {
    
    fn start(&mut self, ctx: &mut C) {

    }

    fn update(&mut self, ctx: &mut C, delta: f32) {

    }

    fn draw(&mut self, ctx: &mut C) {

    }

    fn end(&mut self, ctx: &mut C) {
        
    }

}