use std::time::Duration;

use sdl2::render::Texture;

use crate::{entity::Entity, textures};

pub fn zombie_from_id(id: u8) -> Box<dyn Zombie> {
    match id {
        0 => Box::new(ZombieSimple {
            pos: 0.,
            health: false,
        }),
        _ => panic!("zombie id not found"),
    }
}

pub trait Zombie: Entity {
    fn set_pos(&mut self, x: f32);
    fn pos(&self) -> f32;
    fn hit(&mut self) -> bool;
}

pub struct ZombieSimple {
    pub pos: f32,
    pub health: bool,
}

impl Entity for ZombieSimple {
    fn texture(&self) -> &'static Texture<'static> {
        if self.health {
            textures::zombie_simple_1()
        } else {
            textures::zombie_simple()
        }
    }

    fn width(&self) -> u16 {
        55
    }
    fn height(&self) -> u16 {
        137
    }
    fn update(&mut self, playing: bool, elapsed: Duration) -> Result<(), String> {
        if playing {
            self.pos += elapsed.as_secs_f32() * 0.015;
        }
        Ok(())
    }
}
impl Zombie for ZombieSimple {
    fn set_pos(&mut self, x: f32) {
        self.pos = x;
    }

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
