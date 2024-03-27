use std::collections::HashMap;
use macroquad::color::colors;
use macroquad::rand::rand;
use macroquad::{color::Color, rand}; 
use macroquad::prelude::*;
use crate::globals::*;


pub struct PhysicsType {
    type_id: u128,
    actions: [f32; TYPES_NUM as usize],
    color: Color,
    field: f32,
}

impl PhysicsType {
    pub fn new(type_id: u128, color: Color) -> Self {
        let mut actions: [f32; TYPES_NUM as usize] = [0.0; TYPES_NUM as usize];
        for i in 0..TYPES_NUM as usize {
            let mut a: f32 = 0.0;
            if i == TYPES_NUM-1 && type_id as usize == TYPES_NUM-1 {

            } else {
                a = rand::gen_range(-1.0, 1.0);
            }
            actions[i] = a;
        }
        Self { 
            type_id, 
            actions, 
            color,
            field: 0.5 + rand::gen_range(0.0, 1.0), 
        }
    }

    pub fn get_field_range(&self) -> f32 {
        return self.field;
    }

    pub fn get_action(&self, id: usize) -> f32 {
        return self.actions[id];
    }


}

pub struct PhysicsTypes {
    pub types: HashMap<u128, PhysicsType>,
    pub colors: Vec<Color>,
}

impl PhysicsTypes {

    pub fn random() -> Self {
        let mut types: HashMap<u128, PhysicsType> = HashMap::new();
        let num = get_settings().particles_num;
        //let colors = vec![RED, GREEN, BLUE, YELLOW, ORANGE, MAGENTA, DARKGREEN, PURPLE, PINK, VIOLET, DARKBLUE, WHITE, SKYBLUE, LIME, DARKPURPLE, BROWN, DARKBROWN, DARKGRAY, LIGHTGRAY ];
        let colors = Self::generate_colors(num);
        for n in 0..colors.len()-1 {
            //let action: f32 = rand::gen_range(-1.0, 1.0);
            let type_id = n as u128;
            let color = colors[n];
            let t = PhysicsType::new(type_id, color);
            types.insert(type_id, t);
        }
        Self { types, colors }
    }

    pub fn get_type(&self, id: u128) -> &PhysicsType {
        return self.types.get(&id).unwrap();
    }

    fn generate_colors(n: usize) -> Vec<Color> {
        let mut colors: Vec<Color> = Vec::new();
        let root = (n as f64).powf(1.0/3.0) as i32;
        //let step = (1.0/root) as f32;

        for _ in 0..n {
            let ir = rand::gen_range(0, root);
            let ig = rand::gen_range(0, root);
            let ib = rand::gen_range(0, root);
            let r = ir as f32 / (root as f32);
            let g = ig as f32 / (root as f32);
            let b = ib as f32 / (root as f32);
            let color = Color::new(r, g, b, 1.0);
            colors.push(color);
        }
        return colors;
    }

}

pub struct PhysicsProperties {
    pub friction: f32,
    pub restitution: f32,
    pub density: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
}

impl Default for PhysicsProperties {
    
    fn default() -> Self {
        Self { friction: 0.5, restitution: 0.5, density: 0.5, linear_damping: 0.1, angular_damping: 0.9 }
    }
}

impl PhysicsProperties {
    
    pub fn new(friction: f32, restitution: f32, density: f32, linear_damping: f32, angular_damping: f32) -> Self {
        Self { friction, restitution, density, linear_damping, angular_damping }
    }

    pub fn bounce() -> Self {
        Self { friction: 0.0, restitution: 1.0, density: 1.0, linear_damping: 0.1, angular_damping: 0.1 }
    }

    pub fn free() -> Self {
        Self { friction: 0.0, restitution: 1.4, density: 0.1, linear_damping: 0.01, angular_damping: 0.01 }
    }
}

pub struct PhysicsData {
    pub position: Vec2,
    pub rotation: f32,
    pub mass: f32,
    pub kin_eng: Option<f32>,
}