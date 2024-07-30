use std::time::Duration;

use sdl2::{
    event::Event,
    keyboard::Keycode,
    mouse::MouseButton,
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use sdl::game_window::GameWindow;

pub const SQUARE_SIZE: usize = 16;

#[derive(Copy, Clone)]
pub enum State {
    Paused,
    Playing,
}

fn dummy_texture<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<(Texture<'a>, Texture<'a>), String> {
    let mut square_texture1 = texture_creator
        .create_texture_target(None, SQUARE_SIZE as u32, SQUARE_SIZE as u32)
        .map_err(|e| e.to_string())?;
    let mut square_texture2 = texture_creator
        .create_texture_target(None, SQUARE_SIZE as u32, SQUARE_SIZE as u32)
        .map_err(|e| e.to_string())?;

    // let's change the textures we just created
    let textures = [
        (
            &mut square_texture1,
            (Color::RGB(255, 255, 0), Color::RGB(200, 200, 0)),
        ),
        (
            &mut square_texture2,
            (Color::RGB(192, 192, 192), Color::RGB(64, 64, 64)),
        ),
    ];
    canvas
        .with_multiple_texture_canvas(textures.iter(), |texture_canvas, &(c1, c2)| {
            for i in 0..SQUARE_SIZE as i32 {
                for j in 0..SQUARE_SIZE as i32 {
                    // drawing pixel by pixel isn't very effective, but we only do it once and store
                    // the texture afterwards so it's still alright!
                    if (i + j) % 7 == 0 {
                        // this doesn't mean anything, there was some trial and serror to find
                        // something that wasn't too ugly
                        texture_canvas.set_draw_color(c1);
                        texture_canvas
                            .draw_point(Point::new(i, j))
                            .expect("could not draw point");
                    }
                    if (i + j * 2) % 5 == 0 {
                        texture_canvas.set_draw_color(c2);
                        texture_canvas
                            .draw_point(Point::new(i, j))
                            .expect("could not draw point");
                    }
                }
            }
        })
        .map_err(|e| e.to_string())?;

    Ok((square_texture1, square_texture2))
}

pub struct GameOfLife<const WIDTH: usize, const HEIGHT: usize> {
    texture1: Texture<'static>,
    texture2: Texture<'static>,
    clock: u8,
    playground: [[bool; WIDTH]; HEIGHT],
    state: State,
    pressing: Option<bool>,
    running: bool,
}

impl<const WIDTH: usize, const HEIGHT: usize> GameOfLife<WIDTH, HEIGHT> {
    pub fn new(canvas: &mut Canvas<Window>) -> Result<Self, String> {
        // Create a "target" texture so that we can use our Renderer with it later
        let (texture1, texture2) =
            dummy_texture(canvas, Box::leak(Box::new(canvas.texture_creator())))?;

        Ok(Self {
            texture1,
            texture2,
            clock: 0,
            playground: [[false; WIDTH]; HEIGHT],
            state: State::Paused,
            pressing: None,
            running: true,
        })
    }

    pub const fn width(&self) -> usize {
        WIDTH
    }

    pub const fn height(&self) -> usize {
        HEIGHT
    }

    pub const fn get(&self, x: usize, y: usize) -> Option<bool> {
        if x < WIDTH && y < HEIGHT {
            Some(self.playground[y][x])
        } else {
            None
        }
    }

    pub fn set(
        &mut self,
        canvas: &mut Canvas<Window>,
        x: usize,
        y: usize,
        value: bool,
    ) -> Result<(), String> {
        if x < WIDTH && y < HEIGHT {
            self.playground[y][x] = value;
            if value {
                self.set_text(x, y, canvas)?;
            } else {
                Self::reset_text(x, y, canvas)?;
            }
        }
        Ok(())
    }

    fn reset_text(x: usize, y: usize, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(Rect::new(
            (x * SQUARE_SIZE) as i32,
            (y * SQUARE_SIZE) as i32,
            SQUARE_SIZE as u32,
            SQUARE_SIZE as u32,
        ))
    }

    fn set_text(&self, x: usize, y: usize, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.copy(
            if self.clock == 15 {
                &self.texture1
            } else {
                &self.texture2
            },
            None,
            Rect::new(
                (x * SQUARE_SIZE) as i32,
                (y * SQUARE_SIZE) as i32,
                SQUARE_SIZE as u32,
                SQUARE_SIZE as u32,
            ),
        )
    }

    pub fn toggle_state(&mut self) {
        self.state = match self.state {
            State::Paused => State::Playing,
            State::Playing => State::Paused,
        }
    }

    pub const fn state(&self) -> State {
        self.state
    }

    pub const fn playground(&self) -> &[[bool; WIDTH]; HEIGHT] {
        &self.playground
    }

    pub fn change_color(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let new_playground = self.playground;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let mut count: u32 = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx != 0 || dy != 0 {
                            if let Some(x) = x.checked_add_signed(dx) {
                                if let Some(y) = y.checked_add_signed(dy) {
                                    if x < WIDTH && y < HEIGHT && new_playground[y][x] {
                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                match count {
                    ..=1 | 4.. => {
                        self.playground[y][x] = false;
                        Self::reset_text(x, y, canvas)?;
                    }
                    2 => {}
                    3 => {
                        self.playground[y][x] = true;
                        self.set_text(x, y, canvas)?;
                    }
                };
            }
        }
        Ok(())
    }

    fn click_square(
        &mut self,
        canvas: &mut Canvas<Window>,
        x: usize,
        y: usize,
    ) -> Result<(), String> {
        let x = x / SQUARE_SIZE;
        let y = y / SQUARE_SIZE;
        if let Some(state) = self.pressing {
            self.set(canvas, x, y, state)?;
        }
        Ok(())
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> GameWindow for GameOfLife<WIDTH, HEIGHT> {
    fn running(&mut self) -> bool {
        self.running
    }

    fn update(&mut self, canvas: &mut Canvas<Window>, _: Duration) -> Result<(), String> {
        if let State::Paused = self.state {
            return Ok(());
        }

        self.clock += 1;
        if self.clock >= 30 {
            self.change_color(canvas)?;
            self.clock = 0;
        }
        if self.clock == 0 || self.clock == 15 {
            for (y, row) in self.playground.iter().enumerate() {
                for (x, state) in row.iter().enumerate() {
                    if *state {
                        canvas.copy(
                            if self.clock == 15 {
                                &self.texture1
                            } else {
                                &self.texture2
                            },
                            None,
                            Rect::new(
                                (x * SQUARE_SIZE) as i32,
                                (y * SQUARE_SIZE) as i32,
                                SQUARE_SIZE as u32,
                                SQUARE_SIZE as u32,
                            ),
                        )?;
                    }
                }
            }
        }
        Ok(())
    }

    fn event(&mut self, canvas: &mut Canvas<Window>, event: Event) -> Result<(), String> {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => self.running = false,
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                repeat: false,
                ..
            } => {
                self.toggle_state();
            }
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                if x >= 0 && y >= 0 {
                    self.pressing = Some(true);
                    self.click_square(canvas, x as usize, y as usize)?;
                }
            }
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Right,
                x,
                y,
                ..
            } => {
                if x >= 0 && y >= 0 {
                    self.pressing = Some(false);
                    self.click_square(canvas, x as usize, y as usize)?;
                }
            }
            Event::MouseButtonUp { .. } => {
                self.pressing = None;
            }
            Event::MouseMotion { x, y, .. } => {
                if x >= 0 && y >= 0 {
                    self.click_square(canvas, x as usize, y as usize)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn draw(&self, _: &mut Canvas<Window>) -> Result<(), String> {
        Ok(())
    }
}
