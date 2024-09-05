use std::time::Duration;

use sdl2::{rect::FRect, render::Texture};

use crate::{projectile::DamageType, textures};

pub fn zombie_from_id(id: u8) -> Box<dyn Zombie> {
    match id {
        0 => Box::new(ZombieBase {
            x: 1.,
            health: ZombieBaseHealth::Normal.into(),
            freeze: Duration::new(0, 0),
        }),
        1 => Box::new(ZombieBase {
            x: 1.,
            health: ZombieBaseHealth::Cone.into(),
            freeze: Duration::new(0, 0),
        }),
        _ => panic!("zombie id not found"),
    }
}

pub fn valide_zombie_id(id: u8) -> bool {
    (0..=1).contains(&id)
}

pub trait Zombie {
    fn texture(&self) -> Result<&'static Texture<'static>, String>;
    fn rect(&self, y: f32) -> FRect;
    fn update(&mut self, elapsed: Duration) -> Result<(), String>;

    fn set_x(&mut self, x: f32);
    fn hit(
        &mut self,
        damage_amount: usize,
        damage_type: DamageType,
        propagated: bool,
    ) -> (bool, bool);
    fn hit_box(&self, y: f32) -> FRect;
    fn freezed(&self) -> bool;
}

#[derive(PartialEq)]
enum ZombieBaseHealth {
    MissingHead,
    Normal,
    HalfCone,
    Cone,
}

impl From<usize> for ZombieBaseHealth {
    fn from(value: usize) -> Self {
        match value {
            0..=100 => Self::MissingHead,
            101..=200 => Self::Normal,
            201..=420 => Self::HalfCone,
            421..=640 => Self::Cone,
            _ => panic!("zombie health out of range"),
        }
    }
}

impl From<ZombieBaseHealth> for usize {
    fn from(value: ZombieBaseHealth) -> Self {
        match value {
            ZombieBaseHealth::MissingHead => 100,
            ZombieBaseHealth::Normal => 200,
            ZombieBaseHealth::HalfCone => 420,
            ZombieBaseHealth::Cone => 640,
        }
    }
}

pub struct ZombieBase {
    x: f32,
    health: usize,
    freeze: Duration,
}

impl Zombie for ZombieBase {
    fn texture(&self) -> Result<&'static Texture<'static>, String> {
        let textures = textures::textures()?;
        Ok(if self.freeze.is_zero() {
            match self.health.into() {
                ZombieBaseHealth::MissingHead => &textures.zombie_simple_1,
                ZombieBaseHealth::Normal => &textures.zombie_simple,
                ZombieBaseHealth::HalfCone => &textures.zombie_cone_1,
                ZombieBaseHealth::Cone => &textures.zombie_cone,
            }
        } else {
            match self.health.into() {
                ZombieBaseHealth::MissingHead => &textures.zombie_freeze_simple_1,
                ZombieBaseHealth::Normal => &textures.zombie_freeze_simple,
                ZombieBaseHealth::HalfCone => &textures.zombie_freeze_cone_1,
                ZombieBaseHealth::Cone => &textures.zombie_freeze_cone,
            }
        })
    }

    fn rect(&self, y: f32) -> FRect {
        FRect::new(
            self.x,
            y,
            55. / 1280.,
            match self.health.into() {
                ZombieBaseHealth::MissingHead | ZombieBaseHealth::Normal => 137.,
                ZombieBaseHealth::HalfCone | ZombieBaseHealth::Cone => 171.,
            } / 720.,
        )
    }

    fn update(&mut self, elapsed: Duration) -> Result<(), String> {
        self.x -= elapsed.as_secs_f32() * 17.321472 / 1280.;
        if !self.freeze.is_zero() {
            if self.freeze > elapsed {
                self.freeze -= elapsed
            } else {
                self.freeze = Duration::ZERO;
            }
            self.x += elapsed.as_secs_f32() * 17.321472 * 0.5 / 1280.;
        }
        Ok(())
    }

    fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    fn hit_box(&self, y: f32) -> FRect {
        FRect::new(
            self.x + 16. / 1280.,
            y,
            39. / 1280.,
            match self.health.into() {
                ZombieBaseHealth::MissingHead | ZombieBaseHealth::Normal => 137.,
                ZombieBaseHealth::HalfCone | ZombieBaseHealth::Cone => 171.,
            } / 720.,
        )
    }

    fn hit(
        &mut self,
        damage_amount: usize,
        damage_type: DamageType,
        propagated: bool,
    ) -> (bool, bool) {
        let mut propagate = false;
        match damage_type {
            DamageType::Normal => {}
            DamageType::Fire => {
                self.freeze = Duration::ZERO;
                propagate = !propagated;
            }
            DamageType::Ice => {
                self.freeze = Duration::new(10, 0);
                propagate = !propagated;
                if !propagate {
                    return (false, propagate);
                }
            }
        }
        if self.health <= damage_amount {
            return (true, propagate);
        }
        self.health -= damage_amount;
        (false, propagate)
    }

    fn freezed(&self) -> bool {
        !self.freeze.is_zero()
    }
}
