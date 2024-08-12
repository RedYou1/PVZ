use std::{collections::HashMap, fs, time::Duration};

use pvz::{
    save::SaveFile,
    texts::Texts,
    textures::{load_textures, textures},
};
use sdl::{
    button::Button,
    event::Event,
    game_window::GameWindow,
    grid::{ColType, Grid, GridChildren, Pos, RowType},
    simple_grid,
    text_box::TextBox,
    user_control::UserControl,
};
use sdl2::{
    keyboard::Keycode,
    pixels::Color,
    rect::FRect,
    render::Canvas,
    video::{FullscreenType, Window},
};

pub struct Win {
    running: bool,
    save: SaveFile,

    maps_count: u8,
    levels_count: u8,

    selected: Option<(String, usize, Option<usize>)>,
    main_menu: Grid<Win>,
}

impl Win {
    pub fn new(canvas: &mut Canvas<Window>) -> Result<Self, String> {
        load_textures(canvas, Box::leak(Box::new(canvas.texture_creator())))?;

        let maps_count = fs::read_dir("assets/maps")
            .map_err(|e| e.to_string())?
            .count();
        if maps_count == 0
            || fs::read_dir("assets/maps")
                .map_err(|e| e.to_string())?
                .count()
                > 99
        {
            return Err("Too much or no levels".to_owned());
        }
        let levels_count = fs::read_dir("levels").map_err(|e| e.to_string())?.count();
        if levels_count == 0 || fs::read_dir("levels").map_err(|e| e.to_string())?.count() > 99 {
            return Err("Too much or no levels".to_owned());
        }

        Ok(Self {
            running: true,
            save: SaveFile::load()?,
            maps_count: maps_count as u8,
            levels_count: levels_count as u8,
            selected: None,
            main_menu: unsafe { Grid::empty() },
        })
    }

    pub const fn texts(&self) -> &'static Texts {
        self.save.texts()
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

    fn quit(&mut self, _: f32, _: f32, _: &mut Canvas<Window>) -> Result<(), String> {
        self.running = false;
        Ok(())
    }
}
impl GameWindow for Win {
    fn running(&mut self) -> bool {
        self.running
    }

    fn init(&mut self, _: &mut Canvas<Window>) -> Result<(), String> {
        self.main_menu = simple_grid!(
            self,
            Win,
            ColType::Ratio(100.),
            ColType::Ratio(1080.),
            ColType::Ratio(100.);
            RowType::Ratio(200.),
            RowType::Ratio(150.),
            RowType::Ratio(150.),
            RowType::Ratio(200.);
            Pos{x:1,y:1} => TextBox::new("id".to_owned(), &mut self.selected, &textures()?.font, "".to_owned()),
            Pos{x:1,y:2} => Button::new(&textures()?.font, Self::quit, |_self| _self.texts().quit),
        );
        Ok(())
    }

    fn init_frame(
        &mut self,
        canvas: &mut Canvas<Window>,
        width: f32,
        height: f32,
    ) -> Result<(), String> {
        self.main_menu
            .init_frame(canvas, FRect::new(0., 0., width, height))
    }

    fn event(&mut self, canvas: &mut Canvas<Window>, event: Event) -> Result<(), String> {
        match event {
            Event::Quit { .. } => self.running = false,
            Event::KeyDown {
                keycode: Some(Keycode::F11),
                ..
            } => {
                self.change_full_screen(0., 0., canvas)?;
            }
            _ => {}
        }
        self.main_menu.event(canvas, event)
    }

    fn update(&mut self, canvas: &mut Canvas<Window>, elapsed: Duration) -> Result<(), String> {
        self.main_menu.update(canvas, elapsed)
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.clear();
        self.main_menu.draw(canvas)
    }
}
