use std::time::Duration;

use sdl2::{rect::FRect, render::Texture};

use crate::{projectile::Projectile, sun::Sun, textures, zombie::Zombie};

use super::Plant;

#[derive(Clone)]
pub struct Sunflower {
    charge: Duration,
    health: Duration,
}
impl Sunflower {
    pub const fn new() -> Self {
        Self {
            charge: Duration::new(19, 0),
            health: Duration::new(3, 0),
        }
    }
}
impl Plant for Sunflower {
    fn texture(&self) -> Result<&'static Texture<'static>, String> {
        Ok(&textures::textures()?.plant_sunflower)
    }

    fn rect(&self, x: f32, y: f32) -> FRect {
        FRect::new(x, y, 70. / 1280., 100. / 720.)
    }

    fn update(&mut self, elapsed: Duration) -> Result<(), String> {
        self.charge += elapsed;
        Ok(())
    }

    fn cost(&self) -> u32 {
        50
    }

    fn clone(&self) -> Box<dyn Plant> {
        Box::new(Clone::clone(self))
    }

    fn can_go_in_water(&self) -> bool {
        false
    }

    fn is_nenuphar(&self) -> bool {
        false
    }

    fn health(&mut self) -> &mut Duration {
        &mut self.health
    }

    fn should_spawn(
        &mut self,
        x: f32,
        y: f32,
        _: usize,
        _: usize,
        _: &[Vec<Box<dyn Zombie>>],
    ) -> (Vec<Sun>, Vec<(usize, Box<dyn Projectile>)>) {
        if self.charge >= Duration::from_millis(24000) {
            self.charge -= Duration::from_millis(24000);
            return (
                vec![Sun::new(x, y - 50. / 720., y + 50. / 720.)],
                Vec::new(),
            );
        }
        (Vec::new(), Vec::new())
    }
}
