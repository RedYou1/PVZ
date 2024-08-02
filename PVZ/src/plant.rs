use std::time::Duration;

use sdl2::render::Texture;

use crate::{
    entity::Entity,
    projectile::{DamageType, Pea, Projectile},
    textures,
};

pub trait Plant: Entity {
    fn clone(&self) -> Box<dyn Plant>;
    fn cost(&self) -> u32;
    fn should_spawn(&mut self, x: i32, y: usize, max_y: usize)
        -> Vec<(usize, Box<dyn Projectile>)>;
}

#[derive(Clone)]
pub struct PeaShooter {
    charge: Duration,
    damage_type: DamageType,
}
impl PeaShooter {
    pub const fn new(damage_type: DamageType) -> Self {
        PeaShooter {
            charge: Duration::ZERO,
            damage_type,
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
            DamageType::Normal => 10,
            DamageType::Fire => 30,
            DamageType::Ice => 20,
        }
    }

    fn clone(&self) -> Box<dyn Plant> {
        Box::new(Clone::clone(self))
    }

    fn should_spawn(&mut self, x: i32, y: usize, _: usize) -> Vec<(usize, Box<dyn Projectile>)> {
        if self.charge >= Duration::from_millis(5000) {
            self.charge -= Duration::from_millis(5000);
            return vec![(
                y,
                Box::new(Pea {
                    x: x as f32 - 25.,
                    damage_type: self.damage_type,
                }),
            )];
        }
        Vec::new()
    }
}

#[derive(Default, Clone)]
pub struct PlantTriple {
    charge: Duration,
}
impl Entity for PlantTriple {
    fn texture(&self) -> &'static Texture<'static> {
        &textures::textures().plant_triple
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
impl Plant for PlantTriple {
    fn cost(&self) -> u32 {
        40
    }

    fn clone(&self) -> Box<dyn Plant> {
        Box::new(Clone::clone(self))
    }

    fn should_spawn(
        &mut self,
        x: i32,
        y: usize,
        max_y: usize,
    ) -> Vec<(usize, Box<dyn Projectile>)> {
        if self.charge >= Duration::from_millis(5000) {
            self.charge -= Duration::from_millis(5000);
            if y == 0 {
                return vec![
                    (
                        y,
                        Box::new(Pea {
                            x: x as f32 - 25.,
                            damage_type: DamageType::Normal,
                        }),
                    ),
                    (
                        y + 1,
                        Box::new(Pea {
                            x: x as f32 - 25.,
                            damage_type: DamageType::Normal,
                        }),
                    ),
                ];
            } else if y == max_y {
                return vec![
                    (
                        y - 1,
                        Box::new(Pea {
                            x: x as f32 - 25.,
                            damage_type: DamageType::Normal,
                        }),
                    ),
                    (
                        y,
                        Box::new(Pea {
                            x: x as f32 - 25.,
                            damage_type: DamageType::Normal,
                        }),
                    ),
                ];
            } else {
                return vec![
                    (
                        y - 1,
                        Box::new(Pea {
                            x: x as f32 - 25.,
                            damage_type: DamageType::Normal,
                        }),
                    ),
                    (
                        y,
                        Box::new(Pea {
                            x: x as f32 - 25.,
                            damage_type: DamageType::Normal,
                        }),
                    ),
                    (
                        y + 1,
                        Box::new(Pea {
                            x: x as f32 - 25.,
                            damage_type: DamageType::Normal,
                        }),
                    ),
                ];
            }
        }
        Vec::new()
    }
}
