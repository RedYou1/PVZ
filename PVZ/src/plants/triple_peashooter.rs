use std::time::Duration;

use anyhow::Result;
use red_sdl::refs::Ref;
use sdl2::{rect::FRect, render::Texture};

use crate::{
    projectile::{DamageType, Pea, Projectile},
    sun::Sun,
    zombie::Zombie,
    State,
};

use super::Plant;

#[derive(Clone)]
pub struct PlantTriple {
    charge: Duration,
    health: Duration,
}
impl PlantTriple {
    pub const fn new() -> Self {
        Self {
            charge: Duration::ZERO,
            health: Duration::new(3, 0),
        }
    }
}
impl Plant for PlantTriple {
    fn texture(&self, state: Ref<State>) -> &'static Texture {
        state.as_ref().textures().plant_triple()
    }

    fn rect(&self, x: f32, y: f32) -> FRect {
        FRect::new(x, y, 70. / 1280., 100. / 720.)
    }

    fn update(&mut self, elapsed: Duration) -> Result<()> {
        self.charge += elapsed;
        Ok(())
    }

    fn cost(&self) -> u32 {
        325
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

    fn should_spawn(
        &mut self,
        x: f32,
        _: f32,
        y: usize,
        max_y: usize,
        zombies: &[Vec<Box<dyn Zombie>>],
    ) -> (Vec<Sun>, Vec<(usize, Box<dyn Projectile>)>) {
        if (y == 0 || zombies[y - 1].is_empty())
            && zombies[y].is_empty()
            && (y == max_y || zombies[y + 1].is_empty())
        {
            self.charge = self
                .charge
                .clamp(Duration::ZERO, Duration::from_secs_f32(1.5))
        } else if self.charge >= Duration::from_millis(1500) {
            self.charge -= Duration::from_millis(1500);
            return (
                Vec::new(),
                if y == 0 {
                    vec![new_pea(x, y), new_pea(x, y + 1)]
                } else if y == max_y {
                    vec![new_pea(x, y - 1), new_pea(x, y)]
                } else {
                    vec![new_pea(x, y - 1), new_pea(x, y), new_pea(x, y + 1)]
                },
            );
        }
        (Vec::new(), Vec::new())
    }

    fn health(&mut self) -> &mut Duration {
        &mut self.health
    }
}

fn new_pea(x: f32, y: usize) -> (usize, Box<dyn Projectile>) {
    (
        y,
        Box::new(Pea {
            x: x - 25. / 1280.,
            damage_type: DamageType::Normal,
        }),
    )
}
