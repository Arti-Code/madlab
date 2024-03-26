#![allow(unused)]
use macroquad::experimental::collections::storage;


pub const SCREEN_WIDTH: f32 = 950.0;
pub const SCREEN_HEIGHT: f32 = 950.0;

pub const ZOOM_RATE: f32 = 1.0 / 200.0;
pub const SCREEN_RATIO: f32 = 1.0;
//pub const SCREEN_RATIO: f32 = SCREEN_HEIGHT / SCREEN_WIDTH;

pub const TYPES_NUM: usize = 19;

#[derive(Clone, Copy)]
pub enum DisplayMode {
    ELEMENTS,
    STROKE,
    ENERGY,

}
pub fn set_settings(settings: Settings) {
    storage::store(settings);
}

pub fn get_settings() -> Settings {
    return *storage::get_mut::<Settings>();
}

/* pub fn get_mut_settings() -> Settings {
    return *storage::get_mut::<Settings>();
} */

pub fn set_signals(signals: Signals) {
    storage::store(signals);
}

pub fn signals() -> Signals {
    return *storage::get_mut::<Signals>();
}

/* pub fn get_mut_signals() -> Signals {
    return *storage::get_mut::<Signals>();
} */

#[derive(Clone, Copy)]
pub struct Settings {
    pub world_radius: f32,
    pub field: f32,
    pub force: f32,
    pub repel: f32,
    pub particles_num: usize,
    pub particle_types: usize,
    pub particle_size: f32,
    pub particle_dense: f32,
    pub damping: f32,
    pub display: DisplayMode,
    pub field_range: bool,
    pub repel_on: bool,
    pub collisions: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            world_radius: 2500.0,
            field: 60.0,
            force: 15.0,
            repel: 0.55,
            particles_num: 2000,
            particle_types: 19,
            particle_size: 1.0,
            particle_dense: 1.0,
            damping: 1.0,
            display: DisplayMode::ELEMENTS,
            field_range: false,
            repel_on: true,
            collisions: false,
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
