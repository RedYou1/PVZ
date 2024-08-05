use std::time::Duration;

use sdl2::render::Texture;

use crate::{
    entity::Entity,
    projectile::{DamageType, Pea, Projectile},
    sun::Sun,
    textures,
    zombie::Zombie,
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
impl Entity for PeaShooter {
    fn texture(&self) -> &'static Texture<'static> {
        match self.damage_type {
            DamageType::Normal => &textures::textures().plant_simple,
            DamageType::Fire => &textures::textures().plant_fire_simple,
            DamageType::Ice => &textures::textures().plant_ice_simple,
        }
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
impl Plant for PeaShooter {
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
        x: i32,
        _: i32,
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
                        x: x as f32 - 25.,
                        damage_type: self.damage_type,
                    }),
                )],
            );
        }
        (Vec::new(), Vec::new())
    }
}
