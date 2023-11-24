//#![allow(unused)]

use std::time::{UNIX_EPOCH, Duration};
use std::f32::consts::PI;
use crate::globals::*;
use macroquad::{color, prelude::*};
use rapier2d::na::*;


pub fn random_unit() -> f32 {
    return rand::gen_range(-1.0, 1.0);
}

pub fn random_position(x_max: f32, y_max: f32) -> Vec2 {
    let x = rand::gen_range(0.0, x_max);
    let y = rand::gen_range(0.0, y_max);
    return Vec2::new(x, y);
}

pub fn random_rotation() -> f32 {
    let rot = rand::gen_range(0.0, PI * 2.0);
    return rot;
}

pub fn random_unit_vec2() -> Vec2 {
    let x = rand::gen_range(-1.0, 1.0);
    let y = rand::gen_range(-1.0, 1.0);
    return Vec2::new(x, y).normalize_or_zero();
}

pub fn random_color() -> color::Color {
    let colors = vec![RED, GREEN, BLUE, YELLOW, GRAY/* , ORANGE, GRAY, SKYBLUE, LIME */];
    let num = colors.len();
    let c = rand::gen_range(0, num);
    return colors[c];
}

pub fn random_color_num(mut num: u8) -> color::Color {
    let colors = vec![RED, GREEN, BLUE, YELLOW, ORANGE, LIME, MAGENTA, PINK, VIOLET, SKYBLUE];
    let num_max = colors.len();
    if num >= num_max as u8 {
        num = (num_max - 1) as u8;
    }
    let c = rand::gen_range(0, num);
    return colors[c as usize];
}

pub fn random_color5() -> color::Color {
    let colors = [RED, BLUE, GREEN, YELLOW, WHITE];
    //let num = colors.len();
    let c = rand::gen_range(0, 5);
    return colors[c];
}

pub fn angle2vec2(angle: f32) -> Vec2 {
    let (x, y) = angle.sin_cos();
    let v = Vec2::new(x, y).normalize_or_zero();
    return v;
}

pub fn wrap_around(v: &Vec2) -> Vec2 {
    let tolerance = 5.0;
    let mut vr = Vec2::new(v.x, v.y);
    if vr.x > WORLD_W + tolerance {
        vr.x = 0.0 - tolerance;
    } else if vr.x < 0.0 - tolerance {
        vr.x = WORLD_W + tolerance;
    }
    if vr.y > WORLD_H + tolerance {
        vr.y = 0.0 - tolerance;
    } else if vr.y < 0.0 - tolerance {
        vr.y = WORLD_H + tolerance;
    }
    return vr;
}

pub fn make_isometry(posx: f32, posy: f32, rotation: f32) -> Isometry2<f32> {
    let iso = Isometry2::new(Vector2::new(posx, posy), rotation);
    return iso;
}

pub fn matrix_to_vec2(translation: Translation<f32, 2>) -> Vec2 {
    return Vec2::new(translation.x, translation.y);
}

pub fn map_polygon(n: usize, r: f32, dev: f32) -> Vec<Vec2> {
    let mut points: Vec<Vec2> = vec![];
    //let mut opoints: Vec<Point2<f32>> = vec![];
    let s = 2.0 * PI / (n as f32);
    let mut a = 2.0 * PI;
    for i in 0..n {
        let mut step = s;
        let mut d = rand::gen_range(-dev, dev);
        if dev == 0.0 {
            d = 0.0;
        }
        step = s + s * d;
        if i == 0 {
            step = 0.0;
        }
        a -= step;
        let x = a.sin();
        let y = a.cos();
        let mut v = Vec2::new(x, y);
        v = v.normalize();
        v = v * r;
        //let p = Point2::new(v.x, v.y);
        points.push(v);
        //opoints.push(p);
    }
    return points;
}

fn vec2_to_point2(v: &Vec2) -> Point2<f32> {
    return Point2::new(v.x, v.y);
}

pub fn vec2_to_point2_collection(vec2_list: &Vec<Vec2>) -> Vec<Point2<f32>> {
    let mut points: Vec<Point2<f32>> = vec![];
    for v in vec2_list.iter() {
        let p = Point2::new(v.x, v.y);
        points.push(p);
    }
    return points;
}

pub fn generate_seed() -> u64 {
    let t0 = UNIX_EPOCH.elapsed().unwrap().as_secs();
    let tx = (t0%100).pow(2);
    let t1 = (t0 as f32).sqrt() as u64;
    return t1*tx;
}