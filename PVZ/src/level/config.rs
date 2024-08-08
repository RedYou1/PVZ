use rand::Rng;
use std::{
    fs,
    io::{self},
    time::Duration,
};

use crate::{shop::Shop, zombie::zombie_from_id};

use super::Level;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RowType {
    Grass,
    Water,
}
pub struct LevelConfig {
    pub map: u8,

    pub top: u16,
    pub left: u16,
    pub width: u16,
    pub height: u16,

    pub rows: Vec<RowType>,
    pub cols: u8,

    pub waits: Vec<Duration>,
    #[allow(clippy::type_complexity)]
    pub zombies: Vec<Vec<(u8, f32, f32)>>,
}

impl LevelConfig {
    pub const fn coord_to_pos_x(&self, x: f32) -> Option<usize> {
        let x = x as usize;
        if x < self.left as usize || x >= self.left as usize + self.width as usize {
            None
        } else {
            Some((x - self.left as usize) * self.cols as usize / self.width as usize)
        }
    }
    pub fn coord_to_pos_y(&self, y: f32) -> Option<usize> {
        let y = y as usize;
        if y < self.top as usize || y >= self.top as usize + self.height as usize {
            None
        } else {
            Some((y - self.top as usize) * self.rows.len() / self.height as usize)
        }
    }

    pub fn pos_to_coord_x(&self, x: usize) -> f32 {
        x as f32 * self.width as f32 / self.cols as f32 + self.left as f32
    }
    pub fn pos_to_coord_y(&self, y: usize) -> f32 {
        y as f32 * self.height as f32 / self.rows.len() as f32 + self.top as f32
    }

    pub fn row_heigth(&self) -> f32 {
        self.height as f32 / self.rows.len() as f32
    }

    pub fn col_width(&self) -> f32 {
        self.width as f32 / self.cols as f32
    }

    pub fn load_config(level: u8) -> std::io::Result<Level> {
        let mut level_data = fs::read(format!("levels/{level}.data"))?;

        let map = level_data.remove(0);

        let mut map_data = fs::read(format!("assets/maps/{map}.data"))?;
        let top = u16::from_le_bytes([map_data.remove(0), map_data.remove(0)]);
        let left = u16::from_le_bytes([map_data.remove(0), map_data.remove(0)]);
        let width = u16::from_le_bytes([map_data.remove(0), map_data.remove(0)]);
        let height = u16::from_le_bytes([map_data.remove(0), map_data.remove(0)]);
        let rows = map_data.remove(0);
        let cols = map_data.remove(0);
        let rows_types = (0..rows)
            .map(|_| match map_data.remove(0) {
                0 => RowType::Grass,
                1 => RowType::Water,
                _ => panic!("Not found row type"),
            })
            .collect();

        let money = u32::from_le_bytes([
            level_data.remove(0),
            level_data.remove(0),
            level_data.remove(0),
            level_data.remove(0),
        ]);
        let waves = level_data.remove(0).into();

        let waits = level_data
            .drain(0..waves)
            .map(|secs| Duration::from_secs(secs as u64))
            .collect();

        let min_x = left as f32 + width as f32 - 305.;
        let min_y = top as f32 + height as f32 / rows as f32;
        let max_y = top as f32 + height as f32;
        let zombies = (0..waves)
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

        Ok(Level {
            showing_zombies: true,
            suns: Vec::with_capacity(4),
            next_sun: Duration::new(5, 0),
            plants: (0..rows)
                .map(|_| (0..cols).map(|_| None).collect())
                .collect(),
            zombies: (0..rows).map(|_| Vec::with_capacity(16)).collect(),
            projectiles: (0..rows).map(|_| Vec::with_capacity(4)).collect(),
            config: LevelConfig {
                map,
                top,
                left,
                width,
                height,
                rows: rows_types,
                cols,
                waits,
                zombies,
            },
            shop: Shop::new(money),
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
                        rng.gen_range((min_x)..(1280. - width)),
                        rng.gen_range((min_y - height)..(max_y - height)),
                    )
                })
                .collect::<Vec<(u8, f32, f32)>>()
        })
        .collect()
}
