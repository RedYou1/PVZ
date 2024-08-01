use std::time::Duration;

use sdl2::render::Texture;

use crate::{entity::Entity, projectile::DamageType, textures};

pub fn zombie_from_id(id: u8) -> Box<dyn Zombie> {
    match id {
        0 => Box::new(ZombieBase {
            pos: 0.,
            health: ZombieBaseHealth::Normal,
            freeze: Duration::new(0, 0),
        }),
        1 => Box::new(ZombieBase {
            pos: 0.,
            health: ZombieBaseHealth::Cone,
            freeze: Duration::new(0, 0),
        }),
        _ => panic!("zombie id not found"),
    }
}

pub trait Zombie: Entity {
    fn set_pos(&mut self, x: f32);
    fn pos(&self) -> f32;
    fn hit(&mut self, damage_type: DamageType, propagated: bool) -> (bool, bool);
    fn hit_box(&self) -> (u16, u16);
}

#[derive(PartialEq)]
enum ZombieBaseHealth {
    MissingHead,
    Normal,
    HalfCone,
    Cone,
}

pub struct ZombieBase {
    pos: f32,
    health: ZombieBaseHealth,
    freeze: Duration,
}

impl Entity for ZombieBase {
    fn texture(&self) -> &'static Texture<'static> {
        if self.freeze.is_zero() {
            match self.health {
                ZombieBaseHealth::MissingHead => textures::zombie_simple_1(),
                ZombieBaseHealth::Normal => textures::zombie_simple(),
                ZombieBaseHealth::HalfCone => textures::zombie_cone_1(),
                ZombieBaseHealth::Cone => textures::zombie_cone(),
            }
        } else {
            match self.health {
                ZombieBaseHealth::MissingHead => textures::zombie_freeze_simple_1(),
                ZombieBaseHealth::Normal => textures::zombie_freeze_simple(),
                ZombieBaseHealth::HalfCone => textures::zombie_freeze_cone_1(),
                ZombieBaseHealth::Cone => textures::zombie_freeze_cone(),
            }
        }
    }

    fn width(&self) -> u16 {
        55
    }
    fn height(&self) -> u16 {
        match self.health {
            ZombieBaseHealth::MissingHead | ZombieBaseHealth::Normal => 137,
            ZombieBaseHealth::HalfCone | ZombieBaseHealth::Cone => 171,
        }
    }
    fn update(&mut self, playing: bool, elapsed: Duration) -> Result<(), String> {
        if playing {
            self.pos += elapsed.as_secs_f32() * 0.015;
            if !self.freeze.is_zero() {
                if self.freeze > elapsed {
                    self.freeze -= elapsed
                } else {
                    self.freeze = Duration::ZERO;
                }
                self.pos -= elapsed.as_secs_f32() * 0.015 * 0.75;
            }
        }
        Ok(())
    }
}
impl Zombie for ZombieBase {
    fn set_pos(&mut self, x: f32) {
        self.pos = x;
    }

    fn pos(&self) -> f32 {
        self.pos
    }

    fn hit_box(&self) -> (u16, u16) {
        (16, 39)
    }

    fn hit(&mut self, damage_type: DamageType, propagated: bool) -> (bool, bool) {
        let mut propagate = false;
        match damage_type {
            DamageType::Normal => {}
            DamageType::Fire => {
                self.freeze = Duration::ZERO;
                propagate = !propagated;
            }
            DamageType::Ice => {
                self.freeze = Duration::new(2, 0);
                propagate = !propagated;
                if !propagate {
                    return (false, propagate);
                }
            }
        }
        if self.health == ZombieBaseHealth::MissingHead {
            return (true, propagate);
        }
        self.health = match self.health {
            ZombieBaseHealth::MissingHead => {
                panic!("NO")
            }
            ZombieBaseHealth::Normal => ZombieBaseHealth::MissingHead,
            ZombieBaseHealth::HalfCone => ZombieBaseHealth::Normal,
            ZombieBaseHealth::Cone => ZombieBaseHealth::HalfCone,
        };
        (false, propagate)
    }
}
