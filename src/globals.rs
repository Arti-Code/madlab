#![allow(unused)]
use macroquad::experimental::collections::storage;


pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0;
pub const WORLD_W: f32 = 6000.0;
pub const WORLD_H: f32 = 4500.0;

pub const ELEMENT_SIZE: f32 = 3.0;
pub const ELEMENT_NUM: i32 = 5000;
pub const ELEMENT_SPEED: f32 = 50.0;

pub const ZOOM_RATE: f32 = 1.0 / 800.0;
pub const SCREEN_RATIO: f32 = SCREEN_WIDTH / SCREEN_HEIGHT;

pub const GRAV: f32 = 0.0;
pub const FIELD: f32 = 100.0;

pub const PRECISION: f32 = 0.05;
pub const FORCE: f32 = 250000.0;
pub const TYPES_NUM: u64 = 19;


pub fn set_global_settings(settings: Settings) {
    storage::store(settings);
}

pub fn get_settings() -> Settings {
    return *storage::get::<Settings>();
}

pub fn get_mut_settings() -> Settings {
    return *storage::get_mut::<Settings>();
}

pub fn set_global_signals(signals: Signals) {
    storage::store(signals);
}

pub fn get_signals() -> Signals {
    return *storage::get::<Signals>();
}

pub fn get_mut_signals() -> Signals {
    return *storage::get_mut::<Signals>();
}

/* pub fn set_global_info(info: Info) {
    storage::store(info);
}

pub fn get_info() -> Info {
    return *storage::get::<Info>();
}

pub fn get_mut_info() -> Info {
    return *storage::get_mut::<Info>();
} */

#[derive(Clone, Copy)]
pub struct Settings {
    pub field: f32,
    pub force: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            field: FIELD,
            force: FORCE,
       }
    }
}

#[derive(Clone, Copy)]
pub struct Signals {
    pub quit: bool,
    pub restart: bool,
    pub shuffle_interactions: bool,
}

impl Default for Signals {
    fn default() -> Self {
        Self {
            quit: false,
            restart: false,
            shuffle_interactions: false,
        }
    }
}
