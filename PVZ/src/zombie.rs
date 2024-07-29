use sdl2::render::Texture;

use crate::{entity::Entity, textures};

pub trait Zombie: Entity {
    fn clone(&self) -> Box<dyn Zombie>;
    fn pos(&self) -> f32;
}

#[derive(Clone)]
pub struct Zombie1 {
    pub pos: f32,
}

impl Entity for Zombie1 {
    fn texture(&self) -> &'static Texture<'static> {
        textures::z1()
    }

    fn width(&self) -> u16 {
        90
    }
    fn height(&self) -> u16 {
        159
    }
    fn update(&mut self, playing: bool) -> Result<(), String> {
        if playing {
            self.pos += 0.0003;
        }
        Ok(())
    }
}
impl Zombie for Zombie1 {
    fn pos(&self) -> f32 {
        self.pos
    }

    fn clone(&self) -> Box<dyn Zombie> {
        Box::new(Clone::clone(self))
    }
}
