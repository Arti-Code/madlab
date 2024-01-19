#![allow(unused)]
use macroquad::experimental::collections::storage;


pub const SCREEN_WIDTH: f32 = 1200.0;
pub const SCREEN_HEIGHT: f32 = 800.0;
pub const WORLD_W: f32 = 3000.0;
pub const WORLD_H: f32 = 2000.0;

pub const ELEMENT_SIZE: f32 = 1.0;
pub const ELEMENT_NUM: i32 = 2000;
pub const ELEMENT_SPEED: f32 = 50.0;

pub const ZOOM_RATE: f32 = 1.0 / 800.0;
pub const SCREEN_RATIO: f32 = SCREEN_WIDTH / SCREEN_HEIGHT;

pub const GRAV: f32 = 0.0;
pub const FIELD: f32 = 50.0;

pub const PRECISION: f32 = 0.05;
pub const FORCE: f32 = 10.0;
pub const TYPES_NUM: u64 = 19;

#[derive(Clone, Copy)]
pub enum DisplayMode {
    ELEMENTS,
    STROKE,
    ENERGY,

}
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

#[derive(Clone, Copy)]
pub struct Settings {
    pub world_radius: f32,
    pub field: f32,
    pub force: f32,
    pub strong_force: f32,
    pub strong_field: f32,
    pub repel: f32,
    pub particles_num: usize,
    pub particle_types: usize,
    pub particle_size: f32,
    pub particle_dense: f32,
    pub damping: f32,
    pub display: DisplayMode,
    pub field_range: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            world_radius: 1600.0,
            field: 70.0,
            force: 5.0,
            repel: 0.35,
            particles_num: 1000,
            particle_types: 19,
            particle_size: 1.0,
            particle_dense: 1.0,
            damping: 1.0,
            display: DisplayMode::ELEMENTS,
            strong_force: 500.0,
            strong_field: 0.1,
            field_range: false,
       }
    }
}

#[derive(Clone, Copy)]
pub struct Signals {
    pub start_new_sim: bool,
    pub quit: bool,
    pub restart: bool,
    pub shuffle_interactions: bool,
    pub particles_new_settings: bool,
}

impl Default for Signals {
    fn default() -> Self {
        Self {
            start_new_sim: false,
            quit: false,
            restart: false,
            shuffle_interactions: false,
            particles_new_settings: false,
        }
    }
}
