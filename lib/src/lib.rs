extern crate sdl2;

pub mod game_window;

use std::{thread, time};

use game_window::GameWindow;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn run<Game: GameWindow, Func: Fn(&mut Canvas<Window>) -> Result<Game, String>>(
    title: &str,
    fps: f32,
    width: u32,
    height: u32,
    func: Func,
) -> Result<(), String> {
    let fps = time::Duration::from_secs_f32(1. / fps);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // the window is the representation of a window in your operating system,
    // however you can only manipulate properties of that window, like its size, whether it's
    // fullscreen, ... but you cannot change its content without using a Canvas or using the
    // `surface()` method.
    let window = video_subsystem
        .window(title, width, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // the canvas allows us to both manipulate the property of the window and to change its content
    // via hardware or software rendering. See CanvasBuilder for more info.
    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    println!("Using SDL_Renderer \"{}\"", canvas.info().name);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    // clears the canvas with the color we set in `set_draw_color`.
    canvas.clear();
    // However the canvas has not been updated to the window yet, everything has been processed to
    // an internal buffer, but if we want our buffer to be displayed on the window, we need to call
    // `present`. We need to call this everytime we want to render a new frame on the window.
    canvas.present();

    let mut game = func(&mut canvas)?;

    let mut event_pump = sdl_context.event_pump()?;
    while game.running() {
        let now = time::Instant::now();

        for event in event_pump.poll_iter() {
            game.event(&mut canvas, event)?;
        }

        game.update(&mut canvas)?;
        game.draw(&mut canvas)?;
        canvas.present();

        let elapsed = time::Instant::now() - now;
        if elapsed < fps {
            thread::sleep(fps - elapsed);
        }
    }

    Ok(())
}
