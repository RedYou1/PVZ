use std::time::Duration;

use sdl2::render::Texture;

use crate::{
    entity::Entity,
    projectile::{Pea, Projectile},
    textures,
};

pub trait Plant: Entity {
    fn clone(&self) -> Box<dyn Plant>;
    fn cost(&self) -> usize;
    fn should_spawn(&mut self, x: i32, y: usize) -> Vec<(usize, Box<dyn Projectile>)>;
}

#[derive(Default, Clone)]
pub struct Plant1 {
    charge: Duration,
}
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

    fn update(&mut self, playing: bool, elapsed: Duration) -> Result<(), String> {
        if playing {
            self.charge += elapsed;
        }
        Ok(())
    }
}
impl Plant for Plant1 {
    fn cost(&self) -> usize {
        10
    }

    fn clone(&self) -> Box<dyn Plant> {
        Box::new(Clone::clone(self))
    }

    fn should_spawn(&mut self, x: i32, y: usize) -> Vec<(usize, Box<dyn Projectile>)> {
        if self.charge >= Duration::from_millis(5000) {
            self.charge -= Duration::from_millis(5000);
            return vec![(y, Box::new(Pea { x: x as f32 - 25. }))];
        }
        Vec::new()
    }
}
