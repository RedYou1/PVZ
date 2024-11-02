use std::{collections::HashMap, fs, time::Duration};

use sdl::{
    event::Event,
    functions::StateEnum,
    game_window::GameWindow,
    grid::{ColType, Grid, GridChildren, Pos, RowType},
    missing::ui_string::UIString,
    scroll_view::ScrollView,
    simple_grid,
    ui_rect::UIRect,
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
    level::Level,
    save::SaveFile,
    texts::{load_texts, Texts},
    textures::{load_textures, textures},
    UPDATE_AVAILABLE,
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
        load_texts(&textures()?.font)?;
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

    pub fn texts(&self) -> Result<&'static Texts, String> {
        self.save.texts()
    }

    fn next_lang(
        &mut self,
        _: &UIRect<Win>,
        _: f32,
        _: f32,
        _: &mut Canvas<Window>,
    ) -> Result<(), String> {
        self.save.next_lang()
    }

    fn change_full_screen(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let window = canvas.window_mut();
        window.set_fullscreen(if window.fullscreen_state() == FullscreenType::Off {
            FullscreenType::Desktop
        } else {
            FullscreenType::Off
        })?;
        Ok(())
    }

    fn _return(
        &mut self,
        _: &UIRect<Win>,
        _: f32,
        _: f32,
        _: &mut Canvas<Window>,
    ) -> Result<(), String> {
        self.level = None;
        self.pause = false;
        Ok(())
    }

    fn quit(
        &mut self,
        _: &UIRect<Win>,
        _: f32,
        _: f32,
        _: &mut Canvas<Window>,
    ) -> Result<(), String> {
        self.running = false;
        Ok(())
    }

    fn menu(
        &mut self,
        _: &UIRect<Win>,
        _: f32,
        _: f32,
        _: &mut Canvas<Window>,
    ) -> Result<(), String> {
        self.pause = !self.pause;
        Ok(())
    }

    fn play(
        &mut self,
        _: &UIRect<Win>,
        _: f32,
        _: f32,
        canvas: &mut Canvas<Window>,
    ) -> Result<(), String> {
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
        self.main_menu = simple_grid!(
            self,
            Win,
            ColType::Ratio(380.),
            ColType::Ratio(275.),
            ColType::Ratio(10.),
            ColType::Ratio(235.),
            ColType::Ratio(380.);
            RowType::Ratio(175.),
            RowType::Ratio(370.),
            RowType::Ratio(175.);
            Pos { x: 1, y: 1 } => simple_grid!(
                self,
                Win,
                ColType::Ratio(1.);
                RowType::Ratio(10.),
                RowType::Ratio(1.),
                RowType::Ratio(10.),
                RowType::Ratio(1.),
                RowType::Ratio(10.),
                RowType::Ratio(1.),
                RowType::Ratio(10.),
                RowType::Ratio(1.),
                RowType::Ratio(10.);
                Pos { x: 0, y: 0 } => UIRect::new(Box::new(|_,_|StateEnum::Enable), Box::new(|_,_| Color::BLACK)).action(Box::new(Self::next_lang)).text(Box::new(|_self, _| Ok((Some(_self.texts()?.lang.clone()), Color::WHITE)))),
                Pos { x: 0, y: 2 } => UIRect::new(Box::new(|_:&Win,_|StateEnum::Enable), Box::new(|_,_| Color::BLACK)).action(Box::new(|_self,_,_,_,canvas|_self.change_full_screen(canvas))).text(Box::new(|_self, _| Ok((Some(_self.texts()?.full_screen.clone()), Color::WHITE)))),
                Pos { x: 0, y: 4 } => UIRect::new(Box::new(|_,_|StateEnum::Enable), Box::new(|_,_| Color::BLACK)).action(Box::new(Self::quit)).text(Box::new(|_self, _| Ok((Some(_self.texts()?.quit.clone()), Color::WHITE)))),
                Pos { x: 0, y: 6 } => UIRect::new(Box::new(|_,_|StateEnum::Enable), Box::new(|_,_| Color::BLACK)).text(Box::new(|_self: &Win,_|Ok((Some(match unsafe { UPDATE_AVAILABLE.as_ref() } {
                    Some(Ok(true)) => _self.texts()?.update_available.clone(),
                    Some(Ok(false)) => _self.texts()?.up_to_date.clone(),
                    Some(Err(e)) => UIString::new(&textures()?.font, e.clone())?.ok_or("Error too long".to_owned())?,
                    None => _self.texts()?.loading.clone(),
                }), Color::WHITE)))),
            ),
            Pos { x: 3, y: 1 } => ScrollView::new(Grid::new(
                self,
                vec![ColType::Ratio(1.)],
                (0..self.levels_count).flat_map(|_| [RowType::Ratio(10.),RowType::Ratio(1.)]).take((self.levels_count as usize)*2-1).collect(),
                HashMap::from_iter((0..self.levels_count).map(|level| {
                    (
                        Pos { x: 0, y: level as usize * 2 },
                        Box::new(UIRect::new(
                            Box::new(|_,_|StateEnum::Enable), Box::new(|_,_| Color::BLACK)).action(Box::new(
                            move |_self: &mut Win, _, _, _, canvas| {
                                let win = _self as *mut Win;
                                _self.level = Some(
                                    Level::load(level)
                                        .map_err(|e| e.to_string())?,
                                );
                                _self
                                    .level
                                    .as_mut()
                                    .ok_or("unwrap level after init")?
                                    .grid_init(canvas, unsafe {
                                        win.as_mut().ok_or("unwrap level after init2")?
                                    })
                            })).text(Box::new(
                                move |_self,_| {
                                UIString::new(&textures()?.font, format!("{:0>3}", level+1)).map(|s| (s, Color::WHITE))
                            },
                        ))) as Box<dyn GridChildren<Win>>,
                    )
                }))
            ), 235., 90. * self.levels_count as f32, Box::new(|_, _|Color::RGBA(200,200,200,200))),
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
            Pos { x: 1, y: 1 } => UIRect::new(Box::new(|_,_|StateEnum::Enable), Box::new(|_,_| Color::BLACK)).action(Box::new(Self::next_lang)).text(Box::new(|_self, _| Ok((Some(_self.texts()?.lang.clone()), Color::WHITE)))),
            Pos { x: 1, y: 3 } => UIRect::new(Box::new(|_:&Win,_|StateEnum::Enable), Box::new(|_,_| Color::BLACK)).action(Box::new(|_self,_,_,_,canvas|_self.change_full_screen(canvas))).text(Box::new(|_self, _| Ok((Some(_self.texts()?.full_screen.clone()), Color::WHITE)))),
            Pos { x: 1, y: 5 } => UIRect::new(Box::new(|_,_|StateEnum::Enable), Box::new(|_,_| Color::BLACK)).action(Box::new(Self::_return)).text(Box::new(|_self, _| Ok((Some(_self.texts()?._return.clone()), Color::WHITE)))),
            Pos { x: 1, y: 7 } => UIRect::new(Box::new(|_,_|StateEnum::Enable), Box::new(|_,_| Color::BLACK)).action(Box::new(Self::quit)).text(Box::new(|_self, _| Ok((Some(_self.texts()?.quit.clone()), Color::WHITE)))),
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
            Pos { x: 1, y: 1 } => UIRect::new(Box::new(|_,_|StateEnum::Enable), Box::new(|_,_| Color::BLACK)).action(Box::new(Self::menu)).text(Box::new(|_self, _| Ok((Some(_self.texts()?.menu.clone()), Color::WHITE)))),
            Pos { x: 1, y: 3 } => UIRect::new(Box::new(|_self: &Win,_| if let Some(level) = _self.level.as_ref() {
                if level.started.is_none() {
                    StateEnum::Enable
                } else {
                    StateEnum::Hidden
                }
            } else {
                StateEnum::Hidden
            }), Box::new(|_,_| Color::BLACK)).action(Box::new(Self::play)).text(Box::new(|_self,_| Ok((Some(_self.texts()?.start.clone()), Color::WHITE)))),
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
                self.change_full_screen(canvas)?;
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
