pub mod level;
pub mod map_plant;
pub mod plants;
pub mod projectile;
pub mod save;
pub mod shop_plant;
pub mod sun;
pub mod texts;
pub mod textures;
pub mod win;
pub mod zombie;

pub static mut UPDATE_AVAILABLE: Option<Result<bool, String>> = None;
