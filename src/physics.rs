use std::sync::{LazyLock, Mutex, MutexGuard};

use nalgebra::{Norm, Quaternion, Unit, UnitQuaternion, Vector, Vector3, constraint};

pub static VERTECIES:LazyLock<Mutex<Vec<PhysVert>>> = LazyLock::new(|| Mutex::new(Vec::new()));
pub static CONSTRAINTS: LazyLock<Mutex<Vec<PhysConstraint>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[derive(PartialEq)]
pub struct PhysVert{
    mass:f32,
    pos:[f32;3],
    vel:[f32;3],
    acc:[f32;3],
    is_fixed:bool,
}

impl PhysVert{
    pub fn new_empty() -> PhysVert{
        PhysVert{
            mass:1.0, 
            pos:[0.0,0.0,0.0],
            vel:[0.0,0.0,0.0],
            acc:[0.0,0.0,0.0],
            is_fixed:false,
        }
    }
    pub fn new(mass:f32, pos:[f32;3], vel:[f32;3], acc:[f32;3], is_fixed:bool) -> PhysVert{
        PhysVert { mass:mass, pos:pos, vel:vel, acc:acc, is_fixed:is_fixed }
    }
    pub fn apply_force(&mut self, force_newtons:f32, direction:UnitQuaternion<f32>){
        let force_axis = direction.axis().unwrap();
        let force_accel = force_newtons / self.mass;
        self.acc[0] += force_accel * force_axis[0]; // Apply force in x direction
        self.acc[1] += force_accel * force_axis[1]; // Apply force in y direction
        self.acc[2] += force_accel * force_axis[2]; // Apply force in z direction
    }
    pub fn update(&mut self){
        // add acceleration to velocity
        self.vel[0] += self.acc[0];
        self.vel[1] += self.acc[1];
        self.vel[2] += self.acc[2];
        
        // add velocity to position
        self.pos[0] += self.vel[0];
        self.pos[1] += self.vel[1];
        self.pos[2] += self.vel[2];
    }
}

#[derive(PartialEq)]
pub enum PhysConstraint{
    ElasticConstraint { 
        vert_1:PhysVert,
        vert_2:PhysVert,
        spring_constant:f32,
        resting_length:f32,
        breaking_force:f32,
    },
    StaticConstraint { 
        vert_1:PhysVert,
        vert_2:PhysVert,
        resting_length:f32,
        breaking_force:f32,
    }
}

impl PhysConstraint{
    pub fn new_elastic(vert_1:PhysVert, vert_2:PhysVert, spring_constant:f32, resting_length:f32, breaking_force:f32){
        let new_const = PhysConstraint::ElasticConstraint { vert_1, vert_2, spring_constant, resting_length, breaking_force };
        CONSTRAINTS.lock().unwrap().push(new_const);
    }
    pub fn new_static(vert_1:PhysVert, vert_2:PhysVert, resting_length:f32, breaking_force:f32){
        let new_const = PhysConstraint::StaticConstraint { vert_1, vert_2, resting_length, breaking_force };
        CONSTRAINTS.lock().unwrap().push(new_const);
    }
}

pub fn break_constraint(constraint:&PhysConstraint){
    let mut constraints: MutexGuard<'_, Vec<PhysConstraint>> = CONSTRAINTS.lock().unwrap();
    for i in 0..constraints.len(){
        if &constraints[i] == constraint {
            constraints.remove(i);
            break;
        }
    }
}
pub fn get_quaternion_between(pos_1:[f32;3], pos_2:[f32;3]) -> UnitQuaternion<f32>{
    let pos_1_total = pos_1[0] + pos_1[1] + pos_1[2];
    let unit_pos_1 = Vector3::new(pos_1[0]/pos_1_total, pos_1[1]/pos_1_total, pos_1[2]/pos_1_total);
    
    let pos_2_total = pos_2[0] + pos_2[1] + pos_2[2];
    let unit_pos_2 = Vector3::new(pos_2[0]/pos_2_total, pos_2[1]/pos_2_total, pos_2[2]/pos_2_total);
    
    let cross = Unit::new_normalize(unit_pos_1.cross(&unit_pos_2));

    let theta = f32::acos(unit_pos_1.dot(&unit_pos_2));

    return UnitQuaternion::from_axis_angle(&cross, theta);
}

pub fn get_distance(pos_1:[f32;3], pos_2:[f32;3]) -> f32{
    f32::sqrt(
        f32::powi(pos_1[0] - pos_2[0], 2) +
        f32::powi(pos_1[1] - pos_2[1], 2) +
        f32::powi(pos_1[2] - pos_2[2], 2)
    )
}
pub fn update_constraint(constraint: &mut PhysConstraint){
    let mut do_break = false;
    match constraint{
        PhysConstraint::ElasticConstraint { vert_1, vert_2, spring_constant, resting_length, breaking_force } => {
            let delta_x = get_distance(vert_1.pos, vert_2.pos);
            let applied_force = (delta_x - *resting_length) * *spring_constant;

            if applied_force > *breaking_force {do_break = true;}

            if vert_1.is_fixed {
                if vert_2.is_fixed {
                    return;
                }
                else{
                    vert_2.apply_force(applied_force, get_quaternion_between(vert_1.pos, vert_2.pos));
                }
            }
            else if vert_2.is_fixed {
                vert_1.apply_force(applied_force, get_quaternion_between(vert_2.pos, vert_1.pos));
            }
            else {
                vert_1.apply_force(applied_force/2.0, get_quaternion_between(vert_2.pos, vert_1.pos));
                vert_2.apply_force(applied_force/2.0, get_quaternion_between(vert_1.pos, vert_2.pos));
            }
        }
        PhysConstraint::StaticConstraint { vert_1, vert_2, resting_length, breaking_force } => {
            
        }
    }
    if do_break { break_constraint(constraint); }
}