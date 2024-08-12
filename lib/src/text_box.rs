use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    os::raw::c_int,
    time::Duration,
};

use crate::{draw_string, event::Event, grid::GridChildren};
use sdl2::{
    keyboard::Keycode,
    mouse::MouseButton,
    pixels::Color,
    rect::{FPoint, FRect},
    render::Canvas,
    sys::{SDL_GetClipboardText, SDL_HasClipboardText, SDL_SetClipboardText, SDL_bool},
    ttf::Font,
    video::Window,
};

const MAXLEN: usize = 100;

pub struct TextBox<T> {
    parent: PhantomData<T>,
    id: String,
    selected: *mut Option<(String, usize, Option<usize>)>,
    font: &'static Font<'static, 'static>,
    surface: FRect,
    text: String,
    shift: bool,
    ctrl: bool,
}
impl<T> TextBox<T> {
    pub fn new(
        id: String,
        selected: *mut Option<(String, usize, Option<usize>)>,
        font: &'static Font<'static, 'static>,
        text: String,
    ) -> Self {
        Self {
            parent: PhantomData,
            id,
            selected,
            font,
            surface: FRect::new(0., 0., 0., 0.),
            text,
            shift: false,
            ctrl: false,
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

    fn index_to_position(&self, index: usize) -> f32 {
        if index == 0 {
            return 0.;
        }
        self.font
            .size_of(&self.text[..index])
            .expect("font error")
            .0 as f32
            / self.font.size_of(&self.text).expect("font error").0 as f32
    }

    fn position_to_index(&self, mut pos: f32) -> usize {
        if self.text.is_empty() {
            0
        } else {
            let scale =
                self.surface.width() / self.font.size_of(&self.text).expect("font error").0 as f32;
            pos *= self.surface.width();
            for (i, c) in self.text.chars().enumerate() {
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

    fn delete_selection(&mut self, index: &mut usize, to_index: usize) {
        if *index < to_index {
            self.text.drain(*index..to_index);
            self.select(*index, None);
        } else {
            self.text.drain(to_index..*index);
            self.select(to_index, None);
            *index = to_index
        }
    }

    fn copy(&self, index: usize, to_index: usize) -> Result<(), String> {
        let text = self.text[index.min(to_index)..index.max(to_index)].to_string();
        let text = CString::new(text).map_err(|e| e.to_string())?;
        if unsafe { SDL_SetClipboardText(text.as_ptr()) != c_int::from(0) } {
            return Err("Error set clipboard text_board".to_owned());
        }
        Ok(())
    }
}
impl<T> GridChildren<T> for TextBox<T> {
    fn grid_init(&mut self, _: &mut Canvas<Window>, _: &mut T) -> Result<(), String> {
        Ok(())
    }

    fn grid_init_frame(
        &mut self,
        _: &mut Canvas<Window>,
        surface: FRect,
        _: &mut T,
    ) -> Result<(), String> {
        self.surface = surface;
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    fn grid_event(
        &mut self,
        _: &mut Canvas<Window>,
        event: Event,
        _: &mut T,
    ) -> Result<(), String> {
        match event {
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                ..
            } => {
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
            Event::MouseMotion { mousestate, x, .. } if mousestate.left() => {
                if let Some((index, _)) = self.is_selected() {
                    self.select(
                        index,
                        Some(self.position_to_index((x - self.surface.x()) / self.surface.width())),
                    );
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::LShift),
                scancode: Some(_),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::RShift),
                scancode: Some(_),
                ..
            } => {
                self.shift = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::LShift),
                scancode: Some(_),
                ..
            }
            | Event::KeyUp {
                keycode: Some(Keycode::RShift),
                scancode: Some(_),
                ..
            } => {
                self.shift = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::LCtrl),
                scancode: Some(_),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::RCtrl),
                scancode: Some(_),
                ..
            } => {
                self.ctrl = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::LCtrl),
                scancode: Some(_),
                ..
            }
            | Event::KeyUp {
                keycode: Some(Keycode::RCtrl),
                scancode: Some(_),
                ..
            } => {
                self.ctrl = false;
            }
            Event::KeyDown {
                keycode: Some(keycode),
                scancode: Some(scancode),
                ..
            } => {
                if let Some((mut index, to_index)) = self.is_selected() {
                    match keycode {
                        Keycode::Backspace => {
                            if let Some(to_index) = to_index {
                                self.delete_selection(&mut index, to_index);
                            } else if index > 0 {
                                self.text.remove(index - 1);
                                self.select(index - 1, None);
                            }
                        }
                        Keycode::Delete => {
                            if let Some(to_index) = to_index {
                                self.delete_selection(&mut index, to_index);
                            } else if index < self.text.len() {
                                self.text.remove(index);
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
                                self.delete_selection(&mut index, to_index);
                            }
                            if self.text.len() < MAXLEN {
                                self.text.insert(index, ' ');
                                self.select(index + 1, None);
                            }
                        }
                        Keycode::V if self.ctrl => {
                            if let Some(to_index) = to_index {
                                self.delete_selection(&mut index, to_index);
                            }
                            if self.text.len() < MAXLEN
                                && unsafe { SDL_HasClipboardText() == SDL_bool::SDL_TRUE }
                            {
                                let text = unsafe { CStr::from_ptr(SDL_GetClipboardText()) }
                                    .to_str()
                                    .map_err(|e| e.to_string())?
                                    .to_owned();
                                let text = &text[..text.len().min(MAXLEN - self.text.len())];
                                self.text.insert_str(index, text);
                                self.select(index + text.len(), None);
                            }
                        }
                        Keycode::C if self.ctrl => {
                            if let Some(to_index) = to_index {
                                if index != to_index {
                                    self.copy(index, to_index)?;
                                }
                            }
                        }
                        Keycode::X if self.ctrl => {
                            if let Some(to_index) = to_index {
                                if index != to_index {
                                    self.copy(index, to_index)?;
                                    self.delete_selection(&mut index, to_index);
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
                                self.delete_selection(&mut index, to_index);
                            }
                            let mut text = scancode.to_string();
                            if self.text.len() < MAXLEN {
                                if self.shift {
                                    text = text.to_uppercase();
                                } else {
                                    text = text.to_lowercase();
                                }
                                let text = &text[..text.len().min(MAXLEN - self.text.len())];
                                self.text.insert_str(index, text);
                                self.select(index + text.len(), None);
                            }
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
        _: &mut T,
    ) -> Result<(), String> {
        Ok(())
    }

    fn grid_draw(&self, canvas: &mut Canvas<Window>, _: &T) -> Result<(), String> {
        canvas.set_draw_color(Color::GRAY);
        canvas.draw_frect(self.surface)?;
        if !self.text.is_empty() {
            draw_string(canvas, self.font, self.surface, self.text.as_str())?;
        }
        if let Some((index, to_index)) = self.is_selected() {
            if let Some(to_index) = to_index {
                canvas.set_draw_color(Color::RGBA(255, 255, 255, 100));
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
                canvas.set_draw_color(Color::WHITE);
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
