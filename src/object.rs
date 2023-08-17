#![allow(unused)]
use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
//use std::f32::consts::PI;

use crate::consts::*;
use crate::timer::Timer;
use crate::util::*;
use crate::world::*;
use macroquad::{color, prelude::*};
use macroquad::rand::*;
use rapier2d::geometry::*;
use rapier2d::na::{Vector2, Vector, Unit};
use rapier2d::parry::shape::*;
use rapier2d::prelude::*;

pub enum ObjectType {
    STATIC,
    KINEMATIC,
    DYNAMIC,
    ENERGETIC,
}

pub enum ObjectShape {
    CIRCLE {radius: f32},
    CUBE {side: f32},
    RECT {width: f32, height: f32},
}

pub struct Object {
    pub key: u64,
    pub object_type: u8,
    pub loc: Vec2,
    pub rot: f32,
    pub shape: ObjectShape,
    color: Color,
    rigid: Option<RigidBodyHandle>,
    timer: Timer,
    bounding: Option<PrismaticJoint>,
}

impl Object {
    pub fn circle(radius: f32, position: Vec2, types_num: u8) -> Self {
        let m_type = gen_range(0, types_num);
        Self {
            key: gen_range(u64::MIN, u64::MAX),
            object_type: m_type,
            loc: position,
            rot: 0.0,
            shape: ObjectShape::CIRCLE {radius},
            color: random_color_num(types_num),
            rigid: None,
            timer: Timer::new(PRECISION, true, true, true),
            bounding: Some(PrismaticJointBuilder::new(Unit::new_normalize(Vector2::new(1.0, 0.0))).limits([6.0, 8.0]).build()),
        }
    }
    pub fn add_physics_space(&mut self, physics_space: &mut World) {
        match self.shape {
            ObjectShape::CIRCLE{radius} => {
                let handle = physics_space.add_circle_body(self.object_type, &self.loc, radius);
                self.rigid = Some(handle);
            }
            _ => {},
        }
    }
    pub fn update(&mut self, dt: f32, physics: &mut World) {
        match self.rigid {
            Some(handle) => {
                let physics_data = physics.get_physics_data(handle);
                self.loc = physics_data.position;
                self.rot = physics_data.rotation;
                if self.timer.update(dt) {
                    physics.get_around(handle, false, false);
                }
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
        match self.shape {
            ObjectShape::CIRCLE {radius} => {
                self.draw_circle_object(radius);
            },
            _ => {},
        }
    }

    fn draw_circle_object(&self, radius: f32) {
        let x0 = self.loc.x;
        let y0 = self.loc.y;
        draw_circle(x0, y0, radius, self.color); //draw_text(kin_eng_info, x0-18.0, y0, 16.0, WHITE);
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
}


pub struct ObjectCollector {
    pub elements: HashMap<u64, Object>,
}

impl ObjectCollector {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    pub fn add_many_elements(&mut self, elements_num: usize, physics_world: &mut World) {
        for _ in 0..elements_num {
            let element = Object::circle(PARTICLE_SIZE as f32, random_position(WORLD_W, WORLD_H), 9);
            _ = self.add_element(element, physics_world);
        }
    }

    pub fn add_element(&mut self, mut element: Object, physics_world: &mut World) -> u64 {
        let t = element.object_type;
        let key = element.key;
        match element.shape {
            ObjectShape::CIRCLE {radius} => {
                let s = gen_range(PARTICLE_SIZE_MIN, PARTICLE_SIZE);
                element.shape = ObjectShape::CIRCLE { radius: s as f32 };
                let handle = physics_world.add_circle_body(t, &element.loc, s as f32);
                element.rigid = Some(handle);
                self.elements.insert(key, element);
            },
            _ => {},
        }
        return key;
    }
    pub fn get(&self, id: u64) -> Option<&Object> {
        return self.elements.get(&id);
    }

    pub fn remove(&mut self, id: u64) {
        self.elements.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<u64, Object> {
        return self.elements.iter();
    }

    pub fn get_iter_mut(&mut self) -> IterMut<u64, Object> {
        return self.elements.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.elements.len();
    }
}

pub struct MolecularRules {
    pub rules: Vec<(u8, Vec<f32>)>,
}

impl MolecularRules {
    pub fn new(type_num: usize) -> Self {
        let mut rules: Vec<(u8, Vec<f32>)> = vec![];
        for i in 0..type_num {
            let mut rule: Vec<f32> = vec![]; 
            for j in 0..type_num {
                let f = gen_range(-1.0, 1.0);
                rule.push(f);
            }
            let size = gen_range(PARTICLE_SIZE_MIN, PARTICLE_SIZE);
            rules.push((size, rule));
        }
        Self {
            rules,
        }
    }
}