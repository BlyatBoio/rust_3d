use nalgebra::{Quaternion, UnitQuaternion, Vector3};

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
    pub fn new_elastic(vert_1:PhysVert, vert_2:PhysVert, spring_constant:f32, resting_length:f32, breaking_force:f32) -> PhysConstraint{
        PhysConstraint::ElasticConstraint { vert_1, vert_2, spring_constant, resting_length, breaking_force }
    }
    pub fn new_static(vert_1:PhysVert, vert_2:PhysVert, resting_length:f32, breaking_force:f32) -> PhysConstraint{
        PhysConstraint::StaticConstraint { vert_1, vert_2, resting_length, breaking_force }
    }
}

pub struct PhysObj{
    vertecies:Vec<PhysVert>,
    constraints:Vec<PhysConstraint>,
}

pub struct Simulation{
    vertecies:Vec<PhysVert>,
    constraints:Vec<PhysConstraint>,
    objects:Vec<PhysObj>,
}

impl Simulation{

}

pub fn get_distance(pos_1:[f32;3], pos_2:[f32;3]) -> f32{
    f32::sqrt(
        f32::powi(pos_1[0] - pos_2[0], 2) +
        f32::powi(pos_1[1] - pos_2[1], 2) +
        f32::powi(pos_1[2] - pos_2[2], 2)
    )
}
pub fn update_constraint(constraint: &mut PhysConstraint){
    match constraint{
        PhysConstraint::ElasticConstraint { vert_1, vert_2, spring_constant, resting_length, breaking_force } => {
            let delta_x = get_distance(vert_1.pos, vert_2.pos);
            let applied_force = (delta_x - *resting_length) * *spring_constant;
            if vert_1.is_fixed {
                if vert_2.is_fixed {
                    return;
                }
                else{

                }
            }
            else if vert_2.is_fixed {
                
            }
        }
        PhysConstraint::StaticConstraint { vert_1, vert_2, resting_length, breaking_force } => {
            
        }
    }
}