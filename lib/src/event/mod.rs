use sdl2::rect::{FPoint, FRect};

mod _enum;
mod from;

pub use _enum::Event;

impl Event {
    pub fn hover(self, sub: FRect) -> Result<Self, Self> {
        match self {
            Self::MouseMotion { x, y, .. } => {
                if sub.contains_point(FPoint::new(x, y)) {
                    Ok(self)
                } else {
                    Err(self)
                }
            }
            Self::MouseButtonDown { x, y, .. } => {
                if sub.contains_point(FPoint::new(x, y)) {
                    Ok(self)
                } else {
                    Err(self)
                }
            }
            Self::MouseButtonUp { x, y, .. } => {
                if sub.contains_point(FPoint::new(x, y)) {
                    Ok(self)
                } else {
                    Err(self)
                }
            }
            Self::MouseWheel {
                mouse_x, mouse_y, ..
            } => {
                if sub.contains_point(FPoint::new(mouse_x, mouse_y)) {
                    Ok(self)
                } else {
                    Err(self)
                }
            }
            _ => Ok(self),
        }
    }
}
