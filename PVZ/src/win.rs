use sdl::game_window::GameWindow;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window};

use crate::{
    level::{DefaultConfig, ILevel, Level},
    plant::Plant1,
    textures::load_textures,
    zombie::Zombie1,
};

pub struct Win {
    running: bool,

    level: Box<dyn ILevel>,
}

impl Win {
    pub fn new(canvas: &mut Canvas<Window>) -> Result<Self, String> {
        load_textures(Box::leak(Box::new(canvas.texture_creator())))?;

        Ok(Self {
            running: true,
            level: Box::new(Level {
                showing_zombies: true,
                plants: [
                    [
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                    ],
                    [
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                    ],
                    [
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                    ],
                    [
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                    ],
                    [
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                        Some(Box::new(Plant1 {})),
                    ],
                ],
                zombies: [
                    vec![Box::new(Zombie1 { pos: 0. })],
                    vec![Box::new(Zombie1 { pos: 0. })],
                    vec![Box::new(Zombie1 { pos: 0. })],
                    vec![Box::new(Zombie1 { pos: 0. })],
                    vec![Box::new(Zombie1 { pos: 0. })],
                ],
                config: Box::new(DefaultConfig {
                    probs: vec![(Box::new(Zombie1 { pos: 0. }), 0.006)],
                }),
            }),
        })
    }
}

impl GameWindow for Win {
    fn running(&mut self) -> bool {
        self.running
    }

    fn update(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        self.level.update(canvas)
    }

    fn event(&mut self, canvas: &mut Canvas<Window>, event: Event) -> Result<(), String> {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => self.running = false,
            _ => {}
        }
        self.level.event(canvas, event)
    }

    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        self.level.draw(canvas)
    }
}
