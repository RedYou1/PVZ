mod case;
mod game_of_life;

use crate::game_of_life::SQUARE_SIZE;
use anyhow::{anyhow, Result};
use game_of_life::GameOfLife;
use red_sdl::run_game;
use sdl2::{
    pixels::Color,
    rect::Point,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

pub struct State {
    pub texture1: Texture<'static>,
    pub texture2: Texture<'static>,
}

const WIDTH: usize = 49;
const HEIGHT: usize = 40;
pub fn main() -> Result<()> {
    run_game(
        "Game of Life",
        (SQUARE_SIZE * WIDTH) as u32,
        (SQUARE_SIZE * HEIGHT) as u32,
        |window| window.resizable().position_centered(),
        |canvas| {
            let (texture1, texture2) =
                dummy_texture(canvas, Box::leak(Box::new(canvas.texture_creator())))?;
            Ok(State { texture1, texture2 })
        },
        GameOfLife::new,
    )
}

fn dummy_texture<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<(Texture<'a>, Texture<'a>)> {
    let mut square_texture1 = texture_creator
        .create_texture_target(None, SQUARE_SIZE as u32, SQUARE_SIZE as u32)
        .map_err(|e| anyhow!(e))?;
    let mut square_texture2 = texture_creator
        .create_texture_target(None, SQUARE_SIZE as u32, SQUARE_SIZE as u32)
        .map_err(|e| anyhow!(e))?;

    // let's change the textures we just created
    let textures = [
        (
            &mut square_texture1,
            (Color::RGB(255, 255, 0), Color::RGB(200, 200, 0)),
        ),
        (
            &mut square_texture2,
            (Color::RGB(192, 192, 192), Color::RGB(64, 64, 64)),
        ),
    ];
    let mut success = Ok(());
    canvas
        .with_multiple_texture_canvas(textures.iter(), |texture_canvas, &(c1, c2)| {
            for i in 0..SQUARE_SIZE as i32 {
                for j in 0..SQUARE_SIZE as i32 {
                    // drawing pixel by pixel isn't very effective, but we only do it once and store
                    // the texture afterwards so it's still alright!
                    if (i + j) % 7 == 0 {
                        // this doesn't mean anything, there was some trial and serror to find
                        // something that wasn't too ugly
                        texture_canvas.set_draw_color(c1);
                        success = texture_canvas.draw_point(Point::new(i, j));
                        if success.is_err() {
                            return;
                        }
                    }
                    if (i + j * 2) % 5 == 0 {
                        texture_canvas.set_draw_color(c2);
                        success = texture_canvas.draw_point(Point::new(i, j));
                        if success.is_err() {
                            return;
                        }
                    }
                }
            }
        })
        .map_err(|e| anyhow!(e))?;
    success.map_err(|e| anyhow!(e))?;

    Ok((square_texture1, square_texture2))
}
