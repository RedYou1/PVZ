use std::time::Duration;

use sdl2::render::Texture;

use crate::{entity::Entity, projectile::Projectile, sun::Sun, textures, zombie::Zombie};

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
impl Entity for Sunflower {
    fn texture(&self) -> &'static Texture<'static> {
        &textures::textures().plant_sunflower
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
impl Plant for Sunflower {
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
        x: i32,
        y: i32,
        _: usize,
        _: usize,
        _: &[Vec<Box<dyn Zombie>>],
    ) -> (Vec<Sun>, Vec<(usize, Box<dyn Projectile>)>) {
        if self.charge >= Duration::from_millis(24000) {
            self.charge -= Duration::from_millis(24000);
            return (vec![Sun::new(x, y as f32 - 50., y as f32 + 50.)], Vec::new());
        }
        (Vec::new(), Vec::new())
    }
}
