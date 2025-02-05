use std::time::Duration;

use anyhow::Result;
use red_sdl::refs::Ref;
use sdl2::{rect::FRect, render::Texture};

use crate::{projectile::Projectile, sun::Sun, zombie::Zombie, State};

pub mod nenuphar;
pub mod peashooter;
pub mod sunflower;
pub mod triple_peashooter;

pub trait Plant {
    fn texture(&self, state: Ref<State>) -> &'static Texture;
    fn rect(&self, x: f32, y: f32) -> FRect;
    fn update(&mut self, elapsed: Duration) -> Result<()>;

    fn clone(&self) -> Box<dyn Plant>;
    fn cost(&self) -> u32;
    fn can_go_in_water(&self) -> bool;
    fn is_nenuphar(&self) -> bool;
    #[allow(clippy::type_complexity)]
    fn should_spawn(
        &mut self,
        x: f32,
        y: f32,
        y_pos: usize,
        max_y_pos: usize,
        zombies: &[Vec<Box<dyn Zombie>>],
    ) -> (Vec<Sun>, Vec<(usize, Box<dyn Projectile>)>);
    fn health(&mut self) -> &mut Duration;
}
