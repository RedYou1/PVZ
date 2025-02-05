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
pub struct PeaShooter {
    charge: Duration,
    damage_type: DamageType,
    health: Duration,
}
impl PeaShooter {
    pub const fn new(damage_type: DamageType) -> Self {
        PeaShooter {
            charge: Duration::ZERO,
            damage_type,
            health: Duration::new(3, 0),
        }
    }
}
impl Plant for PeaShooter {
    fn texture(&self, state: Ref<State>) -> &'static Texture {
        let texture = state.as_ref().textures();
        match self.damage_type {
            DamageType::Normal => texture.plant_simple(),
            DamageType::Fire => texture.plant_fire_simple(),
            DamageType::Ice => texture.plant_ice_simple(),
        }
    }

    fn rect(&self, x: f32, y: f32) -> FRect {
        FRect::new(x, y, 70. / 1280., 100. / 720.)
    }

    fn update(&mut self, elapsed: Duration) -> Result<()> {
        self.charge += elapsed;
        Ok(())
    }

    fn cost(&self) -> u32 {
        match self.damage_type {
            DamageType::Normal => 100,
            DamageType::Fire => 175,
            DamageType::Ice => 175,
        }
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
        _: f32,
        y: usize,
        _: usize,
        zombies: &[Vec<Box<dyn Zombie>>],
    ) -> (Vec<Sun>, Vec<(usize, Box<dyn Projectile>)>) {
        if zombies[y].is_empty() {
            self.charge = self
                .charge
                .clamp(Duration::ZERO, Duration::from_secs_f32(1.5))
        } else if self.charge >= Duration::from_millis(1500) {
            self.charge -= Duration::from_millis(1500);
            return (
                Vec::new(),
                vec![(
                    y,
                    Box::new(Pea {
                        x: x - 25. / 1280.,
                        damage_type: self.damage_type,
                    }),
                )],
            );
        }
        (Vec::new(), Vec::new())
    }
}
