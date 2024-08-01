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
    fn damage_type(&self) -> DamageType;
}

pub struct Pea {
    pub x: f32,
}
impl Entity for Pea {
    fn texture(&self) -> &'static sdl2::render::Texture<'static> {
        textures::pea()
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

    fn damage_type(&self) -> DamageType {
        DamageType::Normal
    }
    
    fn to_remove(&self) -> bool {
        self.x > 1280. + self.width() as f32
    }
}

pub struct FirePea {
    pub x: f32,
}
impl Entity for FirePea {
    fn texture(&self) -> &'static sdl2::render::Texture<'static> {
        textures::fire_pea()
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
impl Projectile for FirePea {
    fn x(&self) -> i32 {
        self.x.floor() as i32
    }

    fn damage_type(&self) -> DamageType {
        DamageType::Fire
    }
    
    fn to_remove(&self) -> bool {
        self.x > 1280. + self.width() as f32
    }
}

pub struct IcePea {
    pub x: f32,
}
impl Entity for IcePea {
    fn texture(&self) -> &'static sdl2::render::Texture<'static> {
        textures::ice_pea()
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
impl Projectile for IcePea {
    fn x(&self) -> i32 {
        self.x.floor() as i32
    }

    fn damage_type(&self) -> DamageType {
        DamageType::Ice
    }
    
    fn to_remove(&self) -> bool {
        self.x > 1280. + self.width() as f32
    }
}
