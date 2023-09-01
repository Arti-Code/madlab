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


pub trait Physical {
    fn new(position: Vec2, shape: SharedShape, stroke: Option<Color>, fill: Option<Color>, physics: &mut World) -> Self;
    fn update(&mut self, physics: &mut World);
    fn add_to_physic_space(position: &Vec2, rotation: f32, shape: SharedShape, physics: &mut World) -> RigidBodyHandle;
    fn draw(&self);
}

pub struct Element {
    pub pos: Vec2,
    pub rot: f32,
    pub shape: SharedShape,
    stroke_color: Option<Color>,
    fill_color: Option<Color>,
    rigid_handle: RigidBodyHandle,
}

impl Physical for Element {
    fn new(position: Vec2, shape: SharedShape, stroke: Option<Color>, fill: Option<Color>, physics: &mut World) -> Self {
        let rbh = Self::add_to_physic_space(&position, 0.0, shape.clone(), physics);
        Self { pos: position, rot: 0.0, shape: shape, stroke_color: stroke, fill_color: fill, rigid_handle: rbh }
    }

    fn add_to_physic_space(position: &Vec2, rotation: f32, shape: SharedShape, physics: &mut World) -> RigidBodyHandle {
        let physics_properities = PhysicsProperities::default();
        let rbh = physics.add_dynamic(position, rotation, shape, physics_properities);
        return rbh;
    } 

    fn update(&mut self, physics: &mut World) {
        todo!()
    }

    fn draw(&self) {
        todo!()
    }

}

impl Element {

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
            let element = Object::new(PARTICLE_SIZE as f32, random_position(WORLD_W, WORLD_H));
            _ = self.add_element(element, physics_world);
        }
    }

    pub fn add_element(&mut self, mut element: Object, physics_world: &mut World) -> u64 {
        let key = element.key;
        let s = PARTICLE_SIZE as f32;
        let handle = physics_world.add_circle_body(key, &element.loc, s);
        element.rigid = Some(handle);
        self.elements.insert(key, element);
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
} */