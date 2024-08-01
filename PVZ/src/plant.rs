use std::time::Duration;

use sdl2::render::Texture;

use crate::{
    entity::Entity,
    projectile::{FirePea, IcePea, Pea, Projectile},
    textures,
};

pub trait Plant: Entity {
    fn clone(&self) -> Box<dyn Plant>;
    fn cost(&self) -> u32;
    fn should_spawn(&mut self, x: i32, y: usize, max_y: usize)
        -> Vec<(usize, Box<dyn Projectile>)>;
}

#[derive(Default, Clone)]
pub struct PlantSimple {
    charge: Duration,
}
impl Entity for PlantSimple {
    fn texture(&self) -> &'static Texture<'static> {
        textures::plant_simple()
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
impl Plant for PlantSimple {
    fn cost(&self) -> u32 {
        10
    }

    fn clone(&self) -> Box<dyn Plant> {
        Box::new(Clone::clone(self))
    }

    fn should_spawn(&mut self, x: i32, y: usize, _: usize) -> Vec<(usize, Box<dyn Projectile>)> {
        if self.charge >= Duration::from_millis(5000) {
            self.charge -= Duration::from_millis(5000);
            return vec![(y, Box::new(Pea { x: x as f32 - 25. }))];
        }
        Vec::new()
    }
}

#[derive(Default, Clone)]
pub struct PlantFireSimple {
    charge: Duration,
}
impl Entity for PlantFireSimple {
    fn texture(&self) -> &'static Texture<'static> {
        textures::plant_fire_simple()
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
impl Plant for PlantFireSimple {
    fn cost(&self) -> u32 {
        30
    }

    fn clone(&self) -> Box<dyn Plant> {
        Box::new(Clone::clone(self))
    }

    fn should_spawn(&mut self, x: i32, y: usize, _: usize) -> Vec<(usize, Box<dyn Projectile>)> {
        if self.charge >= Duration::from_millis(5000) {
            self.charge -= Duration::from_millis(5000);
            return vec![(y, Box::new(FirePea { x: x as f32 - 25. }))];
        }
        Vec::new()
    }
}

#[derive(Default, Clone)]
pub struct PlantIceSimple {
    charge: Duration,
}
impl Entity for PlantIceSimple {
    fn texture(&self) -> &'static Texture<'static> {
        textures::plant_ice_simple()
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
impl Plant for PlantIceSimple {
    fn cost(&self) -> u32 {
        20
    }

    fn clone(&self) -> Box<dyn Plant> {
        Box::new(Clone::clone(self))
    }

    fn should_spawn(&mut self, x: i32, y: usize, _: usize) -> Vec<(usize, Box<dyn Projectile>)> {
        if self.charge >= Duration::from_millis(5000) {
            self.charge -= Duration::from_millis(5000);
            return vec![(y, Box::new(IcePea { x: x as f32 - 25. }))];
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
        textures::plant_triple()
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
                    (y, Box::new(Pea { x: x as f32 - 25. })),
                    (y + 1, Box::new(Pea { x: x as f32 - 25. })),
                ];
            } else if y == max_y {
                return vec![
                    (y - 1, Box::new(Pea { x: x as f32 - 25. })),
                    (y, Box::new(Pea { x: x as f32 - 25. })),
                ];
            } else {
                return vec![
                    (y - 1, Box::new(Pea { x: x as f32 - 25. })),
                    (y, Box::new(Pea { x: x as f32 - 25. })),
                    (y + 1, Box::new(Pea { x: x as f32 - 25. })),
                ];
            }
        }
        Vec::new()
    }
}
