use std::time::Duration;

use anyhow::Result;
use red_sdl::refs::Ref;
use sdl2::{rect::FRect, render::Texture};

use crate::State;

#[derive(Clone, Copy)]
pub enum DamageType {
    Normal,
    Fire,
    Ice,
}

pub trait Projectile {
    fn texture(&self, state: Ref<State>) -> &'static Texture<'static>;
    fn rect(&self, y: f32) -> FRect;
    fn update(&mut self, elapsed: Duration) -> Result<()>;

    fn to_remove(&self) -> bool;
    fn damage_amount(&self) -> usize;
    fn damage_type(&self) -> DamageType;
}

pub struct Pea {
    pub x: f32,
    pub damage_type: DamageType,
}
impl Projectile for Pea {
    fn texture(&self, state: Ref<State>) -> &'static Texture<'static> {
        let texture = state.as_ref().textures();
        match self.damage_type {
            DamageType::Normal => texture.pea(),
            DamageType::Fire => texture.fire_pea(),
            DamageType::Ice => texture.ice_pea(),
        }
    }

    fn rect(&self, y: f32) -> FRect {
        FRect::new(self.x, y, 50. / 1280., 50. / 720.)
    }

    fn update(&mut self, elapsed: Duration) -> Result<()> {
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
