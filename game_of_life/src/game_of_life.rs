use anyhow::{anyhow, Result};
use red_sdl::{
    event::Event,
    refs::{MutRef, Ref},
    ui_element::grid::{ColType, Grid, Pos, RowType},
    user_control::{BWindow, GameWindow, UserControl}, zero,
};
use sdl2::{keyboard::Keycode, pixels::Color, rect::FRect, render::Canvas, video::Window};
use std::{collections::HashMap, time::Duration};

use crate::{case::Case, State, HEIGHT, WIDTH};

pub const SQUARE_SIZE: usize = 16;

pub struct GameOfLife {
    grid: Grid<GameOfLife, State, Case>,
    speed: f32,
    running: bool,
    surface: FRect,
    pub clock: f32,
}

impl GameOfLife {
    pub fn new(_: &mut Canvas<Window>, _: MutRef<State>) -> Result<Self> {
        Ok(Self {
            grid: Grid::new(
                (0..WIDTH).map(|_| ColType::Ratio(1.)).collect(),
                (0..HEIGHT).map(|_| RowType::Ratio(1.)).collect(),
                HashMap::from_iter((0..HEIGHT).flat_map(|y| {
                    (0..WIDTH).map(move |x| {
                        (
                            Pos { x, y },
                            Case {
                                value: false,
                                surface: FRect::new(0., 0., 0., 0.),
                            },
                        )
                    })
                })),
            ),
            speed: 0.,
            running: true,
            surface: zero(),
            clock: 0.,
        })
    }

    pub fn set_state(&mut self, state: bool, x: usize, y: usize) {
        self.grid
            .get_element_mut(x, y)
            .expect("cell state exception")
            .value = state;
    }

    pub fn toggle_pause(&mut self) {
        self.speed = match self.speed {
            ..1. => 1.,
            1.0.. => 0.,
            _ => 1.,
        }
    }

    pub fn next_step(&mut self) -> Result<()> {
        let new_playground: HashMap<Pos, bool> =
            HashMap::from_iter(self.grid.iter().map(|(pos, case)| (*pos, case.value)));
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let mut count: u32 = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx != 0 || dy != 0 {
                            if let Some(x) = x.checked_add_signed(dx) {
                                if let Some(y) = y.checked_add_signed(dy) {
                                    if x < WIDTH
                                        && y < HEIGHT
                                        && *new_playground
                                            .get(&Pos { x, y })
                                            .ok_or(anyhow!("next_step out of bound"))?
                                    {
                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                match count {
                    ..=1 | 4.. => {
                        self.set_state(false, x, y);
                    }
                    2 => {}
                    3 => {
                        self.set_state(true, x, y);
                    }
                };
            }
        }
        Ok(())
    }
}

impl BWindow<State> for GameOfLife {
    fn running(this: Ref<Self>, _: Ref<State>) -> bool {
        this.running
    }
}

impl GameWindow<State> for GameOfLife {
    fn time_scale(this: Ref<Self>, _: Ref<State>) -> f32 {
        this.speed
    }

    fn fps(_: Ref<Self>, _: Ref<State>) -> f32 {
        30.
    }
}

impl UserControl<(), State> for GameOfLife {
    fn surface(this: Ref<Self>, _: Ref<()>, _: Ref<State>) -> FRect {
        this.surface
    }

    fn event(
        mut this: MutRef<Self>,
        canvas: &Canvas<Window>,
        event: Event,
        _: MutRef<()>,
        state: MutRef<State>,
    ) -> Result<()> {
        match event {
            Event::ElementMove { .. } => {
                return Ok(());
            }
            Event::ElementResize { width, height } => {
                this.surface.set_width(width);
                this.surface.set_height(height);
            }
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => this.running = false,
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                repeat: false,
                ..
            } => {
                this.toggle_pause();
            }
            _ => {}
        }
        UserControl::event((&mut this.grid).into(), canvas, event, this, state)
    }

    fn update(
        mut this: MutRef<Self>,
        canvas: &Canvas<Window>,
        elapsed: Duration,
        _: MutRef<()>,
        state: MutRef<State>,
    ) -> Result<()> {
        this.clock += elapsed.as_secs_f32();
        if this.clock >= 1. {
            this.clock -= 1.;
            this.next_step()?;
        }
        UserControl::update((&mut this.grid).into(), canvas, elapsed, this, state)
    }

    fn draw(
        this: Ref<Self>,
        canvas: &mut Canvas<Window>,
        _: Ref<()>,
        state: Ref<State>,
    ) -> Result<()> {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        UserControl::draw((&this.grid).into(), canvas, this, state)
    }
}
