

use crossbeam::channel::{Receiver, Sender};
use crossbeam::*;
use rapier2d::parry::shape::Ball;
use crate::globals::*;
use crate::timer::Timer;
use crate::util::*;
use macroquad::prelude::*;
use macroquad::rand::*;
use nalgebra::Point2;
use rapier2d::{na::Vector2, prelude::*};
use std::collections::HashMap;
use std::f32::consts::PI;
use std::num::NonZeroUsize;
use std::ops::Bound;
use crate::dbg::MacroRapierDebugger;
use crate::physics_types::*;

pub struct Physics {
    pub rigid_bodies: RigidBodySet,
    pub colliders: ColliderSet,
    gravity: Vector2<f32>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    query_pipeline: QueryPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: (),
    debug_render_pipeline: DebugRenderPipeline,
    debug_renderer: MacroRapierDebugger,
    //event_handler: ChannelEventCollector,
    grav_time: Timer,
    pub types: PhysicsTypes,
    //pub types2: PhysicsTypes2,
}

impl Physics {

    pub fn new() -> Self {
        let solver_params = IntegrationParameters {
            max_ccd_substeps: 1,
            prediction_distance: 0.005,
            dt: 1.0/60.0,
            min_island_size: 32,
            allowed_linear_error: 0.004,
            num_solver_iterations: NonZeroUsize::new(1).unwrap(),
            num_additional_friction_iterations: 2,
            ..Default::default()
        };
        let dbg_cfg = DebugRenderStyle {
            collider_dynamic_color: [1.0, 0.0, 0.0, 1.0],
            collider_kinematic_color: [1.0, 1.0, 0.0, 1.0],
            impulse_joint_anchor_color: [0.0, 0.0, 1.0, 1.0],
            impulse_joint_separation_color: [0.0, 0.0, 1.0, 1.0],
            ..Default::default()
        };
        let dbg_mode = 
            //DebugRenderMode::COLLIDER_SHAPES | 
            DebugRenderMode::IMPULSE_JOINTS | 
            DebugRenderMode::JOINTS;
            //DebugRenderMode::SOLVER_CONTACTS;
        Self {
            rigid_bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            gravity: Vector2::new(0.0, 0.0),
            integration_parameters: solver_params,   //IntegrationParameters::default(),
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
            debug_render_pipeline: DebugRenderPipeline::new(dbg_cfg, dbg_mode),
            debug_renderer: MacroRapierDebugger,
            //event_handler: event_handler,
            grav_time: Timer::new(0.66, true, true, false),
            types: PhysicsTypes::random(),
            //types2: PhysicsTypes2::random(50),
        }
    }

    pub fn random_types(&mut self) {
        self.types = PhysicsTypes::random();
    }

    fn update_intersections(&mut self) {
        self.query_pipeline.update(&self.rigid_bodies, &self.colliders);
    }

    pub fn add_circle_body(&mut self, key: u64, position: &Vec2, radius: f32) -> RigidBodyHandle {
        let iso = Isometry::new(Vector2::new(position.x, position.y), 0.0);
        let rigid = RigidBodyBuilder::dynamic().position(iso)
            .linear_damping(0.0).angular_damping(1.0)
            .user_data(key as u128).build();
        let collider = ColliderBuilder::ball(radius).density(1.0).restitution(1.0).friction(0.0)
            .active_collision_types(ActiveCollisionTypes::DYNAMIC_DYNAMIC).active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        let rb_handle = self.rigid_bodies.insert(rigid);
        let _coll_handle = self.colliders.insert_with_parent(collider,rb_handle, &mut self.rigid_bodies);
        return rb_handle;
    }

    pub fn add_dynamic_rigidbody(&mut self, position: &Vec2, rotation: f32, linear_damping: f32, angular_damping: f32, p_type: u128) -> RigidBodyHandle {
        let pos = Isometry::new(Vector2::new(position.x, position.y), rotation);
        let dynamic_body = RigidBodyBuilder::dynamic().position(pos).can_sleep(true).ccd_enabled(false)
            .linear_damping(linear_damping).angular_damping(angular_damping)
            .sleeping(false).user_data(p_type).build();
        return self.rigid_bodies.insert(dynamic_body);
    }

    pub fn add_collider(&mut self, body_handle: RigidBodyHandle, rel_position: &Vec2, rotation: f32, shape: SharedShape, physics_props: PhysicsProperties) -> ColliderHandle {
        let mut collision_types = ActiveCollisionTypes::DYNAMIC_DYNAMIC;
        let settings = get_settings();
        if settings.collisions {
            collision_types = ActiveCollisionTypes::DYNAMIC_DYNAMIC;
        }
        let iso = make_isometry(rel_position.x, rel_position.y, rotation);
        let collider = match shape.shape_type() {
            ShapeType::Ball => {
                //let radius = shape.0.as_ball().unwrap().radius;
                ColliderBuilder::new(shape).position(iso).density(physics_props.density).friction(physics_props.friction).restitution(physics_props.restitution)
                    .active_collision_types(collision_types).active_events(ActiveEvents::empty())
                    .build()
            },
            ShapeType::ConvexPolygon => {
                ColliderBuilder::new(shape).density(physics_props.density).friction(physics_props.friction).restitution(physics_props.restitution)
                .active_collision_types(ActiveCollisionTypes::default()).active_events(ActiveEvents::COLLISION_EVENTS).build()
            },
            _ => {
                ColliderBuilder::ball(5.0).position(iso).build()
            },
        };
        return self.colliders.insert_with_parent(collider, body_handle, &mut self.rigid_bodies);
    }

    pub fn add_dynamic(&mut self, position: &Vec2, rotation: f32, shape: SharedShape, physics_props: PhysicsProperties, _random_vel: bool, p_type: u128) -> RigidBodyHandle {
        let rbh = self.add_dynamic_rigidbody(position, rotation, physics_props.linear_damping, physics_props.angular_damping, p_type);
        let _colh = self.add_collider(rbh, &Vec2::ZERO, 0.0, shape, physics_props);
        return rbh;
    }

    pub fn add_bound(&mut self, rbh1: RigidBodyHandle, rbh2: RigidBodyHandle) -> ImpulseJointHandle {
        let pos1 = self.get_object_position(rbh1).unwrap();
        let pos2 = self.get_object_position(rbh2).unwrap();
        let d1 = (pos2 - pos1);
        
        let bound = RopeJointBuilder::new(10.0)
            .local_anchor1(Point2::new(0.0, 0.0)).local_anchor2(Point2::new(d1.x, d1.y)).build();
        let bound_handle = self.impulse_joint_set.insert(rbh1, rbh2, bound, true);
        return bound_handle;
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

    pub fn get_physics_type(&self, id: u128) -> &PhysicsType {
        return self.types.get_type(id);
    }

    fn get_in_range(&self, rbh: RigidBodyHandle, pos: &Vec2, radius: f32) -> Vec<RigidBodyHandle> {
        let mut particles: Vec<RigidBodyHandle> = vec![];
        let field = Ball::new(radius);
        let location = make_isometry(pos.x, pos.y, 0.0);
        let filter = QueryFilter {
            flags: QueryFilterFlags::ONLY_DYNAMIC | QueryFilterFlags::EXCLUDE_SENSORS,
            groups: None,
            exclude_collider: None,
            exclude_rigid_body: Some(rbh),
            ..Default::default()
        };
        self.query_pipeline.intersections_with_shape(&self.rigid_bodies, &self.colliders, &location, &field, filter,
            |collided| {
                self.colliders.get(collided).inspect(|collider| {
                    collider.parent().inspect(|rbh2| {
                        particles.push(rbh2.clone());
                    });
                });
                return true;
        });
        return particles;
    }

    pub fn field_react(&mut self, position: Vec2, _size: f32, p_type: u128, handle: RigidBodyHandle) {
        let settings = get_settings();
        let particle_type0 = self.types.get_type(p_type);
        //let f0 = particle_type0.actions.get(p_type as usize).unwrap();
        let field_radius = particle_type0.get_field_range() * settings.field;
        //let radius = settings.field * 0.5 + settings.field * f0;
        let force = settings.force;
        let repel = settings.repel;
        let iso0 = make_isometry(position.x, position.y, 0.0);
        let field = Ball::new(field_radius);
        let filter = QueryFilter {
            flags: QueryFilterFlags::ONLY_DYNAMIC | QueryFilterFlags::EXCLUDE_SENSORS,
            groups: None,
            exclude_collider: None,
            exclude_rigid_body: Some(handle),
            ..Default::default()
        };
        let mut impulse = Vec2::ZERO;
        self.query_pipeline.intersections_with_shape(&self.rigid_bodies, &self.colliders, &iso0, &field, filter,
            |collided| {
                let particle1_collider = self.colliders.get(collided).unwrap();
                let particle1_handle = particle1_collider.parent().unwrap();
                let particle1 = self.rigid_bodies.get(particle1_handle).unwrap();
                let f = force;
                let t1 = particle1.user_data;
                let a = particle_type0.get_action(t1 as usize);
                let pos2 = matrix_to_vec2(particle1.position().translation);
                let dist = position.distance(pos2);
                let rel_dist = dist/field_radius;
                let mut scalar = 0.0;
                let vector = (pos2 - position).normalize_or_zero();
                if rel_dist > repel && rel_dist != 0.0 {
                    scalar = (f * a) / rel_dist;
                } else if rel_dist > repel/2.0 {
                    if !settings.repel_on {
                        scalar = 0.0;
                    } else {
                        scalar = -(f * a) / rel_dist/repel;
                    }
                } else {
                    scalar = -f;
                }
                impulse += vector * scalar;
                return true;
            },
        );
        let particle0 = self.rigid_bodies.get_mut(handle).unwrap();
        particle0.reset_forces(true);
        particle0.add_force(Vector2::new(impulse.x, impulse.y), true);
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
                position: Vec2::ZERO,
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

    pub fn debug_draw(&mut self) {
        self.debug_render_pipeline.render(
            &mut self.debug_renderer, 
            &self.rigid_bodies, 
            &self.colliders, 
            &self.impulse_joint_set, 
            &self.multibody_joint_set, 
            &self.narrow_phase
        );
    }
}



