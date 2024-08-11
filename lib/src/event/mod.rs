use sdl2::rect::{FPoint, FRect};

mod _enum;
mod from;

pub use _enum::Event;

impl Event {
    pub fn hover(self, sub: FRect) -> Option<Self> {
        match self {
            Self::MouseMotion {
                which,
                mousestate,
                x,
                y,
                moved_x,
                moved_y,
            } => {
                if sub.contains_point(FPoint::new(x, y)) {
                    Some(Self::MouseMotion {
                        which,
                        mousestate,
                        x,
                        y,
                        moved_x,
                        moved_y,
                    })
                } else {
                    None
                }
            }
            Self::MouseButtonDown {
                which,
                mouse_btn,
                clicks,
                x,
                y,
            } => {
                if sub.contains_point(FPoint::new(x, y)) {
                    Some(Self::MouseButtonDown {
                        which,
                        mouse_btn,
                        clicks,
                        x,
                        y,
                    })
                } else {
                    None
                }
            }
            Self::MouseButtonUp {
                which,
                mouse_btn,
                clicks,
                x,
                y,
            } => {
                if sub.contains_point(FPoint::new(x, y)) {
                    Some(Self::MouseButtonUp {
                        which,
                        mouse_btn,
                        clicks,
                        x,
                        y,
                    })
                } else {
                    None
                }
            }
            Self::MouseWheel {
                which,
                scroll_x,
                scroll_y,
                direction,
                mouse_x,
                mouse_y,
            } => {
                if sub.contains_point(FPoint::new(mouse_x, mouse_y)) {
                    Some(Self::MouseWheel {
                        which,
                        scroll_x,
                        scroll_y,
                        direction,
                        mouse_x,
                        mouse_y,
                    })
                } else {
                    None
                }
            }
            _ => Some(self),
        }
    }
}
