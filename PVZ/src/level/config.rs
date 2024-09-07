use rand::Rng;
use sdl::grid::Grid;
use sdl2::rect::FRect;
use std::{
    collections::HashMap,
    fs,
    io::{self},
    time::Duration,
};

use crate::{
    plants::{
        nenuphar::Nenuphar, peashooter::PeaShooter, sunflower::Sunflower,
        triple_peashooter::PlantTriple,
    },
    projectile::DamageType,
    zombie::zombie_from_id,
};

use super::Level;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RowType {
    Grass,
    Water,
}
pub struct Map {
    pub id: u8,

    pub top: f32,
    pub left: f32,
    pub width: f32,
    pub height: f32,

    pub rows: Vec<RowType>,
    pub cols: u8,
}

impl Map {
    pub fn coord_to_pos_x(&self, x: f32) -> Option<usize> {
        if x < self.left || x >= self.left + self.width {
            None
        } else {
            Some(((x - self.left) * self.cols as f32 / self.width) as usize)
        }
    }
    pub fn coord_to_pos_y(&self, y: f32) -> Option<usize> {
        if y < self.top || y >= self.top + self.height {
            None
        } else {
            Some(((y - self.top) * self.rows.len() as f32 / self.height) as usize)
        }
    }

    pub fn pos_to_coord_x(&self, x: usize) -> f32 {
        x as f32 * self.width / self.cols as f32 + self.left
    }
    pub fn pos_to_coord_y(&self, y: usize) -> f32 {
        y as f32 * self.height / self.rows.len() as f32 + self.top
    }

    pub fn row_heigth(&self) -> f32 {
        self.height / self.rows.len() as f32
    }

    pub fn col_width(&self) -> f32 {
        self.width / self.cols as f32
    }

    pub fn load(map: u8) -> std::io::Result<Self> {
        let mut map_data = fs::read(format!("assets/maps/{map}.data"))?;
        let top = f32::from_le_bytes([
            map_data.remove(0),
            map_data.remove(0),
            map_data.remove(0),
            map_data.remove(0),
        ]);
        let left = f32::from_le_bytes([
            map_data.remove(0),
            map_data.remove(0),
            map_data.remove(0),
            map_data.remove(0),
        ]);
        let width = f32::from_le_bytes([
            map_data.remove(0),
            map_data.remove(0),
            map_data.remove(0),
            map_data.remove(0),
        ]);
        let height = f32::from_le_bytes([
            map_data.remove(0),
            map_data.remove(0),
            map_data.remove(0),
            map_data.remove(0),
        ]);
        let rows = map_data.remove(0);
        let cols = map_data.remove(0);
        let rows_types = (0..rows)
            .map(|_| match map_data.remove(0) {
                0 => RowType::Grass,
                1 => RowType::Water,
                _ => panic!("Not found row type"),
            })
            .collect();

        Ok(Self {
            id: map,
            top,
            left,
            width,
            height,
            rows: rows_types,
            cols,
        })
    }

    pub fn save(&self) -> std::io::Result<()> {
        let mut map_data = Vec::with_capacity(32);
        map_data.extend(self.top.to_le_bytes());
        map_data.extend(self.left.to_le_bytes());
        map_data.extend(self.width.to_le_bytes());
        map_data.extend(self.height.to_le_bytes());
        map_data.push(self.rows.len() as u8);
        map_data.push(self.cols);
        map_data.extend(self.rows.iter().map(|row| match row {
            RowType::Grass => 0,
            RowType::Water => 1,
        }));
        fs::write(format!("assets/maps/{}.data", self.id), map_data)
    }
}
impl Level {
    pub fn save_config(&self) -> std::io::Result<()> {
        let mut level_data = Vec::with_capacity(32);
        level_data.push(self.map.id);
        level_data.extend(self.money.to_le_bytes());
        level_data.push(self.spawn_waits.len() as u8);
        level_data.extend(self.spawn_waits.iter().map(|w| w.as_secs() as u8));
        level_data.extend(self.spawn_zombies.iter().flat_map(|z| {
            let mut zombies: HashMap<u8, u8> = HashMap::with_capacity(4);
            for z_id in z.iter().map(|z| z.0) {
                if let Some(zz) = zombies.get_mut(&z_id) {
                    *zz += 1;
                } else {
                    zombies.insert(z_id, 1);
                }
            }
            let mut z_data = Vec::with_capacity(8);
            z_data.push(zombies.len() as u8);
            z_data.extend(zombies.into_iter().flat_map(|(k, v)| [k, v]));
            z_data
        }));
        fs::write(format!("levels/{}.data", self.id), level_data)
    }
    pub fn load(level: u8) -> std::io::Result<Self> {
        let mut level_data = fs::read(format!("levels/{level}.data"))?;

        let map = Map::load(level_data.remove(0))?;
        let rows = map.rows.len();

        let money = u32::from_le_bytes([
            level_data.remove(0),
            level_data.remove(0),
            level_data.remove(0),
            level_data.remove(0),
        ]);
        let waves = level_data.remove(0).into();

        let spawn_waits = level_data
            .drain(0..waves)
            .map(|secs| Duration::from_secs(secs as u64))
            .collect();

        let min_x = map.left + map.width - 305. / 1280.;
        let min_y = map.top + map.height / rows as f32;
        let max_y = map.top + map.height;
        let spawn_zombies = (0..waves)
            .map(|_| {
                let types = level_data.remove(0).into();
                let zombies = generate_zombies_wave(&level_data, types, min_x, min_y, max_y);
                level_data.drain(0..types * 2);
                zombies
            })
            .collect();

        if !level_data.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Wrong format".to_owned(),
            ));
        }

        Ok(Self {
            id: level,
            started: None,
            surface: FRect::new(0., 0., 0., 0.),
            suns: Vec::with_capacity(4),
            next_sun: Duration::new(5, 0),
            plants: (0..rows)
                .map(|_| (0..map.cols).map(|_| None).collect())
                .collect(),
            map_plants: unsafe { Grid::empty() },
            zombies: (0..rows).map(|_| Vec::with_capacity(16)).collect(),
            projectiles: (0..rows).map(|_| Vec::with_capacity(4)).collect(),
            map,
            spawn_waits,
            spawn_zombies,
            shop_plants: vec![
                Box::new(Nenuphar::new()),
                Box::new(Sunflower::new()),
                Box::new(PeaShooter::new(DamageType::Normal)),
                Box::new(PeaShooter::new(DamageType::Ice)),
                Box::new(PeaShooter::new(DamageType::Fire)),
                Box::new(PlantTriple::new()),
            ],
            dragging: None,
            money,
            end: None,
        })
    }
}

fn generate_zombies_wave(
    data: &[u8],
    types: usize,
    min_x: f32,
    min_y: f32,
    max_y: f32,
) -> Vec<(u8, f32, f32)> {
    let mut rng = rand::thread_rng();
    data.chunks_exact(2)
        .take(types)
        .flat_map(|bytes| {
            let (width, height) = zombie_from_id(bytes[0]).rect(0.).size();
            (0..bytes[1])
                .map(|_| {
                    (
                        bytes[0],
                        rng.gen_range((min_x)..(1. - width)),
                        rng.gen_range((min_y - height)..(max_y - height)),
                    )
                })
                .collect::<Vec<(u8, f32, f32)>>()
        })
        .collect()
}
