use red_sdl::{
    event::Event,
    game_window::GameWindow,
    grid::{ColType, Grid, GridChildren, Pos, RowType},
    user_control::UserControl,
};
use sdl2::{
    keyboard::Keycode,
    pixels::Color,
    rect::{FRect, Point},
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};
use std::{collections::HashMap, ptr::addr_of_mut, time::Duration};

use crate::{case::Case, clock, next_clock, reset_clock, set_state, states, HEIGHT, WIDTH};

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
    let mut success = Ok(());
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
                        success = texture_canvas.draw_point(Point::new(i, j));
                        if success.is_err() {
                            return;
                        }
                    }
                    if (i + j * 2) % 5 == 0 {
                        texture_canvas.set_draw_color(c2);
                        success = texture_canvas.draw_point(Point::new(i, j));
                        if success.is_err() {
                            return;
                        }
                    }
                }
            }
        })
        .map_err(|e| e.to_string())?;
    success?;

    Ok((square_texture1, square_texture2))
}

pub struct GameOfLife {
    grid: Grid<()>,
    state: State,
    running: bool,
}

static mut STATE: () = ();

impl GameOfLife {
    pub fn new(canvas: &mut Canvas<Window>) -> Result<Self, String> {
        // Create a "target" texture so that we can use our Renderer with it later
        let (texture1, texture2) =
            dummy_texture(canvas, Box::leak(Box::new(canvas.texture_creator())))?;
        let texture1: &'static Texture<'static> = Box::leak(Box::new(texture1));
        let texture2: &'static Texture<'static> = Box::leak(Box::new(texture2));
        Ok(Self {
            grid: Grid::new(
                unsafe { addr_of_mut!(STATE) },
                (0..WIDTH).map(|_| ColType::Ratio(1.)).collect(),
                (0..HEIGHT).map(|_| RowType::Ratio(1.)).collect(),
                HashMap::from_iter((0..HEIGHT).flat_map(|y| {
                    (0..WIDTH).map(move |x| {
                        (
                            Pos { x, y },
                            Box::new(Case {
                                x,
                                y,
                                texture1,
                                texture2,
                                surface: FRect::new(0., 0., 0., 0.),
                            }) as Box<dyn GridChildren<()>>,
                        )
                    })
                })),
            ),
            state: State::Paused,
            running: true,
        })
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

    pub fn next_step(&mut self) -> Result<(), String> {
        let new_playground = *states();
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
                        set_state(false, x, y);
                    }
                    2 => {}
                    3 => {
                        set_state(true, x, y);
                    }
                };
            }
        }
        Ok(())
    }
}

impl GameWindow for GameOfLife {
    fn running(&mut self) -> bool {
        self.running
    }

    fn init(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        self.grid.init(canvas)
    }

    fn init_frame(
        &mut self,
        canvas: &mut Canvas<Window>,
        width: f32,
        height: f32,
    ) -> Result<(), String> {
        self.grid
            .init_frame(canvas, FRect::new(0., 0., width, height))
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
            _ => {}
        }
        self.grid.event(canvas, event)
    }

    fn update(&mut self, canvas: &mut Canvas<Window>, elapsed: Duration) -> Result<(), String> {
        if let State::Paused = self.state {
            return Ok(());
        }

        next_clock();
        if clock() >= 30 {
            self.next_step()?;
            reset_clock();
        }
        self.grid.update(canvas, elapsed)
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        self.grid.draw(canvas)
    }
}
