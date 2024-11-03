mod case;
mod game_of_life;

use std::ptr::addr_of;

use crate::game_of_life::SQUARE_SIZE;
use game_of_life::GameOfLife;
use red_sdl::run;

pub fn main() -> Result<(), String> {
    run(
        "Game of Life",
        30.,
        (SQUARE_SIZE * WIDTH) as u32,
        (SQUARE_SIZE * HEIGHT) as u32,
        |window| window.resizable().position_centered(),
        GameOfLife::new,
    )
}

const WIDTH: usize = 49;
const HEIGHT: usize = 40;
static mut PLAYGROUND: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];
static mut CLOCK: u8 = 0;

pub fn states() -> &'static [[bool; WIDTH]; HEIGHT] {
    unsafe { addr_of!(PLAYGROUND).as_ref().expect("ref") }
}

pub fn state(x: usize, y: usize) -> bool {
    unsafe { PLAYGROUND[y][x] }
}

pub fn set_state(state: bool, x: usize, y: usize) {
    unsafe {
        PLAYGROUND[y][x] = state;
    }
}

pub fn clock() -> u8 {
    unsafe { CLOCK }
}
pub fn next_clock() {
    unsafe { CLOCK += 1 }
}
pub fn reset_clock() {
    unsafe { CLOCK = 0 }
}
