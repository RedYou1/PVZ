use std::{marker::PhantomData, time::Duration};

use crate::{
    event::Event,
    functions::{FnColor, FnState, StateEnum},
    grid::GridChildren,
    missing::{
        clipboard::{get_clipboard_text, set_clipboard_text},
        ui_string::{draw_string, UIString},
    },
};
use sdl2::{
    keyboard::Keycode,
    mouse::MouseButton,
    rect::{FPoint, FRect},
    render::Canvas,
    ttf::Font,
    video::Window,
};

pub struct TextBox<Parent> {
    parent: PhantomData<Parent>,
    id: String,
    selected: *mut Option<(String, usize, Option<usize>)>,
    font: &'static Font<'static, 'static>,
    surface: FRect,
    text: UIString,
    shift: bool,
    ctrl: bool,
    state: FnState<Parent, Self>,
    select_color: FnColor<Parent, Self>,
    line_color: FnColor<Parent, Self>,
    front_color: FnColor<Parent, Self>,
    back_color: FnColor<Parent, Self>,
}
impl<Parent> TextBox<Parent> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: String,
        selected: *mut Option<(String, usize, Option<usize>)>,
        font: &'static Font<'static, 'static>,
        text: Option<UIString>,
        state: FnState<Parent, Self>,
        select_color: FnColor<Parent, Self>,
        line_color: FnColor<Parent, Self>,
        front_color: FnColor<Parent, Self>,
        back_color: FnColor<Parent, Self>,
    ) -> Self {
        Self {
            parent: PhantomData,
            id,
            selected,
            font,
            surface: FRect::new(0., 0., 0., 0.),
            text: text.unwrap_or(UIString::empty(font)),
            shift: false,
            ctrl: false,
            state,
            select_color,
            line_color,
            front_color,
            back_color,
        }
    }

    pub fn is_selected(&self) -> Option<(usize, Option<usize>)> {
        if let Some((id, index, to_index)) = unsafe { self.selected.as_ref()? }.as_ref() {
            if self.id.eq(id) {
                Some((*index, *to_index))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn select(&mut self, index: usize, to_index: Option<usize>) {
        *unsafe {
            self.selected
                .as_mut()
                .expect("unwrap ptr text_box is_selected")
        } = Some((self.id.clone(), index, to_index));
    }

    pub fn unselect(&mut self) {
        *unsafe {
            self.selected
                .as_mut()
                .expect("unwrap ptr text_box is_selected")
        } = None;
    }

    fn index_to_position(&self, index: usize) -> f32 {
        if index == 0 {
            return 0.;
        }
        self.font
            .size_of(&self.text.as_str()[..index])
            .expect("font error")
            .0 as f32
            / self.font.size_of(self.text.as_str()).expect("font error").0 as f32
    }

    fn position_to_index(&self, mut pos: f32) -> usize {
        if self.text.is_empty() {
            0
        } else {
            let scale = self.surface.width()
                / self.font.size_of(self.text.as_ref()).expect("font error").0 as f32;
            pos *= self.surface.width();
            for (i, c) in self.text.as_str().chars().enumerate() {
                let w = self.font.size_of_char(c).expect("font error").0 as f32 * scale;
                if w > pos {
                    if w / 2. > pos {
                        return i;
                    } else {
                        return i + 1;
                    }
                }
                pos -= w;
            }
            self.text.len()
        }
    }

    fn delete_selection(&mut self, index: &mut usize, to_index: usize) -> Result<(), String> {
        if *index < to_index {
            if self.text.drain(*index, to_index - *index)?.is_some() {
                self.select(*index, None);
            }
        } else if self.text.drain(to_index, *index - to_index)?.is_some() {
            self.select(to_index, None);
            *index = to_index
        }
        Ok(())
    }
}
impl<Parent> GridChildren<Parent> for TextBox<Parent> {
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

    #[allow(clippy::too_many_lines)]
    fn grid_event(
        &mut self,
        _: &mut Canvas<Window>,
        event: Event,
        parent: &mut Parent,
    ) -> Result<(), String> {
        if (self.state)(parent, self) != StateEnum::Enable {
            return Ok(());
        }
        match (event.hover(self.surface), event) {
            (
                true,
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    ..
                },
            ) => {
                let selected = self.is_selected();
                if self.shift && selected.is_some() {
                    let (index, _) = selected.ok_or("Checked")?;
                    self.select(
                        index,
                        Some(self.position_to_index((x - self.surface.x()) / self.surface.width())),
                    );
                } else {
                    self.select(
                        self.position_to_index((x - self.surface.x()) / self.surface.width()),
                        None,
                    );
                }
            }
            (false, Event::MouseButtonDown { .. }) => {
                if self.is_selected().is_some() {
                    self.unselect()
                }
            }
            (true, Event::MouseMotion { mousestate, x, .. }) if mousestate.left() => {
                if let Some((index, _)) = self.is_selected() {
                    self.select(
                        index,
                        Some(self.position_to_index((x - self.surface.x()) / self.surface.width())),
                    );
                }
            }
            (
                _,
                Event::KeyDown {
                    keycode: Some(Keycode::LShift),
                    scancode: Some(_),
                    ..
                },
            )
            | (
                _,
                Event::KeyDown {
                    keycode: Some(Keycode::RShift),
                    scancode: Some(_),
                    ..
                },
            ) => {
                self.shift = true;
            }
            (
                _,
                Event::KeyUp {
                    keycode: Some(Keycode::LShift),
                    scancode: Some(_),
                    ..
                },
            )
            | (
                _,
                Event::KeyUp {
                    keycode: Some(Keycode::RShift),
                    scancode: Some(_),
                    ..
                },
            ) => {
                self.shift = false;
            }
            (
                _,
                Event::KeyDown {
                    keycode: Some(Keycode::LCtrl),
                    scancode: Some(_),
                    ..
                },
            )
            | (
                _,
                Event::KeyDown {
                    keycode: Some(Keycode::RCtrl),
                    scancode: Some(_),
                    ..
                },
            ) => {
                self.ctrl = true;
            }
            (
                _,
                Event::KeyUp {
                    keycode: Some(Keycode::LCtrl),
                    scancode: Some(_),
                    ..
                },
            )
            | (
                _,
                Event::KeyUp {
                    keycode: Some(Keycode::RCtrl),
                    scancode: Some(_),
                    ..
                },
            ) => {
                self.ctrl = false;
            }
            (
                _,
                Event::KeyDown {
                    keycode: Some(keycode),
                    scancode: Some(scancode),
                    ..
                },
            ) => {
                if let Some((mut index, to_index)) = self.is_selected() {
                    match keycode {
                        Keycode::Backspace => {
                            if let Some(to_index) = to_index {
                                self.delete_selection(&mut index, to_index)?;
                            } else if index > 0 && self.text.remove(index - 1)?.is_some() {
                                self.select(index - 1, None);
                            }
                        }
                        Keycode::Delete => {
                            if let Some(to_index) = to_index {
                                self.delete_selection(&mut index, to_index)?;
                            } else if index < self.text.len() && self.text.remove(index)?.is_some()
                            {
                                self.select(index, None);
                            }
                        }
                        Keycode::Left => {
                            if let Some(to_index) = to_index {
                                if self.shift {
                                    if to_index > 0 {
                                        if index == to_index - 1 {
                                            self.select(index, None);
                                        } else {
                                            self.select(index, Some(to_index - 1));
                                        }
                                    }
                                } else {
                                    self.select(index.min(to_index), None);
                                }
                            } else if index == 0 {
                            } else if self.shift {
                                self.select(index, Some(index - 1));
                            } else {
                                self.select(index - 1, None);
                            }
                        }
                        Keycode::Right => {
                            if let Some(to_index) = to_index {
                                if self.shift {
                                    if to_index < self.text.len() {
                                        if index == to_index + 1 {
                                            self.select(index, None);
                                        } else {
                                            self.select(index, Some(to_index + 1));
                                        }
                                    }
                                } else {
                                    self.select(index.max(to_index), None);
                                }
                            } else if index == self.text.len() {
                            } else if self.shift {
                                self.select(index, Some(index + 1));
                            } else {
                                self.select(index + 1, None);
                            }
                        }
                        Keycode::Space => {
                            if let Some(to_index) = to_index {
                                self.delete_selection(&mut index, to_index)?;
                            }
                            if self.text.insert(index, ' ')? {
                                self.select(index + 1, None);
                            }
                        }
                        Keycode::V if self.ctrl => {
                            if let Some(to_index) = to_index {
                                self.delete_selection(&mut index, to_index)?;
                            }
                            if let Some(text) = get_clipboard_text() {
                                let text = text?;
                                let tlen = self.text.insert_str(index, text.as_str())?;
                                self.select(index + tlen, None);
                            }
                        }
                        Keycode::C if self.ctrl => {
                            if let Some(to_index) = to_index {
                                if index != to_index {
                                    set_clipboard_text(
                                        &self.text.as_str()
                                            [index.min(to_index)..index.max(to_index)],
                                    )?;
                                }
                            }
                        }
                        Keycode::X if self.ctrl => {
                            if let Some(to_index) = to_index {
                                if index != to_index {
                                    set_clipboard_text(
                                        &self.text.as_str()
                                            [index.min(to_index)..index.max(to_index)],
                                    )?;
                                    self.delete_selection(&mut index, to_index)?;
                                }
                            }
                        }
                        Keycode::A if self.ctrl => {
                            if self.is_selected().is_some() {
                                self.select(0, Some(self.text.len()));
                            }
                        }
                        _ if self.ctrl => {}
                        _ => {
                            if let Some(to_index) = to_index {
                                self.delete_selection(&mut index, to_index)?;
                            }
                            let mut text = scancode.to_string();
                            if self.shift {
                                text = text.to_uppercase();
                            } else {
                                text = text.to_lowercase();
                            }
                            let tlen = self.text.insert_str(index, text.as_str())?;
                            self.select(index + tlen, None);
                        }
                    }
                }
            }
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
        let front_color = (self.front_color)(parent, self);
        canvas.set_draw_color(front_color);
        canvas.draw_frect(self.surface)?;
        if !self.text.is_empty() {
            draw_string(
                canvas,
                self.font,
                None,
                self.surface,
                &self.text,
                front_color,
            )?;
        }
        if let Some((index, to_index)) = self.is_selected() {
            if let Some(to_index) = to_index {
                canvas.set_draw_color((self.select_color)(parent, self));
                let pos1 = self.surface.width() * self.index_to_position(index) + self.surface.x();
                let pos2 =
                    self.surface.width() * self.index_to_position(to_index) + self.surface.x();
                canvas.fill_frect(FRect::new(
                    pos1.min(pos2),
                    self.surface.y(),
                    pos1.max(pos2) - pos1.min(pos2),
                    self.surface.height(),
                ))?;
            } else {
                canvas.set_draw_color((self.line_color)(parent, self));
                canvas.draw_fline(
                    FPoint::new(
                        self.surface.width() * self.index_to_position(index) + self.surface.x(),
                        self.surface.y(),
                    ),
                    FPoint::new(
                        self.surface.width() * self.index_to_position(index) + self.surface.x(),
                        self.surface.y() + self.surface.height(),
                    ),
                )?;
            }
        }
        Ok(())
    }
}
