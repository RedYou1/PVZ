use std::{collections::HashMap, fs, time::Duration};

use sdl::{
    button::Button,
    event::Event,
    game_window::GameWindow,
    grid::{ColType, Grid, GridChildren, Pos, RowType},
    simple_grid,
    user_control::UserControl,
};
use sdl2::{
    keyboard::Keycode,
    pixels::Color,
    rect::FRect,
    render::Canvas,
    video::{FullscreenType, Window},
};

use crate::{
    level::{config::LevelConfig, Level},
    save::SaveFile,
    texts::Texts,
    textures::{load_textures, textures},
    update::Update,
};

pub struct Win {
    running: bool,
    pub pause: bool,
    save: SaveFile,

    levels_count: u8,
    level: Option<Level>,

    main_menu: Grid<Win>,
    options: Grid<Win>,
    overlay: Grid<Win>,
}

impl Win {
    pub fn new(canvas: &mut Canvas<Window>) -> Result<Self, String> {
        load_textures(canvas, Box::leak(Box::new(canvas.texture_creator())))?;
        let levels_count = fs::read_dir("levels").map_err(|e| e.to_string())?.count();
        if levels_count == 0 || fs::read_dir("levels").map_err(|e| e.to_string())?.count() > 99 {
            return Err("Too much or no levels".to_owned());
        }

        Ok(Self {
            running: true,
            pause: false,
            save: SaveFile::load()?,
            levels_count: levels_count as u8,
            level: None,
            main_menu: unsafe { Grid::empty() },
            options: unsafe { Grid::empty() },
            overlay: unsafe { Grid::empty() },
        })
    }

    pub const fn texts(&self) -> &'static Texts {
        self.save.texts()
    }

    fn next_lang(&mut self, _: f32, _: f32, _: &mut Canvas<Window>) -> Result<(), String> {
        self.save.next_lang()
    }

    fn change_full_screen(
        &mut self,
        _: f32,
        _: f32,
        canvas: &mut Canvas<Window>,
    ) -> Result<(), String> {
        let window = canvas.window_mut();
        window.set_fullscreen(if window.fullscreen_state() == FullscreenType::Off {
            FullscreenType::Desktop
        } else {
            FullscreenType::Off
        })?;
        Ok(())
    }

    fn _return(&mut self, _: f32, _: f32, _: &mut Canvas<Window>) -> Result<(), String> {
        self.level = None;
        self.pause = false;
        Ok(())
    }

    fn quit(&mut self, _: f32, _: f32, _: &mut Canvas<Window>) -> Result<(), String> {
        self.running = false;
        Ok(())
    }

    fn menu(&mut self, _: f32, _: f32, _: &mut Canvas<Window>) -> Result<(), String> {
        self.pause = !self.pause;
        Ok(())
    }

    fn play(&mut self, _: f32, _: f32, canvas: &mut Canvas<Window>) -> Result<(), String> {
        if let Some(level) = self.level.as_mut() {
            level.start(canvas)?;
        }
        Ok(())
    }
}

impl GameWindow for Win {
    fn running(&mut self) -> bool {
        self.running
    }

    #[allow(clippy::too_many_lines)]
    fn init(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let textures = textures()?;
        self.main_menu = simple_grid!(
            self,
            Win,
            ColType::Ratio(485.),
            ColType::Ratio(150.),
            ColType::Ratio(10.),
            ColType::Ratio(150.),
            ColType::Ratio(485.);
            RowType::Ratio(200.),
            RowType::Ratio(320.),
            RowType::Ratio(200.);
            Pos { x: 1, y: 1 } => simple_grid!(
                self,
                Win,
                ColType::Ratio(1.);
                RowType::Ratio(40.),
                RowType::Ratio(20.),
                RowType::Ratio(40.),
                RowType::Ratio(20.),
                RowType::Ratio(40.),
                RowType::Ratio(20.),
                RowType::Ratio(40.);
                Pos { x: 0, y: 0 } => Button::new(&textures.font,Self::next_lang, |_self| _self.texts().lang),
                Pos { x: 0, y: 2 } => Button::new(&textures.font,Self::change_full_screen, |_self| _self.texts().full_screen),
                Pos { x: 0, y: 4 } => Button::new(&textures.font,Self::quit, |_self| _self.texts().quit),
                Pos { x: 0, y: 6 } => Update::new(Self::texts),
            ),
            Pos { x: 3, y: 1 } => Grid::new(
                self,
                vec![ColType::Ratio(1.)],
                (0..self.levels_count).map(|_| RowType::Ratio(1.)).collect(),
                HashMap::from_iter((0..self.levels_count).map(|level| {
                    (
                        Pos { x: 0, y: level as usize },
                        Box::new(Button::new(
                            &textures.font,
                            move |_self: &mut Win, _, _, canvas| {
                                let win = _self as *mut Win;
                                _self.level = Some(
                                    LevelConfig::load_config(level)
                                        .map_err(|e| e.to_string())?,
                                );
                                _self
                                    .level
                                    .as_mut()
                                    .ok_or("unwrap level after init")?
                                    .grid_init(canvas, unsafe {
                                        win.as_mut().ok_or("unwrap level after init2")?
                                    })
                            },
                            move |_| format!("{:0>3}", level + 1),
                        )) as Box<dyn GridChildren<Win>>,
                    )
                }))
            ),
        );
        self.options = simple_grid!(
            self,
            Win,
            ColType::Ratio(565.),
            ColType::Ratio(150.),
            ColType::Ratio(565.);
            RowType::Ratio(200.),
            RowType::Ratio(40.),
            RowType::Ratio(20.),
            RowType::Ratio(40.),
            RowType::Ratio(20.),
            RowType::Ratio(40.),
            RowType::Ratio(20.),
            RowType::Ratio(40.),
            RowType::Ratio(300.);
            Pos { x: 1, y: 1 } => Button::new(&textures.font,Self::next_lang, |_self| _self.texts().lang),
            Pos { x: 1, y: 3 } => Button::new(&textures.font,Self::change_full_screen, |_self| {
                        _self.texts().full_screen
                    }),
            Pos { x: 1, y: 5 } => Button::new(&textures.font,Self::_return, |_self| _self.texts()._return),
            Pos { x: 1, y: 7 } => Button::new(&textures.font,Self::quit, |_self| _self.texts().quit),
        );
        self.overlay = simple_grid!(
            self,
            Win,
            ColType::Ratio(1120.),
            ColType::Ratio(150.),
            ColType::Ratio(10.);
            RowType::Ratio(10.),
            RowType::Ratio(100.),
            RowType::Ratio(500.),
            RowType::Ratio(100.),
            RowType::Ratio(10.);
            Pos { x: 1, y: 1 } => Button::new(&textures.font,Self::menu, |_self| _self.texts().menu),
            Pos { x: 1, y: 3 } => Button::new(&textures.font,Self::play, |_self| {
                        if let Some(level) = _self.level.as_ref() {
                            if level.started.is_none() {
                                _self.texts().start
                            } else {
                                ""
                            }
                        } else {
                            ""
                        }
                    }),
        );
        self.main_menu.init(canvas)?;
        self.options.init(canvas)?;
        self.overlay.init(canvas)
    }

    fn init_frame(
        &mut self,
        canvas: &mut Canvas<Window>,
        width: f32,
        height: f32,
    ) -> Result<(), String> {
        let _self = self as *mut Win;
        self.main_menu
            .init_frame(canvas, FRect::new(0., 0., width, height))?;
        self.options
            .init_frame(canvas, FRect::new(0., 0., width, height))?;
        self.overlay
            .init_frame(canvas, FRect::new(0., 0., width, height))?;

        if let Some(level) = self.level.as_mut() {
            level.grid_init_frame(canvas, FRect::new(0., 0., width, height), unsafe {
                (_self).as_mut().ok_or("unwrap ptr init_frame win")?
            })?;
        }
        Ok(())
    }

    fn event(&mut self, canvas: &mut Canvas<Window>, event: Event) -> Result<(), String> {
        let _self = self as *mut Win;
        match event {
            Event::Quit { .. } => self.running = false,
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => self.pause = !self.pause,
            Event::KeyDown {
                keycode: Some(Keycode::F11),
                ..
            } => {
                self.change_full_screen(0., 0., canvas)?;
            }
            _ => {}
        }
        if let Some(level) = self.level.as_mut() {
            if self.pause {
                self.options.event(canvas, event.clone())?;
            }
            self.overlay.event(canvas, event.clone())?;
            level.grid_event(canvas, event, unsafe {
                (_self).as_mut().ok_or("unwrap ptr event win")?
            })?;
        } else {
            self.main_menu.event(canvas, event)?;
        }
        Ok(())
    }

    fn update(&mut self, canvas: &mut Canvas<Window>, elapsed: Duration) -> Result<(), String> {
        let _self = self as *mut Win;
        if let Some(level) = self.level.as_mut() {
            if self.pause {
                self.options.update(canvas, elapsed)?;
            } else {
                self.overlay.update(canvas, elapsed)?;
                level.grid_update(canvas, elapsed, unsafe {
                    (_self).as_mut().ok_or("unwrap ptr update win")?
                })?;
            }
        } else {
            self.main_menu.update(canvas, elapsed)?;
        }
        Ok(())
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        if let Some(level) = self.level.as_ref() {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
            level.grid_draw(canvas, self)?;
            self.overlay.draw(canvas)?;
            if self.pause {
                self.options.draw(canvas)?;
            }
            return Ok(());
        }
        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.clear();

        self.main_menu.draw(canvas)
    }
}
