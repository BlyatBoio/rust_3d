use std::sync::{LazyLock, Mutex};

use nalgebra::{UnitQuaternion, Vector3};

use crate::draw_helpers;

pub static all_polygons: LazyLock<Mutex<Vec<Polygon>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub struct Camera{
    pos:[f32;3],
    direction:UnitQuaternion<f32>,
    raycasts:Vec<Raycast>,
    grid_resolution:f32,
    fov:f32,
}

pub struct Raycast{
    direction:UnitQuaternion<f32>,
    return_color:[u8;4]
}

pub struct Polygon{
    pos_1:[f32;3],
    pos_2:[f32;3],
    pos_3:[f32;3],
    color:[u8;4],
}

impl Polygon{
    pub fn new(pos_1:[f32;3], pos_2:[f32;3], pos_3:[f32;3], color:[u8;4]) -> Polygon{
        Polygon{ pos_1:pos_1, pos_2:pos_2, pos_3:pos_3, color:color }
    }
}

impl Camera{
    pub fn new() -> Camera{
        let mut cam = Camera{ 
            pos:[0.0, 0.0, 0.0],
            direction: UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
            raycasts:Vec::new(),
            grid_resolution:10.0,
            fov:120.0};
        
        let x_steps = (draw_helpers::WIDTH as f32 / cam.grid_resolution) as usize;
        let y_steps = (draw_helpers::HEIGHT as f32 / cam.grid_resolution) as usize;
        let fov_radians = cam.fov.to_radians();

        for i in 0..x_steps {
            for j in 0..y_steps {
                let x_offset = if x_steps > 1 {
                    (i as f32 / (x_steps as f32 - 1.0)) * fov_radians - fov_radians / 2.0
                } else {
                    0.0
                };
                let y_offset = if y_steps > 1 {
                    (j as f32 / (y_steps as f32 - 1.0)) * fov_radians - fov_radians / 2.0
                } else {
                    0.0
                };

                let ray_direction = cam.direction * UnitQuaternion::from_euler_angles(y_offset, x_offset, 0.0);
                cam.raycasts.push(Raycast{ direction: ray_direction, return_color:[0, 0, 0, 0] });
            }
        }

        return cam;
    }
    pub fn add_global_position(&mut self, pos:[f32;3]){
        self.pos[0] += pos[0];
        self.pos[1] += pos[1];
        self.pos[2] += pos[2];
    }
    pub fn rotate_by(&mut self, angle:UnitQuaternion<f32>){
        self.direction = UnitQuaternion::from_quaternion(self.direction.quaternion() + angle.quaternion());
        for i in 0..self.raycasts.len() {
            self.raycasts[i].direction = UnitQuaternion::from_quaternion(self.direction.quaternion() + angle.quaternion());
        }
    }
    pub fn add_local_position(&mut self, pos:[f32;3]){
        let global_vector = self.direction * Vector3::new(pos[0], pos[1], pos[2]);
        self.add_global_position([global_vector[0], global_vector[1], global_vector[2]]);
    }
    pub fn cast_rays(&mut self){
        let polys = all_polygons.lock().unwrap();
        for i in 0..self.raycasts.len() {
            let mut has_intersected = false;
            for i in 0..polys.len() {
                if ray_poly_intersect(&self.raycasts[i], &polys[i]) {
                    has_intersected = true;
                    self.raycasts[i].return_color = polys[i].color;
                }
            }
            if !has_intersected {
                self.raycasts[i].return_color = [0, 0, 0, 0];
            }
        }
    }
    pub fn update_scren(&mut self){
        let x_steps = (draw_helpers::WIDTH as f32 / self.grid_resolution) as u32;
        let square_size = self.grid_resolution as u32;

        for index in 0..self.raycasts.len() {
            let x = (index as u32 % x_steps) * square_size;
            let y = (index as u32 / x_steps) * square_size;
            draw_helpers::square(x, y, square_size);
        }
    }
}

pub fn ray_poly_intersect(raycast: &Raycast, polygon: &Polygon) -> bool {
    let origin = Vector3::new(0.0, 0.0, 0.0);
    let direction = raycast.direction * Vector3::new(0.0, 0.0, 1.0);

    let v0 = Vector3::new(polygon.pos_1[0], polygon.pos_1[1], polygon.pos_1[2]);
    let v1 = Vector3::new(polygon.pos_2[0], polygon.pos_2[1], polygon.pos_2[2]);
    let v2 = Vector3::new(polygon.pos_3[0], polygon.pos_3[1], polygon.pos_3[2]);

    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let h = direction.cross(&edge2);
    let a = edge1.dot(&h);

    const EPSILON: f32 = 1e-6;
    if a > -EPSILON && a < EPSILON {
        return false;
    }

    let f = 1.0 / a;
    let s = origin - v0;
    let u = f * s.dot(&h);

    if u < 0.0 || u > 1.0 {
        return false;
    }

    let q = s.cross(&edge1);
    let v = f * direction.dot(&q);

    if v < 0.0 || u + v > 1.0 {
        return false;
    }

    let t = f * edge2.dot(&q);
    t > EPSILON
}