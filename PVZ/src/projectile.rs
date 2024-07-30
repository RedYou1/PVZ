use std::time::Duration;

use crate::{entity::Entity, textures};

pub trait Projectile: Entity {
    fn x(&self) -> i32;
}

pub struct Pea {
    pub x: f32,
}
impl Entity for Pea {
    fn texture(&self) -> &'static sdl2::render::Texture<'static> {
        textures::pea()
    }

    fn width(&self) -> u16 {
        50
    }

    fn height(&self) -> u16 {
        50
    }

    fn update(&mut self, playing: bool, elapsed: Duration) -> Result<(), String> {
        if !playing {
            return Ok(());
        }
        self.x += elapsed.as_secs_f32() * 200.;
        Ok(())
    }
}
impl Projectile for Pea {
    fn x(&self) -> i32 {
        self.x.floor() as i32
    }
}
