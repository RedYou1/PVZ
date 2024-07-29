use sdl2::render::Texture;

use crate::{entity::Entity, textures};

pub trait Plant: Entity {
    fn clone(&self) -> Box<dyn Plant>;
    fn cost(&self) -> usize;
}

#[derive(Clone)]
pub struct Plant1 {}
impl Entity for Plant1 {
    fn texture(&self) -> &'static Texture<'static> {
        textures::p1()
    }

    fn width(&self) -> u16 {
        70
    }
    fn height(&self) -> u16 {
        100
    }

    fn update(&mut self, _: bool) -> Result<(), String> {
        Ok(())
    }
}
impl Plant for Plant1 {
    fn cost(&self) -> usize {
        1
    }

    fn clone(&self) -> Box<dyn Plant> {
        Box::new(Clone::clone(self))
    }
}
