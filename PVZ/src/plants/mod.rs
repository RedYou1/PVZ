use std::time::Duration;

use crate::{entity::Entity, projectile::Projectile, sun::Sun, zombie::Zombie};

pub mod nenuphar;
pub mod peashooter;
pub mod sunflower;
pub mod triple_peashooter;

pub trait Plant: Entity {
    fn clone(&self) -> Box<dyn Plant>;
    fn cost(&self) -> u32;
    fn can_go_in_water(&self) -> bool;
    fn is_nenuphar(&self) -> bool;
    #[allow(clippy::type_complexity)]
    fn should_spawn(
        &mut self,
        x: i32,
        y: i32,
        y_pos: usize,
        max_y_pos: usize,
        zombies: &[Vec<Box<dyn Zombie>>],
    ) -> (Vec<Sun>, Vec<(usize, Box<dyn Projectile>)>);
    fn health(&mut self) -> &mut Duration;
}
