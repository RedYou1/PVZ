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
        x: i32,
        _: i32,
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
                    vec![
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
                    ]
                } else if y == max_y {
                    vec![
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
                    ]
                } else {
                    vec![
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
                    ]
                },
            );
        }
        (Vec::new(), Vec::new())
    }

    fn health(&mut self) -> &mut Duration {
        &mut self.health
    }
}
