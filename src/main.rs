#![allow(unused)]

mod camera;
//mod consts;
//od kinetic;
mod sim;
mod timer;
mod element;
mod util;
mod world;
mod ui;
mod globals;

use std::time::{SystemTime, UNIX_EPOCH};
use crate::globals::*;
use crate::sim::*;
use crate::util::*;
use macroquad::prelude::*;

struct App {
    pub sim: Simulation,
}

impl App {
    
    pub fn new(font: Font) -> App {
        let config = SimConfig::default();
        let signals = Signals::default();
        set_global_signals(signals);
        Self {
            sim: Simulation::new(config, font.clone()),
        }
    }

    fn seed_generator(&self) {
        let seed = generate_seed();
        println!("SEED: {}", seed);
        rand::srand(seed);
    }

    pub fn start(&mut self) {
        self.seed_generator();
        self.sim.init();
    }

    async fn run(&mut self) {
        loop {
            self.sim.input();
            //self.sim.process_ui();
            if self.sim.is_running() {
                self.sim.update();
                self.sim.draw();
            } else {
                self.sim.signals_check();
            }
            //self.sim.draw_ui();
            next_frame().await;
        }        
    }
}

fn app_configuration() -> Conf {
    Conf {
        window_title: "LAB OF MADNESS".to_string(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        sample_count: 16,
        window_resizable: false,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(app_configuration)]
async fn main() {
    let font = load_ttf_font("jetbrain.ttf").await.expect("can't load font resource!");
    let mut app = App::new(font);
    app.start();
    app.run().await;

}
