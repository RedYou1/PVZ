use sdl2::{
    pixels::Color,
    render::{Canvas, Texture},
    video::Window,
};

use crate::missing::ui_string::UIString;

pub type FnAction<Parent, Element> =
    Box<dyn FnMut(&mut Parent, &Element, f32, f32, &mut Canvas<Window>) -> Result<(), String>>;
pub type FnText<Parent, Element> =
    Box<dyn Fn(&Parent, &Element) -> Result<(Option<UIString>, Color), String>>;
#[derive(Debug, PartialEq, Eq)]
pub enum StateEnum {
    Hidden,
    Showing,
    Enable,
}
pub type FnState<Parent, Element> = Box<dyn Fn(&Parent, &Element) -> StateEnum>;
pub type FnColor<Parent, Element> = Box<dyn Fn(&Parent, &Element) -> Color>;
pub type FnImage<Parent, Element> =
    Box<dyn Fn(&Parent, &Element) -> Result<&'static Texture<'static>, String>>;
