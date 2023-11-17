#![allow(unused)]
use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
//use std::f32::consts::PI;

use crate::globals::*;
use crate::timer::Timer;
use crate::util::*;
use crate::world::*;
use macroquad::{color, prelude::*};
use macroquad::rand::*;
use rapier2d::geometry::*;
use rapier2d::na::{Vector2, Vector, Unit};
use rapier2d::parry::shape::*;
use rapier2d::prelude::*;


pub trait Physical {
    fn new(position: Vec2, shape: SharedShape, stroke: Option<Color>, fill: Option<Color>, random_vel: bool, physics: &mut World) -> Self;
    fn update(&mut self, physics: &mut World);
    fn add_to_physic_space(position: &Vec2, rotation: f32, shape: SharedShape, random_vel: bool, physics: &mut World, p_type: u128) -> RigidBodyHandle;
    fn draw(&self, physics: &World);
}

pub trait Reactive {
    fn react(&mut self, physics: &mut World);
}

impl Reactive for Element {

    fn react(&mut self, physics: &mut World) {
        physics.field_react(self.pos, FIELD,  self.physics_type, self.rigid_handle);
    }

}

pub struct Element {
    pub key: u64,
    pub pos: Vec2,
    pub rot: f32,
    pub shape: SharedShape,
    stroke_color: Option<Color>,
    fill_color: Color,
    rigid_handle: RigidBodyHandle,
    joint: Option<ImpulseJointHandle>,
    pub physics_type: u128,
    timer: f32
}

impl Physical for Element {
    fn new(position: Vec2, shape: SharedShape, stroke: Option<Color>, fill: Option<Color>, random_vel: bool, physics: &mut World) -> Self {
        let colors = vec![RED, GREEN, BLUE, YELLOW, ORANGE, MAGENTA, DARKGREEN, PURPLE, PINK, VIOLET, DARKBLUE, WHITE, SKYBLUE, LIME, DARKPURPLE, BROWN, DARKBROWN, DARKGRAY, LIGHTGRAY ];
        let key = gen_range(u64::MIN, u64::MAX);
        let t: u64 = rand::gen_range(0, TYPES_NUM);
        //let p_type = physics.types.types.get(&(t as u128)).unwrap();
        let c =  colors.get(t as usize).unwrap();
        let rbh = Self::add_to_physic_space(&position, 0.0, shape.clone(), random_vel, physics, t as u128);
        let timer = PRECISION * rand::gen_range(0.0, 1.0);
        Self {key, pos: position, rot: 0.0, shape: shape, stroke_color: stroke, fill_color: *c, rigid_handle: rbh, joint: None, physics_type: t as u128, timer }
    }

    fn add_to_physic_space(position: &Vec2, rotation: f32, shape: SharedShape, random_vel: bool, physics: &mut World, p_type: u128) -> RigidBodyHandle {
        let physics_properties = PhysicsProperties::new(0.1, 0.25, 1.0, 0.6, 0.3);
        let rbh = physics.add_dynamic(position, rotation, shape, physics_properties, random_vel, p_type);
        return rbh;
    } 

    fn update(&mut self, physics: &mut World) {
        //self.update_motor(physics);
        let physics_data = physics.get_physics_data(self.rigid_handle);
        self.pos = physics_data.position;
        self.rot = physics_data.rotation;
        match physics.rigid_bodies.get_mut(self.rigid_handle) {
            Some(body) => {
                self.edges_check(body);
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
    }

    fn draw(&self, physics: &World) {
        self.draw_circle_object();
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
        let size = self.shape.as_ball().unwrap().radius;
        let rot_vec = Vec2::from_angle(self.rot);
        let rv = rot_vec*size*0.5;
//        let fill = match self.fill_color {
//            Some(color) => color,
//            None => LIGHTGRAY,
//        };
        draw_circle(x0, y0, size*2.0, self.fill_color);
//        match self.stroke_color {
//            Some(color) => {
//                draw_circle(x0+rv.x, y0+rv.y, size/2.0, color);
//            },
//            None =>{},
//        }
    }

    fn update_motor(&mut self, physics: &mut World) {
        match self.joint {
            None => {},
            Some(joint_handle) => {
                let joint = physics.impulse_joint_set.get_mut(joint_handle).unwrap();
                joint.data.set_motor_velocity(JointAxis::Y, 10000.0, 1.0);
            },
        }
    }

    fn edges_check(&mut self, body: &mut RigidBody) {
        let mut raw_pos = matrix_to_vec2(body.position().translation);
        let mut out_of_edge = false;
        if raw_pos.x < 0.0 {
            raw_pos.x = WORLD_W - 5.0;
            out_of_edge = true;
        } else if raw_pos.x > WORLD_W {
            raw_pos.x = 5.0;
            out_of_edge = true;
        }
        if raw_pos.y < 0.0 {
            raw_pos.y = WORLD_H - 5.0;
            out_of_edge = true;
        } else if raw_pos.y > WORLD_H {
            raw_pos.y = 5.0;
            out_of_edge = true;
        }
        if out_of_edge {
            body.set_position(make_isometry(raw_pos.x, raw_pos.y, self.rot), true);
        }
    }
}

/* pub struct Object {
    pub key: u64,
    pub loc: Vec2,
    pub rot: f32,
    pub size: f32,
    color: Color,
    rigid: Option<RigidBodyHandle>,
    timer: Timer,
}

impl Object {

    pub fn new(radius: f32, position: Vec2) -> Self {
        Self {
            key: gen_range(u64::MIN, u64::MAX),
            loc: position,
            rot: 0.0,
            size: radius,
            color: GREEN,
            rigid: None,
            timer: Timer::new(PRECISION, true, true, true),
        }
    }

    pub fn add_physics_space(&mut self, physics_space: &mut World) {
        let handle = physics_space.add_circle_body(self.key, &self.loc, self.size);
        self.rigid = Some(handle);
    }

    pub fn update(&mut self, dt: f32, physics: &mut World) {
        match self.rigid {
            Some(handle) => {
                let physics_data = physics.get_physics_data(handle);
                self.loc = physics_data.position;
                self.rot = physics_data.rotation;
                match physics.rigid_bodies.get_mut(handle) {
                    Some(body) => {
                        self.edges_check(body);
                    }
                    None => {
                        warn!("can't find rigid body!");
                    }
                }
            }
            None => {
                warn!("object rigid body handle is NONE!");
            }
        }
    }

    pub fn draw(&self) {
        self.draw_circle_object();
    }

    fn draw_circle_object(&self) {
        let x0 = self.loc.x;
        let y0 = self.loc.y;
        draw_circle(x0, y0, self.size, self.color); //draw_text(kin_eng_info, x0-18.0, y0, 16.0, WHITE);
    }

    fn edges_check(&mut self, body: &mut RigidBody) {
        let mut raw_pos = matric_to_vec2(body.position().translation);
        let mut out_of_edge = false;
        if raw_pos.x < 0.0 {
            raw_pos.x = WORLD_W - 5.0;
            out_of_edge = true;
        } else if raw_pos.x > WORLD_W {
            raw_pos.x = 5.0;
            out_of_edge = true;
        }
        if raw_pos.y < 0.0 {
            raw_pos.y = WORLD_H - 5.0;
            out_of_edge = true;
        } else if raw_pos.y > WORLD_H {
            raw_pos.y = 5.0;
            out_of_edge = true;
        }
        if out_of_edge {
            body.set_position(make_isometry(raw_pos.x, raw_pos.y, self.rot), true);
        }
    }
} */


pub struct ElementCollector {
    pub elements: HashMap<u64, Element>,
}

impl ElementCollector {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    pub fn add_many_elements(&mut self, elements_num: usize, physics: &mut World) {
        for _ in 0..elements_num {
            _ = self.add_element(None, GREEN, None, false, physics);
        }
    }

    pub fn add_element(&mut self, position: Option<Vec2>, color: Color, no_random_size: Option<f32>, random_vel: bool, physics: &mut World) -> (u64, RigidBodyHandle) {
        let size = match no_random_size {
            None => {
                //gen_range(4.0, 12.0) as f32
                ELEMENT_SIZE
            },
            Some(s) => {
                s
            },
        };
        let circle = SharedShape::ball(size);
        let element = match position {
            Some(pos) => {
                Element::new(pos, circle, Some(BLUE), Some(color), random_vel, physics)
            },
            None => {
                //let mut p = random_unit_vec2()*50.0;
                //let pos = Vec2:: new(p.x+WORLD_W/2.0, p.y+WORLD_H/2.0);

                //let mut p = random_unit_vec2();
                //let pos = Vec2:: new(p.x+WORLD_W, p.y+WORLD_H);
                let coord = random_position(WORLD_W-1000.0, WORLD_H-1000.0) + Vec2::new(500.0, 500.0);
                Element::new(coord, circle, Some(BLUE), Some(color), random_vel, physics)
            },    
        };
        let rbh = element.rigid_handle;
        let key = element.key;
        self.elements.insert(key, element);
        return (key, rbh);
    }

    pub fn add_motor(&mut self, position: Vec2, physics: &mut World) {
        let (k1, rbh1) = self.add_element(Some(position.clone()), YELLOW, Some(6.0), false, physics);
        let (k2, rbh2) = self.add_element(Some(position+Vec2::new(64.0, 0.0)), RED, Some(6.0), false, physics);
        let motor = physics.add_motor(rbh1, rbh2);
        let rb1 = physics.rigid_bodies.get_mut(rbh1).unwrap();
        let e = self.elements.get_mut(&k1).unwrap();
        e.joint = Some(motor);
    }

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