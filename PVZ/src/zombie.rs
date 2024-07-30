use std::time::Duration;

use sdl2::render::Texture;

use crate::{entity::Entity, textures};

pub fn zombie_from_id(id: u8) -> Box<dyn Zombie> {
    match id {
        0 => Box::new(Zombie1 {
            pos: 0.,
            health: false,
        }),
        _ => panic!("zombie id not found"),
    }
}

pub trait Zombie: Entity {
    fn pos(&self) -> f32;
    fn hit(&mut self) -> bool;
}

pub struct Zombie1 {
    pub pos: f32,
    pub health: bool,
}

impl Entity for Zombie1 {
    fn texture(&self) -> &'static Texture<'static> {
        if self.health {
            textures::z1_1()
        } else {
            textures::z1()
        }
    }

    fn width(&self) -> u16 {
        90
    }
    fn height(&self) -> u16 {
        159
    }
    fn update(&mut self, playing: bool, elapsed: Duration) -> Result<(), String> {
        if playing {
            self.pos += elapsed.as_secs_f32() * 0.015;
        }
        Ok(())
    }
}
impl Zombie for Zombie1 {
    fn pos(&self) -> f32 {
        self.pos
    }

    fn hit(&mut self) -> bool {
        if self.health {
            return true;
        }
        self.health = true;
        false
    }
}
