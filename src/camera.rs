use crate::globals::*;
use macroquad::prelude::*;

pub fn create_camera() -> Camera2D {
    let mut camera2d = Camera2D {
        zoom: Vec2 {
            x: ZOOM_RATE,
            y: SCREEN_RATIO * ZOOM_RATE,
        },
        target: Vec2 {
            x: 0.0,
            y: 0.0,
        },
        ..Default::default()
    };
    camera2d.offset.y *= -1.0;
    return camera2d;
}

pub fn control_camera(camera: &mut Camera2D /* , screen_ratio: f32 */) {
    if is_key_pressed(KeyCode::KpAdd) {
        camera.zoom += Vec2::new(ZOOM_RATE * 0.1, SCREEN_RATIO * ZOOM_RATE * 0.1);
    }
    if is_key_pressed(KeyCode::KpSubtract) {
        if camera.zoom.x > 0.0001 {
            camera.zoom -= Vec2::new(ZOOM_RATE * 0.1, SCREEN_RATIO * ZOOM_RATE * 0.1);
        }
    }
    if is_key_pressed(KeyCode::KpMultiply) {
        camera.zoom = Vec2::new(ZOOM_RATE, SCREEN_RATIO * ZOOM_RATE);
        camera.target = Vec2::ZERO;
    }
    if is_key_pressed(KeyCode::KpEnter) {
        //camera.zoom = Vec2::new(ZOOM_RATE, SCREEN_RATIO * ZOOM_RATE);
        camera.target = Vec2::ZERO;
    }
    if is_key_pressed(KeyCode::Left) {
        camera.target.x -= 100.0;
    }
    if is_key_pressed(KeyCode::Right) {
        camera.target.x += 100.0;
    }
    if is_key_pressed(KeyCode::Up) {
        camera.target.y += 100.0;
    }
    if is_key_pressed(KeyCode::Down) {
        camera.target.y -= 100.0;
    }
}
