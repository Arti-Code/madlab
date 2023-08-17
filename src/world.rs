//use crossbeam::channel::{Receiver, Sender};
//use crossbeam::*;
use crate::consts::*;
use crate::object::MolecularRules;
use crate::util::*;
use macroquad::prelude::*;
use macroquad::rand::*;
use nalgebra::Point2;
use rapier2d::{na::Vector2, prelude::*};
use std::collections::HashMap;
use std::f32::consts::PI;

pub struct World {
    pub molecular_rules: MolecularRules,
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
            molecular_rules: MolecularRules::new(9),
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

    pub fn add_circle_body(&mut self, key: u8, position: &Vec2, radius: f32) -> RigidBodyHandle {
        let iso = Isometry::new(Vector2::new(position.x, position.y), 0.0);
        let rigid = RigidBodyBuilder::dynamic().position(iso)
            .linear_damping(0.8).angular_damping(1.0)
            .user_data(key as u128).build();
        let collider = ColliderBuilder::ball(radius).density(1.0).restitution(0.0).friction(1.0)
            .active_collision_types(ActiveCollisionTypes::empty()).active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        let rb_handle = self.rigid_bodies.insert(rigid);
        let coll_handle = self.colliders.insert_with_parent(collider,rb_handle, &mut self.rigid_bodies);
        let field = ColliderBuilder::ball(radius*FIELD).density(0.0).sensor(true).build();
        self.colliders.insert_with_parent(field,rb_handle, &mut self.rigid_bodies);
        let object = self.rigid_bodies.get_mut(rb_handle).unwrap();
        let v = random_unit_vec2();
        //object.set_linvel(Vector2::new(v.x, v.y)*PARTICLE_SPEED, true);
        return rb_handle;
    }

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

    pub fn get_around(&mut self, agent_body_handle: RigidBodyHandle, is_negative: bool, is_neutrino: bool) {
        //let rbm = &mut self.rigid_bodies;
        let mut action = Vec2::ZERO;
        let size1 = 1.0;
        let field = FIELD * size1;
        let rb = self.rigid_bodies.get(agent_body_handle).unwrap();
        let type1 = rb.user_data as u8;
        //let mut collider1: &Collider = &ColliderBuilder::ball(1.).build(); 
        /* for c in rb.colliders().iter() {
            match self.colliders.get(*c) {
                None => {continue;},
                Some(c1) => if !c1.is_sensor() {
                    size1 = c1.shape().as_ball().unwrap().radius;
                },
                Some(_) => {continue;}
            };
        } */
        let pos1 = matric_to_vec2(rb.position().translation);
        let dist = f32::INFINITY;
        //let collider = ColliderBuilder::ball(32.0).build();
        let filter = QueryFilter {
            flags: QueryFilterFlags::ONLY_DYNAMIC | QueryFilterFlags::EXCLUDE_SENSORS,
            groups: None,
            exclude_rigid_body: Some(agent_body_handle),
            ..Default::default()
        };
        let mut handlers: Vec<RigidBodyHandle> = vec![];
        for c in rb.colliders() {
            let collider = self.colliders.get(*c).unwrap();
            if !collider.is_sensor() {
                continue;
            }
            self.query_pipeline.intersections_with_shape(&self.rigid_bodies, &self.colliders, rb.position(), collider.shape(), filter,
                |collided| {
                    let rb2 = self.get_body_handle_from_collider(collided);
                    match rb2 {
                        Some(handle) => {
                            match self.rigid_bodies.get(handle) {
                                Some(rigid2) => {
                                    let type2 = rigid2.user_data as u8;
                                    let pos2 = matric_to_vec2(rigid2.position().translation);
                                    let dist = pos2.distance(pos1);
                                    let rel_dist = dist/field;
                                    if rel_dist >= 0.5 {
                                        let dir = (pos2-pos1).normalize_or_zero();
                                        let act = self.molecular_rules.rules[type1 as usize].1[type2 as usize];
                                        action += dir * act * (size1 as f32) * GRAV/(rel_dist);
                                    } else {
                                        if rel_dist <= 0.1 {
                                            if self.impulse_joint_set.joints_between(agent_body_handle, handle).count() == 0 {
                                                handlers.push(handle);
                                            }
                                        } else if self.impulse_joint_set.joints_between(agent_body_handle, handle).count() == 0 {
                                            let dir = (pos2-pos1).normalize_or_zero();
                                            let act = self.molecular_rules.rules[type1 as usize].1[type2 as usize];
                                            action += -4. * (dir * act * size1 as f32 * GRAV/rel_dist);
                                        }
                                    }
                                },
                                None => {},
                            }
                        },
                        None => {},
                    }
                    return true;
                },
            );    
        }
        for h in handlers.iter() {
            let mut x = Vector::x_axis();
            if gen_range(0, 2) == 1 {
                x = Vector::y_axis();
            }
            let joint = PrismaticJointBuilder::new(x).limits([10.0, 20.0]).build();
            self.impulse_joint_set.insert(agent_body_handle, *h, joint, true);
            info!("JOINED!")
        }
        let rbm = self.rigid_bodies.get_mut(agent_body_handle).unwrap();
        rbm.reset_forces(true);
        rbm.add_force(Vector2::new(action.x, action.y), true);
    }
}

pub struct PhysicsData {
    pub position: Vec2,
    pub rotation: f32,
    pub mass: f32,
    pub kin_eng: Option<f32>,
}
