use std::ops::DerefMut;

use crate::Context;

#[allow(unused_variables)]
#[async_trait::async_trait(?Send)]
pub trait State<C: DerefMut<Target = Context>> {
    
    async fn start(&mut self, ctx: &mut C) {

    }

    fn update(&mut self, ctx: &mut C, delta: f32) {

    }

    fn draw(&mut self, ctx: &mut C) {

    }

    fn end(&mut self, ctx: &mut C) {
        
    }

}