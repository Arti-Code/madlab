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
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: (),
    //event_handler: ChannelEventCollector,
    grav_time: Timer,
    pub types: PhysicsTypes,
}

impl World {

    pub fn new() -> Self {
        //let (collision_send, collision_recv) = crossbeam::channel::unbounded();
        //let (contact_force_send, contact_force_recv) = crossbeam::channel::unbounded();
        //let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);
        let solver_params = IntegrationParameters {
            interleave_restitution_and_friction_resolution: false,
            max_ccd_substeps: 1,
            max_stabilization_iterations: 1,
            max_velocity_iterations: 1,
            prediction_distance: 0.001,
        
            //dt: 1./30.,
            ..Default::default()
        };

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
            //event_handler: event_handler,
            grav_time: Timer::new(0.66, true, true, false),
            types: PhysicsTypes::random(),
        }
    }

    pub fn random_types(&mut self) {
        self.types = PhysicsTypes::random();
    }

    fn update_grav(&mut self) {
        if self.grav_time.update(get_frame_time()) {

            let mut gravity_map: HashMap<RigidBodyHandle, Vec2> = HashMap::new();
            for (id1, body1) in self.rigid_bodies.iter() {
                let mut gforce = Vec2::ZERO;
                let pos1 = matrix_to_vec2(body1.position().translation);
                let coll1 = self.colliders.get(*body1.colliders().first().unwrap()).unwrap();
                let size1 = coll1.shape().as_ball().unwrap().radius;
                for (_, body2) in self.rigid_bodies.iter() { 
                    let pos2 = matrix_to_vec2(body2.position().translation);
                    let coll2 = self.colliders.get(*body2.colliders().first().unwrap()).unwrap();
                    let size2 = coll2.shape().as_ball().unwrap().radius;
                    let dir = (pos1-pos2).normalize_or_zero();
                    let dist = pos2.distance_squared(pos1);
                    gforce += GRAV*((size1+size2)/2.0)*dir/dist; 
                }
                gravity_map.insert(id1, gforce);
            }
            /* for (rbh, gforce) in gravity_map.iter() {
                let rb = self.rigid_bodies.get_mut(*rbh).unwrap();
                rb.reset_forces(true);
                rb.add_force(vector![gforce.x, gforce.y], true);
            } */
        }
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
        //let object = self.rigid_bodies.get_mut(rb_handle).unwrap();
        //let v = random_unit_vec2();
        //object.set_linvel(Vector2::new(v.x, v.y)*PARTICLE_SPEED, true);
        return rb_handle;
    }

    pub fn add_dynamic_rigidbody(&mut self, position: &Vec2, rotation: f32, linear_damping: f32, angular_damping: f32, p_type: u128) -> RigidBodyHandle {
        let pos = Isometry::new(Vector2::new(position.x, position.y), rotation);
        let dynamic_body = RigidBodyBuilder::dynamic().position(pos)
            .linear_damping(linear_damping).angular_damping(angular_damping)
            .sleeping(false).user_data(p_type).build();
        return self.rigid_bodies.insert(dynamic_body);
    }

    pub fn add_collider(&mut self, body_handle: RigidBodyHandle, rel_position: &Vec2, rotation: f32, shape: SharedShape, physics_props: PhysicsProperties) -> ColliderHandle {
        let iso = make_isometry(rel_position.x, rel_position.y, rotation);
        let collider = match shape.shape_type() {
            ShapeType::Ball => {
                //let radius = shape.0.as_ball().unwrap().radius;
                ColliderBuilder::new(shape).position(iso).density(physics_props.density).friction(physics_props.friction).restitution(physics_props.restitution)
                    .active_collision_types(ActiveCollisionTypes::empty()).active_events(ActiveEvents::empty())
                    //.active_collision_types(ActiveCollisionTypes::DYNAMIC_DYNAMIC).active_events(ActiveEvents::empty())
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

    pub fn add_dynamic(&mut self, position: &Vec2, rotation: f32, shape: SharedShape, physics_props: PhysicsProperties, random_vel: bool, p_type: u128) -> RigidBodyHandle {
        let rbh = self.add_dynamic_rigidbody(position, rotation, physics_props.linear_damping, physics_props.angular_damping, p_type);
        let _colh = self.add_collider(rbh, &Vec2::ZERO, 0.0, shape, physics_props);
        if random_vel {
            let object = self.rigid_bodies.get_mut(rbh).unwrap();
            let v = random_unit_vec2();
            object.set_linvel(Vector2::new(v.x, v.y)*ELEMENT_SPEED, true);
        }
        return rbh;
    }

    pub fn add_motor(&mut self, rbh1: RigidBodyHandle, rbh2: RigidBodyHandle) -> ImpulseJointHandle {
        let motor = RevoluteJointBuilder::new().motor_velocity(10000.0, 0.1).contacts_enabled(false)
            .local_anchor1(Point2::new(-15.0, 0.0)).local_anchor2(Point2::new(-30.0, 0.0))
            .limits([0.0, 0.0]).motor_model(MotorModel::AccelerationBased).build();
        //motor.data.as_prismatic_mut().unwrap().set_limits([64.0, 72.0]);
        //motor.data.as_revolute_mut().unwrap().motor_velocity(1.0, 0.1).contacts_enabled(false).limits([2.0*PI, 2.0*PI]);
        let motor_handle = self.impulse_joint_set.insert(rbh1, rbh2, motor, true);
        return motor_handle;
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

    pub fn field_react(&mut self, position: Vec2, size: f32, p_type: u128, handle: RigidBodyHandle) {
        let settings = get_settings();
        let radius = settings.field * size;
        let force = settings.force;
        let repel = settings.repel;
        let iso0 = make_isometry(position.x, position.y, 0.0);
        let field = Ball::new(radius);
        let filter = QueryFilter {
            flags: QueryFilterFlags::ONLY_DYNAMIC | QueryFilterFlags::EXCLUDE_SENSORS,
            groups: None,
            exclude_collider: None,
            exclude_rigid_body: Some(handle),
            ..Default::default()
        };
        let particle_type = self.types.types.get(&p_type).unwrap();
        let mut impulse = Vec2::ZERO;
        self.query_pipeline.intersections_with_shape(&self.rigid_bodies, &self.colliders, &iso0, &field, filter,
            |collided| {
                let particle1_collider = self.colliders.get(collided).unwrap();
                let particle1_handle = particle1_collider.parent().unwrap();
                let particle1 = self.rigid_bodies.get(particle1_handle).unwrap();
                //let size1 = particle1_collider.shape().as_ball().unwrap().radius;
                //let f = force * (size + size1)/2.0;
                let f = force;
                let t1 = particle1.user_data;
                let a = particle_type.actions[t1 as usize];
                let pos2 = matrix_to_vec2(particle1.position().translation);
                let dist = position.distance(pos2);
                let rel_dist = dist/radius;
                let mut scalar = 0.0;
                let vector = (pos2 - position).normalize_or_zero();
                if rel_dist >= repel {
                    scalar = (f * a) / (dist).powi(2);
                } else if rel_dist < repel && rel_dist != 0.0 {
                    scalar = -(f * a.abs()) / (dist.powi(2));
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
        //let (collision_send, collision_recv) = crossbeam::channel::unbounded();
        //let (contact_force_send, contact_force_recv) = crossbeam::channel::unbounded();
        //let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);
        //self.update_grav();
        
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

        //while let Ok(collision_event) = collision_recv.try_recv() {
            // Handle the collision event.
            //println!("Received collision event: {:?}", collision_event);
        //}
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

    
}

pub struct PhysicsData {
    pub position: Vec2,
    pub rotation: f32,
    pub mass: f32,
    pub kin_eng: Option<f32>,
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

pub struct PhysicsType {
    pub type_id: u128,
    pub actions: [f32; TYPES_NUM as usize],
    pub color: Color,
}

impl PhysicsType {
    pub fn new(type_id: u128, color: Color) -> Self {
        let mut actions: [f32; TYPES_NUM as usize] = [0.0; TYPES_NUM as usize];
        for i in 0..TYPES_NUM as usize {
            let a = rand::gen_range(-1.0, 1.0);
            actions[i] = a;
        }
        Self { type_id, actions, color }
    }
}

pub struct PhysicsTypes {
    pub types: HashMap<u128, PhysicsType>,
}

impl PhysicsTypes {
    pub fn random() -> Self {
        let mut types: HashMap<u128, PhysicsType> = HashMap::new();
        let colors = vec![RED, GREEN, BLUE, YELLOW, ORANGE, MAGENTA, DARKGREEN, PURPLE, PINK, VIOLET, DARKBLUE, WHITE, SKYBLUE, LIME, DARKPURPLE, BROWN, DARKBROWN, DARKGRAY, LIGHTGRAY ];
        for n in 0..colors.len() {
            //let action: f32 = rand::gen_range(-1.0, 1.0);
            let type_id = n as u128;
            let color = colors[n];
            let t = PhysicsType::new(type_id, color);
            types.insert(type_id, t);
        }
        Self { types }
    }
}