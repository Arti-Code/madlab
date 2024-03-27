#![allow(unused)]
use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
//use std::f32::consts::PI;

use crate::globals::*;
use crate::timer::Timer;
use crate::util::*;
use crate::physics::*;
use crate::physics_types::*;
use macroquad::{color, prelude::*};
use macroquad::rand::*;
use rapier2d::geometry::*;
use rapier2d::na::{Vector2, Vector, Unit};
use rapier2d::parry::shape::*;
use rapier2d::prelude::*;


pub trait Physical {
    fn new(position: Vec2, shape: SharedShape, damping: f32, stroke: Option<Color>, fill: Option<Color>, random_vel: bool, physics: &mut Physics) -> Self;
    fn update(&mut self, physics: &mut Physics);
    fn add_to_physic_space(position: &Vec2, rotation: f32, shape: SharedShape, random_vel: bool, damping: f32, physics: &mut Physics, p_type: u128) -> RigidBodyHandle;
    fn draw(&self, display_mode: DisplayMode, physics: &Physics);
    fn draw_joint(&self, physics: &Physics);
    fn react(&mut self, physics: &mut Physics);
}

pub trait Reactive {
    fn react(&mut self, physics: &mut Physics);
}

/* impl Reactive for Element {

    fn react(&mut self, physics: &mut Physics) {
        let settings = get_settings();
        physics.field_react(self.pos, self.size,  self.physics_type, self.rigid_handle);
    }

} */

pub struct Element {
    pub key: u64,
    pub pos: Vec2,
    pub rot: f32,
    pub shape: SharedShape,
    pub field: f32,
    pub force: f32,
    stroke_color: Option<Color>,
    fill_color: Color,
    rigid_handle: RigidBodyHandle,
    joint: Option<ImpulseJointHandle>,
    pub physics_type: u128,
    timer: f32,
    energy: f32,
    size: f32,
}

impl Physical for Element {
    fn new(position: Vec2, shape: SharedShape, damping: f32, stroke: Option<Color>, fill: Option<Color>, random_vel: bool, physics: &mut Physics) -> Self {
        let settings = get_settings();
        //let types_num = settings.particle_types;
        let colors = physics.types.colors.clone();

        let types_num = colors.len();
        //let colors = vec![
        //    RED, GREEN, BLUE, YELLOW, ORANGE, MAGENTA, DARKGREEN, PURPLE, 
        //    PINK, VIOLET, DARKBLUE, WHITE, SKYBLUE, LIME, DARKPURPLE, 
        //    BROWN, DARKBROWN, DARKGRAY, LIGHTGRAY, 
        //];
        let key = gen_range(u64::MIN, u64::MAX);
        let t = rand::gen_range(0, types_num);
        /* let t: usize = match fix_type {
            None => rand::gen_range(0, types_num),
            Some(t) => t as usize,
        }; */
        let types: Vec<(f32, Color, f32)> = vec![(-1.0_f32, RED, 0.5_f32), (0.0_f32, BLUE, 1.0_f32), (1.0_f32, GREEN, 1.5_f32)];
        let (force, color, field) = types.choose().unwrap();
        //let p_type = physics.types.types.get(&(t as u128)).unwrap();
        let c =  colors.get(t as usize).unwrap();
        let rbh = Self::add_to_physic_space(&position, 0.0, shape.clone(), random_vel, damping, physics, t as u128);
        let timer = 0.1 * rand::gen_range(0.0, 1.0);
        Self {
            key,
            pos: position,
            rot: 0.0,
            shape: shape.clone(),
            field: *field,
            force: *force as f32,
            stroke_color: stroke,
            fill_color: *color,
            rigid_handle: rbh,
            joint: None,
            physics_type: t as u128, 
            timer,
            energy: 0.0, 
            size: shape.0.as_ball().unwrap().radius,
        }
    }

    fn add_to_physic_space(position: &Vec2, rotation: f32, shape: SharedShape, random_vel: bool, damping: f32, physics: &mut Physics, p_type: u128) -> RigidBodyHandle {
        let physics_properties = PhysicsProperties::new(0.1, 0.25, 1.0, damping, 0.3);
        let rbh = physics.add_dynamic(position, rotation, shape, physics_properties, random_vel, p_type);
        return rbh;
    } 

/*     fn update(&mut self, physics: &mut Physics) {
        //self.update_motor(physics);
        let physics_data = physics.get_physics_data(self.rigid_handle);
        self.pos = physics_data.position;
        self.rot = physics_data.rotation;
        if let Some(ek) = physics_data.kin_eng {
            self.energy = ek;
        } 
        match physics.rigid_bodies.get_mut(self.rigid_handle) {
            Some(body) => {
                //self.edges_check(body);
                self.out_of_edges(body);
            }
            None => {
                warn!("can't find rigid body!");
            }
        }
        self.react(physics);
        /* self.timer += get_frame_time();
        if self.timer >= PRECISION {
            self.timer -= PRECISION;
            self.react(physics);
        } */
    } */

    fn update(&mut self, physics: &mut Physics) {
        let physics_data = physics.get_physics_data(self.rigid_handle);
        self.pos = physics_data.position;
        self.rot = physics_data.rotation;
        if let Some(ek) = physics_data.kin_eng {
            self.energy = ek;
        } 
        match physics.rigid_bodies.get_mut(self.rigid_handle) {
            Some(body) => {
                self.out_of_edges(body);
            }
            None => {
                warn!("can't find rigid body!");
            }
        }
        self.react(physics);
    }

    fn react(&mut self, physics: &mut Physics) {
        let particles = physics.get_in_range(self.rigid_handle, &self.pos, self.field);
        
    }

    fn draw(&self, display_mode: DisplayMode, physics: &Physics) {
        let settings = get_settings();
        match display_mode {
            DisplayMode::ELEMENTS => {
                self.draw_circle_object();
            },
            DisplayMode::STROKE => {
                self.draw_circle_stroke_object();
            },
            DisplayMode::ENERGY => {
                self.draw_circle_energy();
            },
        }
        if settings.field_range {
            let r = physics.get_physics_type(self.physics_type).get_field_range() * settings.field;
            draw_circle_lines(self.pos.x, self.pos.y, r, 0.1, LIGHTGRAY);
        }
        
    }

    fn draw_joint(&self, physics: &Physics) {
        let rot_vec = Vec2::from_angle(self.rot)*100.0;
        match self.joint {
            Some(joint_handle) => {
                let joint = physics.impulse_joint_set.get(joint_handle).unwrap();
                match joint.data.as_revolute() {
                    Some(motor) => {
                        let trans = motor.data.local_frame1.translation;
                        
                        let pos = Vec2::new(trans.x, trans.y) + self.pos;
                        draw_circle_lines(pos.x, pos.y, 32.0, 1.5, RED);
                        draw_line(self.pos.x, self.pos.y, self.pos.x+rot_vec.x, self.pos.y+rot_vec.y, 1.0, WHITE);
                    },
                    None => {},
                }                
            },
            None => {},
        }
    }
}

impl Element {

    fn draw_circle_object(&self) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let size = self.size;
        let rot_vec = Vec2::from_angle(self.rot);
        let rv = rot_vec*size*0.5;
        draw_circle(x0, y0, size*2.0, self.fill_color);
    }

    fn draw_circle_stroke_object(&self) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let size = self.size;
        let rot_vec = Vec2::from_angle(self.rot);
        let rv = rot_vec*size*0.5;
        draw_circle_lines(x0, y0, size*2.0, 1.0, self.fill_color);
    }

    fn draw_circle_energy(&self) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let eng = (self.energy/500.0).log10()-1.0;
        let r = clamp(0.2 + eng, 0.0, 1.0);
        let color = Color::new(r, 0.2, 0.2, 1.0);
        let size = self.size;
        let rot_vec = Vec2::from_angle(self.rot);
        let rv = rot_vec*size*0.5;
        draw_circle(x0, y0, size*2.0, color);
    }

    fn update_motor(&mut self, physics: &mut Physics) {
        match self.joint {
            None => {},
            Some(joint_handle) => {
                let joint = physics.impulse_joint_set.get_mut(joint_handle).unwrap();
                joint.data.set_motor_velocity(JointAxis::Y, 10000.0, 1.0);
            },
        }
    }

    fn out_of_edges(&mut self, body: &mut RigidBody) {
        let settings = get_settings();
        let r = settings.world_radius;
        let mut raw_pos = matrix_to_vec2(body.position().translation);
        let dist_from_center = Vec2::ZERO.distance(raw_pos).abs();
        let dir = (raw_pos).normalize_or_zero();
        if dist_from_center >= r/2. {
            let hold_force = dir * (r/2.0 - dist_from_center)/5.0;
            let hf = vector![hold_force.x, hold_force.y];
            body.apply_impulse(hf, true) 
        };
    }

    pub fn set_damping(&mut self, damping: f32, physics: &mut Physics) {
        let rb = physics.rigid_bodies.get_mut(self.rigid_handle).unwrap();
        rb.set_linear_damping(damping);
    }

    pub fn set_size(&mut self, size: f32, density: f32, physics: &mut Physics) {
        let rb = physics.rigid_bodies.get_mut(self.rigid_handle).unwrap();
        if let Some(collider_handle) = rb.colliders().first() {
            let ch = *collider_handle;
            let mut collider = physics.colliders.get_mut(ch).unwrap();
            let shape = SharedShape::ball(size);
            collider.set_shape(shape.clone());
            collider.set_density(density);
            self.shape = shape.clone();
            self.size = size;
        }
    }

}



pub struct ElementCollector {
    pub elements: HashMap<u64, Element>,
}

impl ElementCollector {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    pub fn add_many_elements(&mut self, elements_num: usize, physics: &mut Physics) {
        for _ in 0..elements_num {
            _ = self.add_element(None, GREEN, None, false, physics);
        }
    }

    pub fn add_element(&mut self, position: Option<Vec2>, color: Color, no_random_size: Option<f32>, random_vel: bool, physics: &mut Physics) -> (u64, RigidBodyHandle) {
        let settings = get_settings();
        //let w = settings.width;
        //let h = settings.height;
        let damping = settings.damping;
        let size = match no_random_size {
            None => {
                //gen_range(4.0, 12.0) as f32
                settings.particle_size
            },
            Some(s) => {
                s
            },
        };
        //let r = rand::gen_range(1, 3) + rand::gen_range(1, 2) + rand::gen_range(0, 2);
        let r = settings.particle_size;
        let circle = SharedShape::ball(r as f32);
        let element = match position {
            Some(pos) => {
                Element::new(pos, circle, damping, Some(BLUE), Some(color), random_vel, physics)
            },
            None => {
                //let coord = random_position(w-10.0, h-10.0) + Vec2::new(5.0, 5.0);
                let coord = random_circle_position(settings.world_radius/2.);
                Element::new(coord, circle, damping, Some(BLUE), Some(color), random_vel, physics)
            },    
        };
        let rbh = element.rigid_handle;
        let key = element.key;
        self.elements.insert(key, element);
        return (key, rbh);
    }

    /* pub fn add_motor(&mut self, position: Vec2, physics: &mut Physics) {
        let (k1, rbh1) = self.add_element(Some(position.clone()), YELLOW, Some(6.0), false, physics);
        let (k2, rbh2) = self.add_element(Some(position+Vec2::new(64.0, 0.0)), RED, Some(6.0), false, physics);
        let motor = physics.add_motor(rbh1, rbh2);
        let rb1 = physics.rigid_bodies.get_mut(rbh1).unwrap();
        let e = self.elements.get_mut(&k1).unwrap();
        e.joint = Some(motor);
    } */

    pub fn get(&self, id: u64) -> Option<&Element> {
        return self.elements.get(&id);
    }

    pub fn remove(&mut self, id: u64) {
        self.elements.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<u64, Element> {
        return self.elements.iter();
    }

    pub fn get_iter_mut(&mut self) -> IterMut<u64, Element> {
        return self.elements.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.elements.len();
    }
} 