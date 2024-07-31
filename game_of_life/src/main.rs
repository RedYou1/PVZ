mod game_of_life;

use crate::game_of_life::SQUARE_SIZE;
use game_of_life::GameOfLife;
use sdl::run;

pub fn main() -> Result<(), String> {
    const WIDTH: usize = 49;
    const HEIGHT: usize = 40;

    run(
        "Game of Life",
        30.,
        (SQUARE_SIZE * WIDTH) as u32,
        (SQUARE_SIZE * HEIGHT) as u32,
        |window| window.position_centered(),
        GameOfLife::<WIDTH, HEIGHT>::new,
    )
}
