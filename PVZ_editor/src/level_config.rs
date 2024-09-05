use std::{collections::HashMap, time::Duration};

use pvz::{
    level::{config::Map, Level},
    textures::textures,
    zombie::zombie_from_id,
};
use sdl::{
    event::Event,
    functions::StateEnum,
    grid::{ColType, Grid, GridChildren, Pos, RowType},
    missing::ui_string::UIString,
    scroll_view::ScrollView,
    simple_grid,
    text_box::TextBox,
    ui_rect::UIRect,
    user_control::UserControl,
};
use sdl2::{
    pixels::Color,
    rect::FRect,
    render::{Canvas, Texture},
    video::Window,
};

use crate::win::Win;

pub struct LevelConfig {
    level: Level,
    surface: FRect,
    grid: Grid<LevelConfig>,
    waves: Vec<(UIString, Vec<(UIString, UIString)>)>,
    selected: Option<(String, usize, Option<usize>)>,
}

impl LevelConfig {
    pub fn new(id: u8) -> Result<Self, String> {
        Ok(Self {
            level: Level::load(id).map_err(|e| e.to_string())?,
            surface: FRect::new(0., 0., 0., 0.),
            grid: unsafe { Grid::empty() },
            waves: Vec::new(),
            selected: None,
        })
    }

    pub fn empty() -> Self {
        Self {
            level: Level {
                id: 0,
                started: None,
                surface: FRect::new(0., 0., 0., 0.),
                suns: Vec::new(),
                next_sun: Duration::from_secs(0),
                plants: Vec::new(),
                map_plants: unsafe { Grid::empty() },
                zombies: Vec::new(),
                projectiles: Vec::new(),
                map: Map {
                    id: 0,
                    top: 0.,
                    left: 0.,
                    width: 0.,
                    height: 0.,
                    rows: Vec::new(),
                    cols: 0,
                },
                spawn_waits: Vec::new(),
                spawn_zombies: Vec::new(),
                shop_plants: Vec::new(),
                dragging: None,
                money: 0,
                end: None,
            },
            surface: FRect::new(0., 0., 0., 0.),
            grid: unsafe { Grid::empty() },
            waves: Vec::new(),
            selected: None,
        }
    }

    #[allow(clippy::too_many_lines)]
    fn reset(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let font = &textures()?.font;
        self.waves = self
            .level
            .spawn_waits
            .iter()
            .enumerate()
            .flat_map(|(i, wait)| {
                let mut zombies: HashMap<u8, u8> = HashMap::with_capacity(3);
                for z in self.level.spawn_zombies[i]
                    .iter()
                    .map(|(_type, _, _)| *_type)
                {
                    if let Some(zz) = zombies.get_mut(&z) {
                        *zz += 1;
                    } else {
                        zombies.insert(z, 1);
                    }
                }
                Ok::<(UIString, Vec<(UIString, UIString)>), String>((
                    UIString::new(font, wait.as_secs().to_string())?.ok_or("sized".to_owned())?,
                    zombies
                        .into_iter()
                        .flat_map(|(k, v)| {
                            Ok::<(UIString, UIString), String>((
                                UIString::new(font, k.to_string())?.ok_or("sized".to_owned())?,
                                UIString::new(font, v.to_string())?.ok_or("sized".to_owned())?,
                            ))
                        })
                        .collect(),
                ))
            })
            .collect();

        let mut elements = HashMap::with_capacity(self.waves.len() * 3);
        let mut i = 0;
        for (i1, (time, zombies)) in self.waves.iter_mut().enumerate() {
            elements.insert(
                Pos { x: 0, y: i },
                Box::new(TextBox::new(
                    format!("time:{i}"),
                    &mut self.selected,
                    font,
                    time,
                    Box::new(|_, _| StateEnum::Enable),
                    Box::new(|_, _| Color::RGBA(255, 255, 255, 100)),
                    Box::new(|_, _| Color::WHITE),
                    Box::new(|_, _| Color::WHITE),
                    Box::new(|_, _| Color::BLACK),
                )) as Box<dyn GridChildren<LevelConfig>>,
            );
            for (i2, (_type, amount)) in zombies.iter_mut().enumerate() {
                elements.insert(
                    Pos { x: 2, y: i },
                    Box::new(TextBox::new(
                        format!("type:{i}"),
                        &mut self.selected,
                        font,
                        _type,
                        Box::new(|_, _| StateEnum::Enable),
                        Box::new(|_, _| Color::RGBA(255, 255, 255, 100)),
                        Box::new(|_, _| Color::WHITE),
                        Box::new(|_, _| Color::WHITE),
                        Box::new(|_, _| Color::BLACK),
                    )) as Box<dyn GridChildren<LevelConfig>>,
                );
                elements.insert(
                    Pos { x: 4, y: i },
                    Box::new(TextBox::new(
                        format!("amount:{i}"),
                        &mut self.selected,
                        font,
                        amount,
                        Box::new(|_, _| StateEnum::Enable),
                        Box::new(|_, _| Color::RGBA(255, 255, 255, 100)),
                        Box::new(|_, _| Color::WHITE),
                        Box::new(|_, _| Color::WHITE),
                        Box::new(|_, _| Color::BLACK),
                    )) as Box<dyn GridChildren<LevelConfig>>,
                );
                elements.insert(
                    Pos { x: 6, y: i },
                    Box::new(
                        UIRect::new(
                            font,
                            Box::new(move |_self: &LevelConfig, _| {
                                match _self.waves[i1].1[i2].0.as_str().parse::<u8>() {
                                    Ok(0) | Ok(1) => StateEnum::Enable,
                                    _ => StateEnum::Hidden,
                                }
                            }),
                            Box::new(|_, _| Color::BLACK),
                        )
                        .image(Box::new(move |_self: &LevelConfig, _| {
                            let a: &'static Texture<'_> = zombie_from_id(
                                _self.waves[i1].1[i2]
                                    .0
                                    .as_str()
                                    .parse::<u8>()
                                    .map_err(|e| e.to_string())?,
                            )
                            .texture()?;
                            Ok(a)
                        })),
                    ) as Box<dyn GridChildren<LevelConfig>>,
                );
                i += 1;
            }
        }

        self.grid = simple_grid!(
            self,
            LevelConfig,
            ColType::Ratio(1.);
            RowType::Ratio(1.);
            Pos{x:0,y:0} => ScrollView::new(Grid::new(
                    self,
                    vec![
                        ColType::Ratio(100.),
                        ColType::Ratio(10.),
                        ColType::Ratio(100.),
                        ColType::Ratio(10.),
                        ColType::Ratio(100.),
                        ColType::Ratio(10.),
                        ColType::Ratio(100.),
                    ],
                    (0..i).map(|_| RowType::Ratio(1.)).collect(),
                    elements,
                ),
                1000.,
                95. * i as f32,
                Box::new(|_, _| Color::RGBA(255, 255, 255, 100)),
            ),
        );

        self.grid.init(canvas)
    }
}

impl GridChildren<Win> for LevelConfig {
    fn grid_init(&mut self, canvas: &mut Canvas<Window>, _: &mut Win) -> Result<(), String> {
        self.reset(canvas)
    }

    fn grid_init_frame(
        &mut self,
        canvas: &mut Canvas<Window>,
        surface: FRect,
        _: &mut Win,
    ) -> Result<(), String> {
        self.surface = surface;
        self.grid.init_frame(canvas, surface)
    }

    fn grid_event(
        &mut self,
        canvas: &mut Canvas<Window>,
        event: Event,
        _: &mut Win,
    ) -> Result<(), String> {
        self.grid.event(canvas, event)
    }

    fn grid_update(
        &mut self,
        canvas: &mut Canvas<Window>,
        elapsed: Duration,
        _: &mut Win,
    ) -> Result<(), String> {
        self.grid.update(canvas, elapsed)
    }

    fn grid_draw(&self, canvas: &mut Canvas<Window>, _: &Win) -> Result<(), String> {
        self.grid.draw(canvas)
    }
}
