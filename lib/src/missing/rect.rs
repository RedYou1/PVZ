use sdl2::rect::FRect;

pub fn scale(surface: FRect, scale: FRect) -> FRect {
    FRect::new(
        scale.x() * surface.width() + surface.x(),
        scale.y() * surface.height() + surface.y(),
        scale.width() * surface.width(),
        scale.height() * surface.height(),
    )
}
