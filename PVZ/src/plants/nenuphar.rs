use std::time::Duration;

use sdl2::render::Texture;

use crate::{entity::Entity, projectile::Projectile, sun::Sun, textures, zombie::Zombie};

use super::Plant;

#[derive(Clone)]
pub struct Nenuphar {
    health: Duration,
}
impl Nenuphar {
    pub const fn new() -> Self {
        Self {
            health: Duration::new(3, 0),
        }
    }
}
impl Entity for Nenuphar {
    fn texture(&self) -> Result<&'static Texture<'static>, String> {
        Ok(&textures::textures()?.plant_nenuphar)
    }

    fn width(&self) -> u16 {
        70
    }
    fn height(&self) -> u16 {
        100
    }

    fn update(&mut self, _: bool, _: Duration) -> Result<(), String> {
        Ok(())
    }
}
impl Plant for Nenuphar {
    fn cost(&self) -> u32 {
        25
    }

    fn clone(&self) -> Box<dyn Plant> {
        Box::new(Clone::clone(self))
    }

    fn can_go_in_water(&self) -> bool {
        true
    }

    fn is_nenuphar(&self) -> bool {
        true
    }

    fn should_spawn(
        &mut self,
        _: i32,
        _: i32,
        _: usize,
        _: usize,
        _: &[Vec<Box<dyn Zombie>>],
    ) -> (Vec<Sun>, Vec<(usize, Box<dyn Projectile>)>) {
        (Vec::new(), Vec::new())
    }

    fn health(&mut self) -> &mut Duration {
        &mut self.health
    }
}
