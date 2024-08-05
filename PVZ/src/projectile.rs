use std::time::Duration;

use crate::{entity::Entity, textures};

#[derive(Clone, Copy)]
pub enum DamageType {
    Normal,
    Fire,
    Ice,
}

pub trait Projectile: Entity {
    fn x(&self) -> i32;
    fn to_remove(&self) -> bool;
    fn damage_amount(&self) -> usize;
    fn damage_type(&self) -> DamageType;
}

pub struct Pea {
    pub x: f32,
    pub damage_type: DamageType,
}
impl Entity for Pea {
    fn texture(&self) -> &'static sdl2::render::Texture<'static> {
        match self.damage_type {
            DamageType::Normal => &textures::textures().pea,
            DamageType::Fire => &textures::textures().fire_pea,
            DamageType::Ice => &textures::textures().ice_pea,
        }
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

    fn damage_amount(&self) -> usize {
        20
    }

    fn damage_type(&self) -> DamageType {
        self.damage_type
    }

    fn to_remove(&self) -> bool {
        self.x > 1280. + self.width() as f32
    }
}
