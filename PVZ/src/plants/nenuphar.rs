use std::time::Duration;

use anyhow::Result;
use red_sdl::refs::Ref;
use sdl2::{rect::FRect, render::Texture};

use crate::{projectile::Projectile, sun::Sun, zombie::Zombie, State};

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
impl Plant for Nenuphar {
    fn texture(&self, state: Ref<State>) -> &'static Texture {
        state.as_ref().textures().plant_nenuphar()
    }

    fn rect(&self, x: f32, y: f32) -> FRect {
        FRect::new(x, y, 70. / 1280., 100. / 720.)
    }

    fn update(&mut self, _: Duration) -> Result<()> {
        Ok(())
    }

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
        _: f32,
        _: f32,
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
