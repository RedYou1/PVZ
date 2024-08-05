use rand::Rng;
use std::{
    fs,
    io::{self},
    time::Duration,
};

use crate::{shop::Shop, zombie::zombie_from_id};

use super::Level;

#[derive(PartialEq, Eq)]
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

    pub wait: Vec<Duration>,
    #[allow(clippy::type_complexity)]
    pub zombies: Vec<Vec<(u8, i32, i32)>>,
}

impl LevelConfig {
    pub const fn coord_to_pos_x(&self, x: i32) -> Option<usize> {
        if x < self.left as i32 || x >= self.left as i32 + self.width as i32 {
            None
        } else {
            Some((x as usize - self.left as usize) * self.cols as usize / self.width as usize)
        }
    }
    pub fn coord_to_pos_y(&self, y: i32) -> Option<usize> {
        if y < self.top as i32 || y >= self.top as i32 + self.height as i32 {
            None
        } else {
            Some((y as usize - self.top as usize) * self.rows.len() / self.height as usize)
        }
    }

    pub const fn pos_to_coord_x(&self, x: usize) -> i32 {
        x as i32 * self.width as i32 / self.cols as i32 + self.left as i32
    }
    pub fn pos_to_coord_y(&self, y: usize) -> i32 {
        y as i32 * self.height as i32 / self.rows.len() as i32 + self.top as i32
    }

    pub fn row_heigth(&self) -> u32 {
        self.height as u32 / self.rows.len() as u32
    }

    pub const fn col_width(&self) -> u32 {
        self.width as u32 / self.cols as u32
    }

    #[allow(clippy::unwrap_in_result)]
    pub fn load_config(path: &str) -> std::io::Result<Level> {
        let mut data = fs::read(path)?;

        let map = data.remove(0);

        let mut data2 = fs::read(format!("assets/maps/{map}.data"))?;
        let top = u16::from_le_bytes([data2.remove(0), data2.remove(0)]);
        let left = u16::from_le_bytes([data2.remove(0), data2.remove(0)]);
        let width = u16::from_le_bytes([data2.remove(0), data2.remove(0)]);
        let height = u16::from_le_bytes([data2.remove(0), data2.remove(0)]);
        let rows = data2.remove(0);
        let cols = data2.remove(0);
        let rows_types = (0..rows)
            .map(|_| match data2.remove(0) {
                0 => RowType::Grass,
                1 => RowType::Water,
                _ => panic!("Not found row type"),
            })
            .collect();

        let money = u32::from_le_bytes([
            data.remove(0),
            data.remove(0),
            data.remove(0),
            data.remove(0),
        ]);
        let waves = data.remove(0).into();

        let wait = data
            .drain(0..waves)
            .map(|secs| Duration::from_secs(secs as u64))
            .collect();

        let mut rng = rand::thread_rng();
        let z_rng_x1 = left + width - 305;
        let z_rng_y1 = top + height / rows as u16;
        let z_rng_y2 = top + height;
        let zombies = (0..waves)
            .map(|_| {
                let types = data.remove(0).into();

                let zombies = data
                    .chunks_exact(2)
                    .take(types)
                    .flat_map(|bytes| {
                        let z = zombie_from_id(bytes[0]);
                        (0..bytes[1])
                            .map(|_| {
                                (
                                    bytes[0],
                                    rng.gen_range((z_rng_x1 as i32)..(1280 - z.width() as i32)),
                                    rng.gen_range(
                                        (z_rng_y1 as i32 - z.height() as i32)
                                            ..(z_rng_y2 as i32 - z.height() as i32),
                                    ),
                                )
                            })
                            .collect::<Vec<(u8, i32, i32)>>()
                    })
                    .collect();

                data.drain(0..types * 2);

                zombies
            })
            .collect();

        if !data.is_empty() {
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
                wait,
                zombies,
            },
            shop: Shop::new(money),
            end: None,
        })
    }
}
