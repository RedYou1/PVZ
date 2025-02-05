use std::time::Duration;

use anyhow::{anyhow, Result};
use red_sdl::{
    event::Event,
    refs::{MutRef, Ref},
    user_control::UserControl,
};
use sdl2::{mouse::MouseButton, rect::FRect, render::Canvas, video::Window};

use crate::{game_of_life::GameOfLife, State};

pub struct Case {
    pub value: bool,
    pub surface: FRect,
}

impl UserControl<GameOfLife, State> for Case {
    fn surface(this: Ref<Self>, _: Ref<GameOfLife>, _: Ref<State>) -> FRect {
        this.surface
    }

    fn event(
        mut this: MutRef<Self>,
        _: &Canvas<Window>,
        event: Event,
        _: MutRef<GameOfLife>,
        _: MutRef<State>,
    ) -> Result<()> {
        match event {
            Event::ElementMove { x, y } => {
                this.surface.set_x(x);
                this.surface.set_y(y);
            }
            Event::ElementResize { width, height } => {
                this.surface.set_width(width);
                this.surface.set_height(height);
            }
            _ => {}
        }
        if event.hover(this.surface) {
            match event {
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    this.value = true;
                }
                Event::MouseMotion { mousestate, .. } if mousestate.left() => {
                    this.value = true;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Right,
                    ..
                } => {
                    this.value = false;
                }
                Event::MouseMotion { mousestate, .. } if mousestate.right() => {
                    this.value = false;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn update(
        _: MutRef<Self>,
        _: &Canvas<Window>,
        _: Duration,
        _: MutRef<GameOfLife>,
        _: MutRef<State>,
    ) -> Result<()> {
        Ok(())
    }

    fn draw(
        this: Ref<Self>,
        canvas: &mut Canvas<Window>,
        parent: Ref<GameOfLife>,
        state: Ref<State>,
    ) -> Result<()> {
        if this.value {
            canvas
                .copy_f(
                    if parent.clock >= 0.5 {
                        &state.texture1
                    } else {
                        &state.texture2
                    },
                    None,
                    this.surface,
                )
                .map_err(|e| anyhow!(e))?;
        }
        Ok(())
    }
}
