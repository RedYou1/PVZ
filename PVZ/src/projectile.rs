use std::time::Duration;

use sdl2::{rect::FRect, render::Texture};

use crate::textures;

#[derive(Clone, Copy)]
pub enum DamageType {
    Normal,
    Fire,
    Ice,
}

pub trait Projectile {
    fn texture(&self) -> Result<&'static Texture<'static>, String>;
    fn rect(&self, y: f32) -> FRect;
    fn update(&mut self, elapsed: Duration) -> Result<(), String>;

    fn to_remove(&self) -> bool;
    fn damage_amount(&self) -> usize;
    fn damage_type(&self) -> DamageType;
}

pub struct Pea {
    pub x: f32,
    pub damage_type: DamageType,
}
impl Projectile for Pea {
    fn texture(&self) -> Result<&'static sdl2::render::Texture<'static>, String> {
        let textures = textures::textures()?;
        Ok(match self.damage_type {
            DamageType::Normal => &textures.pea,
            DamageType::Fire => &textures.fire_pea,
            DamageType::Ice => &textures.ice_pea,
        })
    }

    fn rect(&self, y: f32) -> FRect {
        FRect::new(self.x, y, 50. / 1280., 50. / 720.)
    }

    fn update(&mut self, elapsed: Duration) -> Result<(), String> {
        self.x += elapsed.as_secs_f32() * 200. / 1280.;
        Ok(())
    }

    fn damage_amount(&self) -> usize {
        20
    }

    fn damage_type(&self) -> DamageType {
        self.damage_type
    }

    fn to_remove(&self) -> bool {
        self.x > 1. + self.rect(0.).width()
    }
}
