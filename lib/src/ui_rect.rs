use std::{marker::PhantomData, time::Duration};

use crate::{
    event::Event,
    functions::{FnAction, FnColor, FnState, FnText, StateEnum},
    grid::GridChildren,
    missing::ui_string::{draw_string, UIString},
};
use sdl2::{mouse::MouseButton, rect::FRect, render::Canvas, ttf::Font, video::Window};

pub struct UIRect<Parent> {
    parent: PhantomData<Parent>,
    font: &'static Font<'static, 'static>,
    action: Option<FnAction<Parent, Self>>,
    surface: FRect,
    text: Option<FnText<Parent, Self>>,
    state: FnState<Parent, Self>,
    back_color: FnColor<Parent, Self>,
    hover: bool,
}
impl<Parent> UIRect<Parent> {
    pub fn new(
        font: &'static Font<'static, 'static>,
        state: FnState<Parent, Self>,
        back_color: FnColor<Parent, Self>,
    ) -> Self {
        Self {
            parent: PhantomData,
            font,
            action: None,
            surface: FRect::new(0., 0., 0., 0.),
            text: None,
            state,
            back_color,
            hover: false,
        }
    }

    pub fn action(mut self, action: FnAction<Parent, Self>) -> Self {
        self.action = Some(action);
        self
    }

    pub fn text(mut self, text: FnText<Parent, Self>) -> Self {
        self.text = Some(text);
        self
    }

    pub const fn hover(&self) -> bool {
        self.hover
    }

    pub fn get_text(&self, parent: &Parent) -> Result<Option<UIString>, String> {
        if let Some(text) = self.text.as_ref() {
            if let (Some(text), _) = text(parent, self)? {
                return Ok(Some(text));
            }
        }
        Ok(None)
    }
}
impl<Parent> GridChildren<Parent> for UIRect<Parent> {
    fn grid_init(&mut self, _: &mut Canvas<Window>, _: &mut Parent) -> Result<(), String> {
        Ok(())
    }

    fn grid_init_frame(
        &mut self,
        _: &mut Canvas<Window>,
        surface: FRect,
        _: &mut Parent,
    ) -> Result<(), String> {
        self.surface = surface;
        Ok(())
    }

    fn grid_event(
        &mut self,
        canvas: &mut Canvas<Window>,
        event: Event,
        parent: &mut Parent,
    ) -> Result<(), String> {
        if (self.state)(parent, self) != StateEnum::Enable {
            return Ok(());
        }
        let _self = self as *mut Self;
        match event.hover(self.surface) {
            Ok(Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            }) => {
                if let Some(action) = self.action.as_mut() {
                    (action)(
                        parent,
                        unsafe { _self.as_mut().ok_or("unwrap ptr ui_rect event")? },
                        x,
                        y,
                        canvas,
                    )?;
                }
            }
            Ok(Event::MouseMotion { .. }) => {
                self.hover = true;
            }
            Err(_) => self.hover = false,
            _ => {}
        }
        Ok(())
    }

    fn grid_update(
        &mut self,
        _: &mut Canvas<Window>,
        _: Duration,
        _: &mut Parent,
    ) -> Result<(), String> {
        Ok(())
    }

    fn grid_draw(&self, canvas: &mut Canvas<Window>, parent: &Parent) -> Result<(), String> {
        if (self.state)(parent, self) == StateEnum::Hidden {
            return Ok(());
        }
        canvas.set_draw_color((self.back_color)(parent, self));
        canvas.fill_frect(self.surface)?;
        if let Some(text) = self.text.as_ref() {
            if let (Some(text), color) = text(parent, self)? {
                draw_string(canvas, self.font, None, self.surface, &text, color)?;
            }
        }
        Ok(())
    }
}
