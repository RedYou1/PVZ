use anyhow::{anyhow, Result};
use config::Map;
use red_sdl::{
    event::Event,
    missing::{rect::scale, ui_string::UIString},
    refs::{MutRef, Ref},
    ui_element::{
        grid::{ColType, Grid, Pos, RowType},
        ui_rect::UIRect,
    },
    user_control::UserControl,
    zero,
};
use red_sdl_macro::UserControl;
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
    default_button,
    map_plant::MapPlant,
    plants::{
        nenuphar::Nenuphar, peashooter::PeaShooter, sunflower::Sunflower,
        triple_peashooter::PlantTriple, Plant,
    },
    projectile::{DamageType, Projectile},
    shop_plant::ShopPlant,
    sun::Sun,
    win::Win,
    zombie::{zombie_from_id, Zombie},
    State,
};

pub struct Level {
    pub id: u8,
    pub started: Option<Grid<Level, State, LevelShopElement>>,
    pub surface: FRect,
    pub suns: Vec<Sun>,
    pub next_sun: Duration,
    pub map_plants: Grid<Level, State, MapPlant>,
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

#[derive(UserControl)]
#[parent(Level)]
#[state(State)]
pub enum LevelShopElement {
    Plant(ShopPlant),
    Text(UIRect<Level, State>),
}

impl Level {
    fn new(
        level: u8,
        map: Map,
        rows: usize,
        money: u32,
        spawn_waits: Vec<Duration>,
        spawn_zombies: Vec<Vec<(u8, f32, f32)>>,
    ) -> Self {
        let (c_width, c_height) = (map.col_width(), map.row_heigth());
        let rows_type = &map.rows;
        Self {
            id: level,
            started: None,
            surface: zero(),
            suns: Vec::with_capacity(4),
            next_sun: Duration::new(5, 0),
            map_plants: Grid::new(
                {
                    let mut cols: Vec<ColType> = (0..map.cols)
                        .flat_map(|_| {
                            [
                                ColType::Ratio(5. / 1280.),
                                ColType::Ratio(c_width - 10. / 1280.),
                                ColType::Ratio(5. / 1280.),
                            ]
                        })
                        .collect();
                    cols.insert(0, ColType::Ratio(map.left));
                    cols.push(ColType::Ratio(1. - map.left - map.width));
                    cols
                },
                {
                    let mut rows: Vec<RowType> = (0..rows_type.len())
                        .flat_map(|_| {
                            [
                                RowType::Ratio(5. / 720.),
                                RowType::Ratio(c_height - 10. / 720.),
                                RowType::Ratio(5. / 720.),
                            ]
                        })
                        .collect();
                    rows.insert(0, RowType::Ratio(map.top));
                    rows.push(RowType::Ratio(1. - map.top - map.height));
                    rows
                },
                HashMap::from_iter((0..rows).flat_map(|y| {
                    (0..map.cols as usize).map(move |x| {
                        (
                            Pos {
                                x: x * 3 + 2,
                                y: y * 3 + 2,
                            },
                            MapPlant {
                                row_type: rows_type[y],
                                plant: None,
                                surface: zero(),
                            },
                        )
                    })
                })),
            ),
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
        }
    }

    fn take_plant(mut this: MutRef<Self>, plant: Box<dyn Plant>, x: f32, y: f32) {
        if this.dragging.is_none() {
            this.dragging = Some((x, y, plant));
        }
    }

    fn drop_plant(&mut self, x: f32, y: f32) {
        if let Some((_, _, plant)) = self.dragging.as_ref() {
            if self.money >= plant.cost() {
                if let Some(x) = self.map.coord_to_pos_x(x / self.surface.width()) {
                    if let Some(y) = self.map.coord_to_pos_y(y / self.surface.height()) {
                        if let Some(slot) = self.map_plants.get_element_mut(x * 3 + 2, y * 3 + 2) {
                            match self.map.rows[y] {
                                config::RowType::Grass => {
                                    if !plant.is_nenuphar() && slot.plant.is_none() {
                                        self.money -= plant.cost();
                                        slot.plant = Some(plant.as_ref().clone());
                                    }
                                }
                                config::RowType::Water => {
                                    if plant.can_go_in_water() && slot.plant.is_none() {
                                        self.money -= plant.cost();
                                        slot.plant = Some(plant.as_ref().clone());
                                    } else if let Some(nen) = slot.plant.as_ref() {
                                        if nen.is_nenuphar() {
                                            self.money -= plant.cost();
                                            slot.plant = Some(plant.as_ref().clone());
                                        }
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

    pub fn start(
        mut this: MutRef<Self>,
        canvas: &Canvas<Window>,
        state: MutRef<State>,
    ) -> Result<()> {
        if this.started.is_some() {
            return Ok(());
        }
        let mut rows: Vec<RowType> = this
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
            HashMap::from_iter(this.shop_plants.iter().enumerate().map(|(i, plant)| {
                (
                    Pos { x: 1, y: i * 2 + 1 },
                    ShopPlant::new(Self::take_plant, plant.as_ref().clone()).into(),
                )
            }));
        element.insert(
            Pos { x: 1, y: moneyid },
            default_button()
                .text(Box::new(|_, _self: Ref<Level>, _state: Ref<State>| {
                    UIString::new(
                        _state.as_ref().textures().font(),
                        format!("{}$", _self.money),
                    )
                    .map(|s| (s, Color::WHITE))
                }))
                .into(),
        );
        let mut grid = Grid::new(
            vec![
                ColType::Ratio(10.),
                ColType::Ratio(100.),
                ColType::Ratio(1150.),
            ],
            rows,
            element,
        );
        let surface = this.surface;
        UserControl::event(
            (&mut grid).into(),
            canvas,
            Event::ElementMove {
                x: surface.x(),
                y: surface.y(),
            },
            this,
            state,
        )?;
        UserControl::event(
            (&mut grid).into(),
            canvas,
            Event::ElementResize {
                width: surface.width(),
                height: surface.height(),
            },
            this,
            state,
        )?;
        this.started = Some(grid);
        Ok(())
    }
}

impl UserControl<Win, State> for Level {
    fn surface(this: Ref<Self>, _: Ref<Win>, _: Ref<State>) -> FRect {
        this.surface
    }

    fn event(
        mut this: MutRef<Self>,
        canvas: &Canvas<Window>,
        event: Event,
        _: MutRef<Win>,
        state: MutRef<State>,
    ) -> Result<()> {
        match event {
            Event::ElementMove { x, y } => {
                this.surface.set_x(x);
                this.surface.set_y(y);
            }
            Event::ElementResize { width, height } => {
                this.surface.set_width(width);
                this.surface.set_height(height);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                Self::start(this, canvas, state)?;
            }
            Event::MouseMotion { x, y, .. } => {
                for i in this
                    .suns
                    .iter()
                    .enumerate()
                    .filter_map(|(i, sun)| {
                        if sun.rect().contains_point(FPoint::new(
                            x / this.surface.width(),
                            y / this.surface.height(),
                        )) {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .rev()
                    .collect::<Vec<usize>>()
                {
                    this.money += 25;
                    this.suns.remove(i);
                }
                if let Some(plant) = this.as_mut().dragging.as_mut() {
                    plant.0 = x / this.surface.width();
                    plant.1 = y / this.surface.height();
                }
            }
            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                this.drop_plant(x, y);
            }
            _ => {}
        }

        if let Some(started) = this.started.as_mut() {
            UserControl::event(started.into(), canvas, event.clone(), this, state)?;
        }
        UserControl::event((&mut this.map_plants).into(), canvas, event, this, state)
    }

    fn update(
        mut this: MutRef<Self>,
        canvas: &Canvas<Window>,
        elapsed: Duration,
        _: MutRef<Win>,
        state: MutRef<State>,
    ) -> Result<()> {
        if this.started.is_none() {
            return Ok(());
        }
        if this.end.is_some() {
            return Ok(());
        }
        if !this.zombies.iter().flatten().any(|_| true) && this.spawn_waits.is_empty() {
            this.end = Some(true);
            return Ok(());
        }
        for (_, mut plant) in this.map_plants.iter_mut() {
            if let Some(plant) = plant.plant.as_mut() {
                plant.update(elapsed)?;
            }
        }
        UserControl::update((&mut this.map_plants).into(), canvas, elapsed, this, state)?;
        this.as_mut().update_zombies(elapsed)?;
        if let Some(false) = this.end {
            return Ok(());
        }
        this.as_mut().update_projectiles(elapsed)?;
        this.as_mut().update_suns(elapsed)?;
        this.as_mut().spawn_projectiles();
        this.as_mut().update_zombie_wave(elapsed);
        if let Some(started) = this.as_mut().started.as_mut() {
            UserControl::update(started.into(), canvas, elapsed, this, state)?;
        }
        Ok(())
    }

    fn draw(
        this: Ref<Self>,
        canvas: &mut Canvas<Window>,
        _: Ref<Win>,
        state: Ref<State>,
    ) -> Result<()> {
        canvas
            .copy(
                state.as_ref().textures().map(this.map.id as usize),
                Some(Rect::new(
                    if this.started.is_none() { 238 } else { 0 },
                    0,
                    762,
                    429,
                )),
                None,
            )
            .map_err(|e| anyhow!(e))?;

        if let Some(started) = this.started.as_ref() {
            UserControl::draw((&this.map_plants).into(), canvas, this, state)?;
            this.as_ref().draw_zombies(canvas, state.as_ref())?;
            this.as_ref().draw_projectiles(canvas, state.as_ref())?;
            UserControl::draw(started.into(), canvas, this, state)?;
            this.as_ref().draw_suns(canvas, state)?;
            if let Some(end) = this.end {
                if end {
                    &state.texts().win
                } else {
                    &state.texts().lost
                }
                .draw(
                    canvas,
                    None,
                    scale(this.surface, FRect::new(0.25, 0.25, 0.5, 0.5)),
                    Color::WHITE,
                )?;
            }
            if let Some((x, y, plant)) = this.as_ref().dragging.as_ref() {
                canvas
                    .copy_f(
                        plant.texture(state),
                        None,
                        scale(
                            this.surface,
                            FRect::new(
                                x - (this.map.col_width() - 10. / 1280.) / 2.,
                                y - (this.map.row_heigth() - 10. / 720.) / 2.,
                                this.map.col_width() - 10. / 1280.,
                                this.map.row_heigth() - 10. / 720.,
                            ),
                        ),
                    )
                    .map_err(|e| anyhow!(e))?;
            }
            return Ok(());
        }

        let mut t: Vec<&(u8, f32, f32)> = this.spawn_zombies.iter().flatten().collect();
        t.sort_by(|(_, _, y1), (_, _, y2)| y1.total_cmp(y2));
        for &(z, x, y) in t {
            let mut z = zombie_from_id(z);
            z.set_x(x);
            canvas
                .copy_f(
                    z.texture(state.as_ref().textures()),
                    None,
                    scale(this.surface, z.rect(y)),
                )
                .map_err(|e| anyhow!(e))?;
        }
        Ok(())
    }
}
