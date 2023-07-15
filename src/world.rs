//use crossbeam::channel::{Receiver, Sender};
//use crossbeam::*;
use crate::consts::*;
use crate::util::*;
use macroquad::prelude::*;
use nalgebra::Point2;
use rapier2d::{na::Vector2, prelude::*};
use std::collections::HashMap;
use std::f32::consts::PI;

pub struct World {
    pub rigid_bodies: RigidBodySet,
    pub colliders: ColliderSet,
    gravity: Vector2<f32>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    query_pipeline: QueryPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: (),
}

impl World {
    pub fn new() -> Self {
        //let (collision_send, collision_recv) = crossbeam::channel::unbounded();
        //let (contact_force_send, contact_force_recv) = crossbeam::channel::unbounded();
        //let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);
        Self {
            rigid_bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            gravity: Vector2::new(0.0, 0.0),
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            query_pipeline: QueryPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_hooks: (),
            event_handler: (),
        }
    }

    /* fn update_intersections(&mut self) {
        self.query_pipeline.update(&self.rigid_bodies, &self.colliders);
    } */

    pub fn add_circle_body(&mut self, key: u64, position: &Vec2, radius: f32) -> RigidBodyHandle {
        let iso = Isometry::new(Vector2::new(position.x, position.y), 0.0);
        let rigid = RigidBodyBuilder::dynamic().position(iso)
            .linear_damping(0.0).angular_damping(0.0)
            .user_data(key as u128).build();
        let mut collider = ColliderBuilder::ball(radius).density(1.0).restitution(1.0).friction(0.0)
            .active_collision_types(ActiveCollisionTypes::default()).active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        let rb_handle = self.rigid_bodies.insert(rigid);
        let coll_handle = self.colliders.insert_with_parent(collider,rb_handle, &mut self.rigid_bodies);
        let object = self.rigid_bodies.get_mut(rb_handle).unwrap();
        let v = random_unit_vec2();
        object.set_linvel(Vector2::new(v.x, v.y)*PARTICLE_SPEED, true);
        return rb_handle;
    }

    pub fn add_poly_body(&mut self, key: u64, position: &Vec2, points: Vec<Point2<f32>>) -> RigidBodyHandle {
        let iso = Isometry::new(Vector2::new(position.x, position.y), 0.0);
        let poly = RigidBodyBuilder::dynamic().position(iso)
            .linear_damping(0.0).angular_damping(0.0).can_sleep(true)
            .user_data(key as u128).build();
        let collider = ColliderBuilder::convex_polyline(points).unwrap()
            .restitution(0.5).friction(0.6)
            .active_collision_types(ActiveCollisionTypes::default() | ActiveCollisionTypes::DYNAMIC_DYNAMIC, )
            .active_events(ActiveEvents::COLLISION_EVENTS).density(1.0).build();
        let rb_handle = self.rigid_bodies.insert(poly);
        let coll_handle = self.colliders.insert_with_parent(collider, rb_handle, &mut self.rigid_bodies);
        let obj = self.rigid_bodies.get_mut(rb_handle).unwrap();
        //obj.add
        let imp = Vector2::new(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0))*1.0;
        //obj.apply_impulse(imp, true);
        obj.set_linvel(imp, true);
        return rb_handle;
    }

    pub fn add_jet_hull(&mut self, key: u64, position: &Vec2, points: Vec<Point2<f32>>) -> RigidBodyHandle {
        let iso = Isometry::new(Vector2::new(position.x, position.y), 0.0);
        let poly = RigidBodyBuilder::dynamic().position(iso)
            .linear_damping(0.05).angular_damping(0.5).can_sleep(false)
            .user_data(key as u128).build();
        let collider = ColliderBuilder::convex_polyline(points).expect("cant build hull collider")
            .active_collision_types(ActiveCollisionTypes::default() | ActiveCollisionTypes::DYNAMIC_DYNAMIC)
            .active_events(ActiveEvents::COLLISION_EVENTS).density(1.0).build();
        let rb_handle = self.rigid_bodies.insert(poly);
        let coll_handle = self.colliders.insert_with_parent(collider, rb_handle, &mut self.rigid_bodies);
        let obj = self.rigid_bodies.get_mut(rb_handle).expect("can't get mut hull collider");
        return rb_handle;
    }

    /* fn get_object_handle_from_collider(&self, collider_handle: ColliderHandle) -> Option<RigidBodyHandle> {
        let collider = self.colliders.get(collider_handle);
        match collider {
            None => {
                return None;
            },
            Some(collider) => {
                return collider.parent();
            }
        }
    } */

    pub fn remove_physics_object(&mut self, body_handle: RigidBodyHandle) {
        _ = self.rigid_bodies.remove(body_handle, &mut self.island_manager, &mut self.colliders, &mut self.impulse_joint_set, &mut self.multibody_joint_set, true, );
    }

    pub fn get_total_kinetic_eng(&self) -> f32 {
        let mut eng: f32 = 0.0;
        for (_, rb) in self.rigid_bodies.iter() {
            eng += rb.kinetic_energy();
        }
        return eng;
    }

    pub fn get_physics_obj_num(&self) -> usize {
        let body_num = self.rigid_bodies.len();
        return body_num;
    }

    pub fn step_physics(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_bodies,
            &mut self.colliders,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &self.physics_hooks,
            &self.event_handler,
        );
    }

    fn iso_to_vec2_rot(&self, isometry: &Isometry<Real>) -> (Vec2, f32) {
        let pos = Vec2::new(isometry.translation.x, isometry.translation.y);
        let rot = isometry.rotation.angle() + PI;
        return (pos, rot);
    }

    pub fn get_physics_data(&self, handle: RigidBodyHandle) -> PhysicsData {
        return if let Some(rb) = self.rigid_bodies.get(handle) {
            //.expect("handle to non-existent rigid body");
            let iso = rb.position();
            let (pos, rot) = self.iso_to_vec2_rot(iso);
            let data = PhysicsData {
                position: pos,
                rotation: rot,
                mass: rb.mass(),
                kin_eng: Some(rb.kinetic_energy()),
            };
            data
        } else {
            PhysicsData {
                position: Vec2::new(WORLD_W / 2., WORLD_H / 2.),
                rotation: 0.0,
                mass: 0.0,
                kin_eng: Some(0.0),
            }
        }
    }

    pub fn get_object_position(&self, handle: RigidBodyHandle) -> Option<Vec2> {
        let rb = self.rigid_bodies.get(handle);
        return match rb {
            Some(body) => {
                let pos = Vec2::new(body.position().translation.x, body.position().translation.y);
                Some(pos)
            }
            None => {
                None
            }
        }
    }

    fn get_body_handle_from_collider(&self, collider_handle: ColliderHandle) -> Option<RigidBodyHandle> {
        let collider: &Collider;
        match self.colliders.get(collider_handle) {
            Some(col) => {
                collider = col;
            }
            None => {
                return None;
            }
        }
        return match collider.parent() {
            Some(rbh) => {
                Some(rbh)
            }
            None => {
                None
            }
        }
    }

    pub fn get_around(&mut self, agent_body_handle: RigidBodyHandle) {
        //let rbm = &mut self.rigid_bodies;
        let mut action = Vec2::ZERO;
        let rb = self.rigid_bodies.get(agent_body_handle).unwrap();
        let pos1 = matric_to_vec2(rb.position().translation);
        let dist = f32::INFINITY;
        //let collider = ColliderBuilder::ball(32.0).build();
        let filter = QueryFilter {
            flags: QueryFilterFlags::ONLY_DYNAMIC | QueryFilterFlags::EXCLUDE_SENSORS,
            groups: None,
            exclude_rigid_body: Some(agent_body_handle),
            ..Default::default()
        };
        for c in rb.colliders() {
            let collider = self.colliders.get(*c).unwrap();
            if !collider.is_sensor() {
                continue;
            }
            self.query_pipeline.intersections_with_shape(&self.rigid_bodies, &self.colliders, rb.position(), collider.shape(), filter,
                |collided| {
                    return true;
                },
            );
        }
        //let rbm = self.rigid_bodies.get_mut(agent_body_handle).unwrap();
        //rbm.apply_impulse(Vector2::new(action.x, action.y), true);
    }
}

pub struct PhysicsData {
    pub position: Vec2,
    pub rotation: f32,
    pub mass: f32,
    pub kin_eng: Option<f32>,
}
