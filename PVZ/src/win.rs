use std::{fs, time::Duration};

use sdl::game_window::GameWindow;
use sdl2::{
    event::Event,
    gfx::primitives::DrawRenderer,
    keyboard::Keycode,
    mouse::MouseButton,
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::{FullscreenType, Window},
};

use crate::{level::Level, textures::load_textures};

pub struct Win {
    running: bool,
    pause: bool,

    levels_count: u8,
    level: Option<Level>,
}

impl Win {
    pub fn new(canvas: &mut Canvas<Window>) -> Result<Self, String> {
        load_textures(Box::leak(Box::new(canvas.texture_creator())))?;
        let levels_count = fs::read_dir("levels").map_err(|e| e.to_string())?.count();
        if levels_count == 0 || fs::read_dir("levels").map_err(|e| e.to_string())?.count() > 99 {
            return Err("Too much or no levels".to_owned());
        }

        Ok(Self {
            running: true,
            pause: false,
            levels_count: levels_count as u8,
            level: None,
        })
    }
}

impl GameWindow for Win {
    fn running(&mut self) -> bool {
        self.running
    }

    fn update(&mut self, canvas: &mut Canvas<Window>, elapsed: Duration) -> Result<(), String> {
        if self.pause {
            return Ok(());
        }
        if let Some(level) = self.level.as_mut() {
            level.update(canvas, elapsed)?;
        }
        Ok(())
    }

    fn event(&mut self, canvas: &mut Canvas<Window>, event: Event) -> Result<(), String> {
        let (width, height) = canvas.output_size()?;
        let scale_x = |x: i32| x as f32 * 1280. / width as f32;
        let scale_y = |y: i32| y as f32 * 720. / height as f32;

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
                let window = canvas.window_mut();
                window.set_fullscreen(if window.fullscreen_state() == FullscreenType::Off {
                    FullscreenType::Desktop
                } else {
                    FullscreenType::Off
                })?;
            }
            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                let x = scale_x(x) as i32;
                let y = scale_y(y) as i32;
                if self.level.is_some() {
                    if self.pause && (565..=715).contains(&x) {
                        if (200..=240).contains(&y) {
                            let window = canvas.window_mut();
                            window.set_fullscreen(
                                if window.fullscreen_state() == FullscreenType::Off {
                                    FullscreenType::Desktop
                                } else {
                                    FullscreenType::Off
                                },
                            )?;
                        } else if (260..=300).contains(&y) {
                            self.level = None;
                            self.pause = false;
                        } else if (320..=380).contains(&y) {
                            self.running = false;
                        }
                    }
                } else if (485..=635).contains(&x) {
                    if (200..=240).contains(&y) {
                        let window = canvas.window_mut();
                        window.set_fullscreen(
                            if window.fullscreen_state() == FullscreenType::Off {
                                FullscreenType::Desktop
                            } else {
                                FullscreenType::Off
                            },
                        )?;
                    } else if (260..=300).contains(&y) {
                        self.running = false;
                    }
                } else if (645..=796).contains(&x) && y >= 200 {
                    let y = (y as f32 - 200.) / 60.;
                    if (y - y.floor()) <= 40. / 60. {
                        let y = y.floor() as u8;
                        if (0..self.levels_count).contains(&y) {
                            self.level = Some(
                                Level::load_config(format!("levels/{y}.data").as_str())
                                    .map_err(|e| e.to_string())?,
                            );
                        }
                    }
                }
            }
            _ => {}
        }
        if let Some(level) = self.level.as_mut() {
            level.event(canvas, event.clone())?;
        }
        Ok(())
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        if let Some(level) = self.level.as_ref() {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            set_scale(canvas, 1., 1.)?;

            level.draw(canvas)?;
            if self.pause {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.fill_rect(Rect::new(565, 200, 150, 40))?;
                canvas.fill_rect(Rect::new(565, 260, 150, 40))?;
                canvas.fill_rect(Rect::new(565, 320, 150, 40))?;

                const SCALE: i16 = 4;
                set_scale(canvas, SCALE as f32, SCALE as f32)?;
                canvas.string(574 / SCALE, 206 / SCALE, "FULL", Color::RGB(255, 255, 255))?;
                canvas.string(574 / SCALE, 266 / SCALE, "RETN", Color::RGB(255, 255, 255))?;
                canvas.string(574 / SCALE, 326 / SCALE, "QUIT", Color::RGB(255, 255, 255))?;
                set_scale(canvas, 1., 1.)?;
            }
            return Ok(());
        }
        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.clear();
        set_scale(canvas, 1., 1.)?;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(Rect::new(485, 200, 150, 40))?;
        canvas.fill_rect(Rect::new(485, 260, 150, 40))?;
        for i in 0..self.levels_count {
            canvas.fill_rect(Rect::new(645, 200 + i as i32 * 60, 150, 40))?;
        }

        const SCALE: i16 = 4;
        set_scale(canvas, SCALE as f32, SCALE as f32)?;
        canvas.string(494 / SCALE, 206 / SCALE, "FULL", Color::RGB(255, 255, 255))?;
        canvas.string(494 / SCALE, 266 / SCALE, "QUIT", Color::RGB(255, 255, 255))?;
        for i in 0..self.levels_count {
            canvas.string(
                654 / SCALE,
                (206 + i as i16 * 60) / SCALE,
                format!(" {}{} ", (i + 1) / 10, (i + 1) % 10).as_str(),
                Color::RGB(255, 255, 255),
            )?;
        }
        set_scale(canvas, 1., 1.)?;
        Ok(())
    }
}

pub fn set_scale(canvas: &mut Canvas<Window>, scale_x: f32, scale_y: f32) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;
    canvas.set_scale(
        scale_x * width as f32 / 1280.,
        scale_y * height as f32 / 720.,
    )
}
