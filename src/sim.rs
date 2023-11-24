#![allow(unused)]
use crate::element::*;
use crate::camera::*;
use crate::globals::*;
//use crate::kinetic::*;
use crate::timer::Timer;
use crate::ui::*;
use crate::util::*;
use crate::world::*;
//use egui_macroquad;
use macroquad::camera::Camera2D;
use macroquad::prelude::*;
use std::collections::VecDeque;
use std::f32::consts::PI;

pub struct Simulation {
    pub simulation_name: String,
    pub world_size: Vec2,
    pub font: Font,
    pub world: World,
    pub camera: Camera2D,
    pub running: bool,
    pub sim_time: f64,
    config: SimConfig,
    //pub ui: UISystem,
    pub sim_state: SimState,
    pub signals: Signals,
    select_phase: f32,
    pub selected: u64,
    pub mouse_state: MouseState,
    //pub object_collector: ObjectCollector,
    pub elements: ElementCollector,
    info_time: Timer,
    info: bool,
    total_eng: f32,
    ui: UI,
    fps: i32,
    fps2: VecDeque<i32>,
    avg: i32,
    fps_timer: Timer,
}

impl Simulation {
    pub fn new(configuration: SimConfig, font: Font) -> Self {
        Self {
            simulation_name: String::new(),
            world_size: Vec2 {
                x: WORLD_W,
                y: WORLD_H,
            },
            font,
            world: World::new(),
            camera: create_camera(),
            running: true,
            sim_time: 0.0,
            config: configuration,
            //ui: UISystem::new(),
            sim_state: SimState::new(),
            signals: Signals::default(),
            selected: 0,
            select_phase: 0.0,
            mouse_state: MouseState { pos: Vec2::NAN },
            elements: ElementCollector::new(),
            info_time: Timer::new(1.0, true, true, false),
            info: true,
            total_eng: 0.0,
            ui: UI::new(),
            fps: 0,
            fps2: [30; 30].into(),
            avg: 0,
            fps_timer: Timer::new(1.0, true, true, false),
        }
    }

    fn reset_sim(&mut self, sim_name: Option<&str>) {
        let seed = generate_seed();
        println!("SEED: {}", seed);
        rand::srand(seed);
        self.simulation_name = match sim_name {
            Some(name) => name.to_string(),
            None => String::new(),
        };
        self.world = World::new();
        self.elements = ElementCollector::new();
        //self.elements = ObjectCollector::new();
        self.sim_time = 0.0;
        self.sim_state = SimState::new();
        self.sim_state.sim_name = String::from(&self.simulation_name);
        self.signals = Signals::default();
        self.selected = 0;
        self.select_phase = 0.0;
        self.mouse_state = MouseState { pos: Vec2::NAN };
        self.running = true;
        self.init();
    }

    pub fn init(&mut self) {
        let settings = get_settings();
        self.elements.add_many_elements(settings.particles_num, &mut self.world);
    }

    fn update_particles(&mut self) {
        for (id, elem) in self.elements.get_iter_mut() {
            elem.update(&mut self.world);
        }
    }

    fn set_particles_damping(&mut self, damping: f32) {
        for (_, mut particle) in self.elements.get_iter_mut() {
            particle.set_damping(damping, &mut self.world);
        }
    }

    pub fn update(&mut self) {
        //if self.info_time.update(get_frame_time()) {
        //    self.total_eng = self.world.get_total_kinetic_eng();
        //}
        self.signals_check();
        self.process_ui();
        self.update_sim_state();
        //self.check_agents_num();
        self.calc_selection_time();
        self.update_particles();
        self.world.step_physics();
    }

    pub fn draw(&mut self) {

        //set_default_camera();
        set_camera(&self.camera);
        clear_background(BLACK);
        draw_rectangle_lines(0.0, 0.0, self.world_size.x, self.world_size.y, 3.0, WHITE);
        //self.draw_grid(50);
        self.draw_particles();
        self.draw_info();
        self.draw_ui();
    }

    fn draw_info(&mut self) {
        if self.fps_timer.update(get_frame_time()) {
            self.fps = get_fps();
            let sum: i32 = self.fps2.iter().sum();
            self.avg = sum/(self.fps2.len() as i32);
        }
        self.fps2.push_back(get_fps());
        self.fps2.pop_front();
    }

    fn draw_particles(&self) {
        let settings = get_settings();
        for (id, p) in self.elements.get_iter() {
            p.draw(settings.display, &self.world);
        }
    }

    fn draw_grid(&self, cell_size: u32) {
        let w = self.world_size.x;
        let h = self.world_size.y;
        let col_num = (w / cell_size as f32).floor() as u32;
        let row_num = (h / cell_size as f32).floor() as u32;
        //draw_grid(100, 20.0, GRAY, DARKGRAY);
        for x in 0..col_num + 1 {
            for y in 0..row_num + 1 {
                draw_circle((x * cell_size) as f32, (y * cell_size) as f32, 1.0, GRAY);
            }
        }
    }

    pub fn signals_check(&mut self) {
        let mut signals = get_signals();
        if signals.shuffle_interactions {
            signals.shuffle_interactions = false;
            self.world.random_types();
        }
        if signals.start_new_sim {
            signals.start_new_sim = false;
            self.reset_sim(None);
        }
        if signals.restart {
            self.reset_sim(None);
            signals.restart = false;
        }
        if self.signals.quit {
            std::process::exit(0);
        }
        if signals.particles_new_settings {
            signals.particles_new_settings = false;
            let settings = get_settings();
            self.set_particles_damping(settings.damping);
        }
        set_global_signals(signals);
    }

    pub fn input(&mut self) {
        self.mouse_input();
        control_camera(&mut self.camera);
    }

    fn mouse_input(&mut self) {
        if is_mouse_button_released(MouseButton::Left) {
            //if !self.ui.pointer_over {
            if true {
                self.selected = 0;
                let (mouse_posx, mouse_posy) = mouse_position();
                let mouse_pos = Vec2::new(mouse_posx, mouse_posy);
                let rel_coords = self.camera.screen_to_world(mouse_pos);
            }
        }
    }

    fn update_sim_state(&mut self) {
        self.sim_state.fps = get_fps();
        self.sim_state.dt = get_frame_time();
        self.sim_state.total_k_eng = self.world.get_total_kinetic_eng().round();
        self.sim_state.sim_time += self.sim_state.dt as f64;
        let (mouse_x, mouse_y) = mouse_position();
        self.mouse_state.pos = Vec2::new(mouse_x, mouse_y);
        self.sim_state.physics_num = self.world.get_physics_obj_num() as i32;
    }

    fn check_agents_num(&mut self) {
        if self.elements.count() < ELEMENT_NUM as usize {
            self.elements.add_many_elements(1, &mut self.world);
        }
    }

    fn calc_selection_time(&mut self) {
        self.select_phase += self.sim_state.dt * 4.0;
        self.select_phase = self.select_phase % (2.0 * PI);
    }

    pub fn process_ui(&mut self) {
        self.ui.process(self.fps, self.avg);
    }

    pub fn draw_ui(&self) {
        self.ui.draw();
    }

    pub fn is_running(&self) -> bool {
        return self.running;
    }
}

//?         [[[SIM_CONFIG]]]
#[derive(Clone, Copy)]
pub struct SimConfig {
    pub agents_init_num: usize,
    pub agent_min_num: usize,
    pub agent_speed: f32,
    pub agent_vision_range: f32,
    pub agent_rotation: f32,
    pub sources_init_num: usize,
    pub sources_min_num: usize,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            agents_init_num: 0,
            agent_min_num: 0,
            agent_speed: 0.0,
            agent_rotation: 0.0,
            agent_vision_range: 0.0,
            sources_init_num: 0,
            sources_min_num: 0,

        }
    }
}

impl SimConfig {
    pub fn new(
        agents_num: usize,
        agents_min_num: usize,
        agent_speed: f32,
        agent_turn: f32,
        vision_range: f32,
        sources_num: usize,
        sources_min_num: usize,
    ) -> Self {
        Self {
            agents_init_num: agents_num,
            agent_min_num: agents_min_num,
            agent_speed,
            agent_rotation: agent_turn,
            agent_vision_range: vision_range,
            sources_init_num: sources_num,
            sources_min_num,
        }
    }
}

//?         [[[SIM_STATE]]]
pub struct SimState {
    pub sim_name: String,
    pub agents_num: i32,
    pub sources_num: i32,
    pub asteroids_num: i32,
    pub particles_num: i32,
    pub jets_num: i32,
    pub physics_num: i32,
    pub sim_time: f64,
    pub fps: i32,
    pub dt: f32,
    pub total_k_eng: f32,
}

impl SimState {
    pub fn new() -> Self {
        Self {
            sim_name: String::new(),
            agents_num: 0,
            sources_num: 0,
            asteroids_num: 0,
            particles_num: 0,
            jets_num: 0,
            physics_num: 0,
            sim_time: 0.0,
            fps: 0,
            dt: 0.0,
            total_k_eng: 0.0,
        }
    }
}

//?         [[[MOUSESTATE]]]
pub struct MouseState {
    pub pos: Vec2,
}
