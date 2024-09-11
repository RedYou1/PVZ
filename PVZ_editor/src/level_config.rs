use std::{collections::HashMap, time::Duration};

use pvz::{
    level::{config::Map, Level},
    textures::textures,
    zombie::{valide_zombie_id, zombie_from_id},
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
    ttf::Font,
    video::Window,
};

use crate::win::Win;

pub struct LevelConfig {
    pub level: Level,
    surface: FRect,
    grid: Grid<LevelConfig>,
    pub money: UIString,
    pub map: UIString,
    #[allow(clippy::type_complexity)]
    action: Option<Box<dyn FnMut(&mut LevelConfig) -> Result<(), String>>>,
    waves: Vec<(UIString, Vec<(UIString, UIString)>)>,
    selected: Option<(String, usize, Option<usize>)>,
}

impl LevelConfig {
    pub fn new(id: u8) -> Result<Self, String> {
        let level = Level::load(id).map_err(|e| e.to_string())?;
        let font = &textures()?.font;
        let waves = level
            .spawn_waits
            .iter()
            .enumerate()
            .flat_map(|(i, wait)| {
                let mut zombies = HashMap::new();
                for z in level.spawn_zombies[i].iter().map(|z| z.0) {
                    if let Some(v) = zombies.get_mut(&z) {
                        *v += 1;
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
        let map =
            UIString::new(font, level.map.id.to_string())?.ok_or("map id too large".to_owned())?;
        let money =
            UIString::new(font, level.money.to_string())?.ok_or("money too large".to_owned())?;
        Ok(Self {
            level,
            surface: FRect::new(0., 0., 0., 0.),
            grid: unsafe { Grid::empty() },
            action: None,
            waves,
            selected: None,
            map,
            money,
        })
    }

    pub fn empty(font: &'static Font<'static, 'static>) -> Self {
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
            action: None,
            waves: Vec::new(),
            selected: None,
            map: UIString::empty(font),
            money: UIString::new_const(font, "50"),
        }
    }

    #[allow(clippy::too_many_lines)]
    fn reset(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let font = &textures()?.font;
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
                    Box::new(|_, t| {
                        if t.text().as_str().parse::<u8>().is_ok() {
                            Color::BLACK
                        } else {
                            Color::RED
                        }
                    }),
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
                        Box::new(|_, t| {
                            if let Ok(id) = t.text().as_str().parse::<u8>() {
                                if valide_zombie_id(id) {
                                    return Color::BLACK;
                                }
                            }
                            Color::RED
                        }),
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
                        Box::new(|_, t| {
                            if let Ok(t) = t.text().as_str().parse::<u8>() {
                                if t != 0 {
                                    Color::BLACK
                                } else {
                                    Color::RED
                                }
                            } else {
                                Color::RED
                            }
                        }),
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
            elements.insert(
                Pos { x: 0, y: i },
                Box::new(
                    UIRect::new(
                        font,
                        Box::new(|_, _| StateEnum::Enable),
                        Box::new(|_, _| Color::BLACK),
                    )
                    .text(Box::new(|_, _| {
                        Ok((UIString::new(font, "+ Zombie".to_owned())?, Color::WHITE))
                    }))
                    .action(Box::new(
                        move |_self: &mut LevelConfig, _, _, _, _| {
                            _self.action = Some(Box::new(move |_self| {
                                let font = &textures()?.font;
                                _self.waves[i1].1.push((
                                    UIString::empty(font),
                                    UIString::empty(font),
                                ));
                                Ok(())
                            }));
                            Ok(())
                        },
                    )),
                ) as Box<dyn GridChildren<LevelConfig>>,
            );
            elements.insert(
                Pos { x: 2, y: i },
                Box::new(
                    UIRect::new(
                        font,
                        Box::new(|_, _| StateEnum::Enable),
                        Box::new(|_, _| Color::BLACK),
                    )
                    .text(Box::new(|_, _| {
                        Ok((UIString::new(font, "- Zombie".to_owned())?, Color::WHITE))
                    }))
                    .action(Box::new(
                        move |_self: &mut LevelConfig, _, _, _, _| {
                            _self.action = Some(Box::new(move |_self| {
                                _self.waves[i1].1.pop();
                                if _self.waves[i1].1.is_empty() {
                                    _self.waves.remove(i1);
                                }
                                Ok(())
                            }));
                            Ok(())
                        },
                    )),
                ) as Box<dyn GridChildren<LevelConfig>>,
            );
            i += 1;
            elements.insert(
                Pos { x: 0, y: i },
                Box::new(
                    UIRect::new(
                        font,
                        Box::new(|_, _| StateEnum::Enable),
                        Box::new(|_, _| Color::BLACK),
                    )
                    .text(Box::new(|_, _| {
                        Ok((UIString::new(font, "+ Wave".to_owned())?, Color::WHITE))
                    }))
                    .action(Box::new(
                        move |_self: &mut LevelConfig, _, _, _, _| {
                            _self.action = Some(Box::new(move |_self| {
                                let font = &textures()?.font;
                                _self.waves.insert(
                                    i1 + 1,
                                    (
                                        UIString::empty(font),
                                        vec![(
                                            UIString::empty(font),
                                            UIString::empty(font),
                                        )],
                                    ),
                                );
                                Ok(())
                            }));
                            Ok(())
                        },
                    )),
                ) as Box<dyn GridChildren<LevelConfig>>,
            );
            elements.insert(
                Pos { x: 2, y: i },
                Box::new(
                    UIRect::new(
                        font,
                        Box::new(|_, _| StateEnum::Enable),
                        Box::new(|_, _| Color::BLACK),
                    )
                    .text(Box::new(|_, _| {
                        Ok((UIString::new(font, "- Wave".to_owned())?, Color::WHITE))
                    }))
                    .action(Box::new(
                        move |_self: &mut LevelConfig, _, _, _, _| {
                            _self.action = Some(Box::new(move |_self| {
                                _self.waves.remove(i1);
                                Ok(())
                            }));
                            Ok(())
                        },
                    )),
                ) as Box<dyn GridChildren<LevelConfig>>,
            );
            i += 1;
        }
        if i == 0 {
            elements.insert(
                Pos { x: 0, y: i },
                Box::new(
                    UIRect::new(
                        font,
                        Box::new(|_, _| StateEnum::Enable),
                        Box::new(|_, _| Color::BLACK),
                    )
                    .text(Box::new(|_, _| {
                        Ok((UIString::new(font, "+ Wave".to_owned())?, Color::WHITE))
                    }))
                    .action(Box::new(
                        move |_self: &mut LevelConfig, _, _, _, _| {
                            _self.action = Some(Box::new(|_self| {
                                let font = &textures()?.font;
                                _self.waves.push((
                                    UIString::empty(font),
                                    vec![(
                                        UIString::empty(font),
                                        UIString::empty(font),
                                    )],
                                ));
                                Ok(())
                            }));
                            Ok(())
                        },
                    )),
                ) as Box<dyn GridChildren<LevelConfig>>,
            );
            i = 1;
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

    pub fn try_save(&mut self) -> Result<(), String> {
        self.level.money = self
            .money
            .as_str()
            .parse::<u32>()
            .map_err(|e| e.to_string())?;
        self.level.map = Map::load(self.map.as_str().parse::<u8>().map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        self.level.spawn_waits = self
            .waves
            .iter()
            .map(|(wait, _)| {
                Ok(Duration::from_secs(
                    wait.as_str().parse::<u8>().map_err(|e| e.to_string())? as u64,
                ))
            })
            .collect::<Result<Vec<Duration>, String>>()?;
        self.level.spawn_zombies = self
            .waves
            .iter()
            .map(|(_, zombies)| {
                Ok(zombies
                    .iter()
                    .map(|(zombie, amount)| {
                        let zombie = zombie.as_str().parse::<u8>().map_err(|e| e.to_string())?;
                        let amount = amount.as_str().parse::<u8>().map_err(|e| e.to_string())?;
                        if amount == 0 {
                            return Err("Amount too low".to_owned());
                        }
                        Ok::<Vec<(u8, f32, f32)>, String>(
                            (0..amount)
                                .map(move |_| (zombie, 0., 0.))
                                .collect(),
                        )
                    })
                    .collect::<Result<Vec<Vec<(u8, f32, f32)>>, String>>()?
                    .into_iter()
                    .flatten()
                    .collect())
            })
            .collect::<Result<Vec<Vec<(u8, f32, f32)>>, String>>()?;
        Ok(())
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
        let _self = self as *mut Self;
        if let Some(action) = self.action.as_mut() {
            (*action)(unsafe {
                _self
                    .as_mut()
                    .ok_or("unwrap self level_config".to_owned())?
            })?;
            self.reset(canvas)?;
            self.action = None;
        }
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
        self.grid.update(canvas, elapsed)?;
        Ok(())
    }

    fn grid_draw(&self, canvas: &mut Canvas<Window>, _: &Win) -> Result<(), String> {
        self.grid.draw(canvas)
    }
}
