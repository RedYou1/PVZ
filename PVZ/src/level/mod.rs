use config::Map;
use sdl::{
    event::Event,
    functions::StateEnum,
    grid::{ColType, Grid, GridChildren, Pos, RowType},
    missing::{
        rect::scale,
        ui_string::{draw_string, UIString},
    },
    ui_rect::UIRect,
    user_control::UserControl,
};
use sdl2::{
    keyboard::Keycode,
    mouse::MouseButton,
    pixels::Color,
    rect::{FPoint, FRect, Rect},
    render::Canvas,
    video::Window,
};
use std::{collections::HashMap, time::Duration};

mod collision;
pub mod config;
mod draws;
mod updates;

use crate::{
    map_plant::MapPlant,
    plants::Plant,
    projectile::Projectile,
    shop_plant::ShopPlant,
    sun::Sun,
    textures::{self, textures},
    win::Win,
    zombie::{zombie_from_id, Zombie},
};

pub struct Level {
    pub started: Option<Grid<Level>>,
    pub surface: FRect,
    pub suns: Vec<Sun>,
    pub next_sun: Duration,
    pub plants: Vec<Vec<Option<Box<dyn Plant>>>>,
    pub map_plants: Grid<Level>,
    pub zombies: Vec<Vec<Box<dyn Zombie>>>,
    pub projectiles: Vec<Vec<Box<dyn Projectile>>>,
    pub map: Map,
    pub spawn_waits: Vec<Duration>,
    #[allow(clippy::type_complexity)]
    pub spawn_zombies: Vec<Vec<(u8, f32, f32)>>,
    pub shop_plants: Vec<Box<dyn Plant>>,
    pub dragging: Option<(f32, f32, Box<dyn Plant>)>,
    pub money: u32,
    pub end: Option<bool>,
}

impl Level {
    fn take_plant(&mut self, plant: &dyn Plant, x: f32, y: f32) {
        if self.dragging.is_none() {
            self.dragging = Some((x, y, plant.clone()));
        }
    }

    fn drop_plant(&mut self, x: f32, y: f32) {
        if let Some((_, _, plant)) = self.dragging.as_ref() {
            if self.money >= plant.cost() {
                if let Some(x) = self.map.coord_to_pos_x(x / self.surface.width()) {
                    if let Some(y) = self.map.coord_to_pos_y(y / self.surface.height()) {
                        match self.map.rows[y] {
                            config::RowType::Grass => {
                                if !plant.is_nenuphar() && self.plants[y][x].is_none() {
                                    self.money -= plant.cost();
                                    self.plants[y][x] = Some(plant.as_ref().clone());
                                }
                            }
                            config::RowType::Water => {
                                if plant.can_go_in_water() {
                                    if self.plants[y][x].is_none() {
                                        self.money -= plant.cost();
                                        self.plants[y][x] = Some(plant.as_ref().clone());
                                    }
                                } else if let Some(p) = self.plants[y][x].as_mut() {
                                    if p.is_nenuphar() {
                                        self.money -= plant.cost();
                                        *p = plant.as_ref().clone();
                                    }
                                }
                            }
                        }
                    }
                }
            }
            self.dragging = None;
        }
    }

    pub fn start(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let _self = self as *mut Self;
        if self.started.is_some() {
            return Ok(());
        }
        let mut rows: Vec<RowType> = self
            .shop_plants
            .iter()
            .flat_map(|_| [RowType::Ratio(132.5), RowType::Ratio(10.)])
            .collect();
        rows.insert(0, RowType::Ratio(10.));
        let moneyid = rows.len();
        rows.push(RowType::Ratio(37.5));
        let remain: f32 = rows
            .iter()
            .map(|r| if let RowType::Ratio(p) = r { *p } else { 0. })
            .sum();
        if remain < 1280. {
            rows.push(RowType::Ratio(1280. - remain));
        }

        let mut element =
            HashMap::from_iter(self.shop_plants.iter().enumerate().map(|(i, plant)| {
                (
                    Pos { x: 1, y: i * 2 + 1 },
                    Box::new(ShopPlant::new(Self::take_plant, plant.as_ref().clone()))
                        as Box<dyn GridChildren<Level>>,
                )
            }));
        element.insert(
            Pos { x: 1, y: moneyid },
            Box::new(
                UIRect::new(
                    &textures()?.font,
                    Box::new(|_, _| StateEnum::Enable),
                    Box::new(|_, _| Color::BLACK),
                )
                .text(Box::new(|_self: &Level, _| {
                    UIString::new(&textures()?.font, format!("{}$", _self.money))
                        .map(|s| (s, Color::WHITE))
                })),
            ) as Box<dyn GridChildren<Level>>,
        );
        let mut shop = Grid::new(
            self,
            vec![
                ColType::Ratio(10.),
                ColType::Ratio(100.),
                ColType::Ratio(1150.),
            ],
            rows,
            element,
        );
        shop.init(canvas)?;
        shop.init_frame(canvas, self.surface)?;
        self.started = Some(shop);
        Ok(())
    }
}

impl GridChildren<Win> for Level {
    fn grid_init(&mut self, canvas: &mut Canvas<Window>, _: &mut Win) -> Result<(), String> {
        let _self = self as *mut Self;
        let mut cols: Vec<ColType> = (0..self.map.cols)
            .flat_map(|_| {
                [
                    ColType::Ratio(5. / 1280.),
                    ColType::Ratio(self.map.col_width() - 10. / 1280.),
                    ColType::Ratio(5. / 1280.),
                ]
            })
            .collect();
        cols.insert(0, ColType::Ratio(self.map.left));
        cols.push(ColType::Ratio(1. - self.map.left - self.map.width));
        let mut rows: Vec<RowType> = (0..self.map.rows.len())
            .flat_map(|_| {
                [
                    RowType::Ratio(5. / 720.),
                    RowType::Ratio(self.map.row_heigth() - 10. / 720.),
                    RowType::Ratio(5. / 720.),
                ]
            })
            .collect();
        rows.insert(0, RowType::Ratio(self.map.top));
        rows.push(RowType::Ratio(1. - self.map.top - self.map.height));
        let rows_type = self.map.rows.clone();
        let rows_type: &[config::RowType] = rows_type.as_ref();
        self.map_plants = Grid::new(
            self,
            cols,
            rows,
            HashMap::from_iter(self.plants.iter().enumerate().flat_map(|(y, plants)| {
                plants.iter().enumerate().map(move |(x, plant)| {
                    (
                        Pos {
                            x: x * 3 + 2,
                            y: y * 3 + 2,
                        },
                        Box::new(MapPlant {
                            row_type: rows_type[y],
                            plant,
                            surface: FRect::new(0., 0., 0., 0.),
                        }) as Box<dyn GridChildren<Level>>,
                    )
                })
            })),
        );
        self.map_plants.grid_init(canvas, unsafe {
            _self.as_mut().ok_or("unwrap ptr init level")?
        })
    }

    fn grid_init_frame(
        &mut self,
        canvas: &mut Canvas<Window>,
        surface: FRect,
        _: &mut Win,
    ) -> Result<(), String> {
        if self.surface != surface {
            //TODO
            self.surface = surface;
        }
        if let Some(started) = self.started.as_mut() {
            started.init_frame(canvas, self.surface)?;
        }
        self.map_plants.init_frame(canvas, surface)
    }

    fn grid_event(
        &mut self,
        canvas: &mut Canvas<Window>,
        event: Event,
        _: &mut Win,
    ) -> Result<(), String> {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                self.start(canvas)?;
            }
            Event::MouseMotion { x, y, .. } => {
                for i in self
                    .suns
                    .iter()
                    .enumerate()
                    .filter_map(|(i, sun)| {
                        if sun.rect().contains_point(FPoint::new(
                            x / self.surface.width(),
                            y / self.surface.height(),
                        )) {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .rev()
                    .collect::<Vec<usize>>()
                {
                    self.money += 25;
                    self.suns.remove(i);
                }
                if let Some(plant) = self.dragging.as_mut() {
                    plant.0 = x / self.surface.width();
                    plant.1 = y / self.surface.height();
                }
            }
            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                self.drop_plant(x, y);
            }
            _ => {}
        }

        if let Some(started) = self.started.as_mut() {
            started.event(canvas, event.clone())?;
        }
        self.map_plants.event(canvas, event)?;
        Ok(())
    }

    fn grid_update(
        &mut self,
        canvas: &mut Canvas<Window>,
        elapsed: Duration,
        _: &mut Win,
    ) -> Result<(), String> {
        if self.started.is_none() {
            return Ok(());
        }
        if self.end.is_some() {
            return Ok(());
        }
        if !self.zombies.iter().flatten().any(|_| true) && self.spawn_waits.is_empty() {
            self.end = Some(true);
            return Ok(());
        }
        for plant in self.plants.iter_mut().flatten().flatten() {
            plant.update(elapsed)?;
        }
        self.map_plants.update(canvas, elapsed)?;
        self.update_zombies(elapsed)?;
        if let Some(false) = self.end {
            return Ok(());
        }
        self.update_projectiles(elapsed)?;
        self.update_suns(elapsed)?;
        self.spawn_projectiles();
        self.update_zombie_wave(elapsed);
        if let Some(started) = self.started.as_mut() {
            started.update(canvas, elapsed)?;
        }
        Ok(())
    }

    fn grid_draw(&self, canvas: &mut Canvas<Window>, parent: &Win) -> Result<(), String> {
        canvas.copy(
            &textures::textures()?.maps[self.map.id as usize],
            Some(Rect::new(
                if self.started.is_none() { 238 } else { 0 },
                0,
                762,
                429,
            )),
            None,
        )?;

        if let Some(started) = self.started.as_ref() {
            self.map_plants.draw(canvas)?;
            self.draw_zombies(canvas)?;
            self.draw_projectiles(canvas)?;
            started.draw(canvas)?;
            self.draw_suns(canvas)?;
            if let Some(end) = self.end {
                draw_string(
                    canvas,
                    &textures()?.font,
                    None,
                    scale(self.surface, FRect::new(0.25, 0.25, 0.5, 0.5)),
                    if end {
                        &parent.texts()?.win
                    } else {
                        &parent.texts()?.lost
                    },
                    Color::WHITE,
                )?;
            }
            if let Some((x, y, plant)) = self.dragging.as_ref() {
                canvas.copy_f(
                    plant.texture()?,
                    None,
                    scale(
                        self.surface,
                        FRect::new(
                            x - (self.map.col_width() - 10. / 1280.) / 2.,
                            y - (self.map.row_heigth() - 10. / 720.) / 2.,
                            self.map.col_width() - 10. / 1280.,
                            self.map.row_heigth() - 10. / 720.,
                        ),
                    ),
                )?;
            }
            return Ok(());
        }

        let mut t: Vec<&(u8, f32, f32)> = self.spawn_zombies.iter().flatten().collect();
        t.sort_by(|(_, _, y1), (_, _, y2)| y1.total_cmp(y2));
        for &(z, x, y) in t {
            let mut z = zombie_from_id(z);
            z.set_x(x);
            canvas.copy_f(z.texture()?, None, scale(self.surface, z.rect(y)))?;
        }
        Ok(())
    }
}
