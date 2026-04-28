use std::sync::{LazyLock, Mutex};

use nalgebra::{Unit, UnitQuaternion, UnitVector3, Vector, Vector3};

use crate::draw_helpers;

pub static ALL_POLYGONS: LazyLock<Mutex<Vec<Polygon>>> = LazyLock::new(|| Mutex::new(Vec::new()));

// Constants
pub const FORWARD_VECTOR:Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);
pub const BACKGROUND_COLOR:[u8;4] = [100, 100, 100, 255];
pub const RAY_DID_NOT_HIT:RaycastIntersectInfo = RaycastIntersectInfo{did_hit:false, color:BACKGROUND_COLOR, u:0.0, v:0.0, t:0.0};
pub const EPSILON: f32 = 1e-6;

pub struct Camera{
    pos:Vector3<f32>,
    direction:UnitQuaternion<f32>, // forward facing direction
    raycasts:Vec<Raycast>, // raycast instances itterated over every frame
    grid_resolution:f32, // size of squares rendered by each raycasts
    fov:f32, // Field Of View
}

pub struct Raycast{
    local_direction:Vector3<f32>,
    return_color:[u8;4],
    screen_relative_pos:[u32;2]
}

pub struct RaycastIntersectInfo{
    did_hit:bool,
    color:[u8;4],
    u:f32,
    v:f32,
    t:f32,
}

pub struct Polygon{
    pos_1:Vector3<f32>,
    pos_2:Vector3<f32>,
    pos_3:Vector3<f32>,
    edge_1:Vector3<f32>,
    edge_2:Vector3<f32>,
    normal:Vector3<f32>,
    color:[u8;4],
}

impl Polygon{
    // Define a new polygon
    pub fn new(pos_1:Vector3<f32>, pos_2:Vector3<f32>, pos_3:Vector3<f32>, color:[u8;4]){
        let edge_1 = pos_2 - pos_1;
        let edge_2 = pos_3 - pos_1;
        let normal = edge_1.cross(&edge_2);
        ALL_POLYGONS.lock().unwrap().push(Polygon{ pos_1, pos_2, pos_3, edge_1, edge_2, normal, color });
    }
}

pub fn create_cube(x:f32, y:f32, z:f32, w:f32, h:f32, l:f32){
    // top face (y = y)
    Polygon::new(Vector3::new(x, y, z), Vector3::new(x+w, y, z), Vector3::new(x, y, z+l), [0, 255, 0, 255]);
    Polygon::new(Vector3::new(x, y, z+l), Vector3::new(x+w, y, z), Vector3::new(x+w, y, z+l), [0, 255, 0, 255]);

    // bottom face (y = y+h)
    Polygon::new(Vector3::new(x, y+h, z), Vector3::new(x+w, y+h, z), Vector3::new(x, y+h, z+l), [0, 255, 0, 255]);
    Polygon::new(Vector3::new(x, y+h, z+l), Vector3::new(x+w, y+h, z), Vector3::new(x+w, y+h, z+l), [0, 255, 0, 255]);

    // front face (z = z)
    Polygon::new(Vector3::new(x, y, z), Vector3::new(x+w, y, z), Vector3::new(x, y+h, z), [0, 255, 0, 255]);
    Polygon::new(Vector3::new(x, y+h, z), Vector3::new(x+w, y, z), Vector3::new(x+w, y+h, z), [0, 255, 0, 255]);

    // back face (z = z+l)
    Polygon::new(Vector3::new(x, y, z+l), Vector3::new(x, y+h, z+l), Vector3::new(x+w, y, z+l), [0, 255, 0, 255]);
    Polygon::new(Vector3::new(x+w, y, z+l), Vector3::new(x, y+h, z+l), Vector3::new(x+w, y+h, z+l), [0, 255, 0, 255]);

    // left face (x = x)
    Polygon::new(Vector3::new(x, y, z), Vector3::new(x, y+h, z), Vector3::new(x, y, z+l), [0, 255, 0, 255]);
    Polygon::new(Vector3::new(x, y, z+l), Vector3::new(x, y+h, z), Vector3::new(x, y+h, z+l), [0, 255, 0, 255]);

    // right face (x = x+w)
    Polygon::new(Vector3::new(x+w, y, z), Vector3::new(x+w, y, z+l), Vector3::new(x+w, y+h, z), [0, 255, 0, 255]);
    Polygon::new(Vector3::new(x+w, y+h, z), Vector3::new(x+w, y, z+l), Vector3::new(x+w, y+h, z+l), [0, 255, 0, 255]);

}

impl Camera{
    pub fn new() -> Camera{
        // define new camera
        let mut cam = Camera{ 
            pos:Vector3::new(0.0, 0.0, 0.0),
            direction: UnitQuaternion::from_axis_angle(&Unit::new_normalize(FORWARD_VECTOR), 0.0),
            raycasts:Vec::new(),
            grid_resolution:10.0,
            fov:120.0};
        
        // get the width & height of the grid of raycasts
        let x_steps = draw_helpers::WIDTH as f32 / cam.grid_resolution;
        let y_steps = draw_helpers::HEIGHT as f32 / cam.grid_resolution;

        let fov_rad = cam.fov * std::f32::consts::PI / 180.0;
        let tan_half_fov = (fov_rad / 2.0).tan();

        for i in 0..x_steps as i32 {
            for j in 0..y_steps as i32 {
                // get position 0-1 for the x and y location of the raycast
                let u = i as f32 / x_steps;
                let v = j as f32 / y_steps;
                let x_ndc = u * 2.0 - 1.0;
                let y_ndc = v * 2.0 - 1.0;
                // Adjust for FOV Angle
                let x_image = x_ndc * tan_half_fov;
                let y_image = y_ndc * tan_half_fov;
                // get direction offset from camera fwd
                let local_direction = Vector3::new(x_image, y_image, 1.0).normalize();
                // append new raycast
                cam.raycasts.push(Raycast{ local_direction, return_color:BACKGROUND_COLOR, screen_relative_pos:[(i as f32 *cam.grid_resolution) as u32, (j as f32 *cam.grid_resolution) as u32] });
            }
        }

        return cam;
    }
    pub fn add_global_position(&mut self, pos:[f32;3]){
        self.pos[0] += pos[0];
        self.pos[1] += pos[1];
        self.pos[2] += pos[2];
    }
    pub fn rotate_by(&mut self, x_rotation:f32, y_rotation:f32, z_rotation:f32){
        let x_axis = UnitQuaternion::from_axis_angle(&Unit::new_normalize(self.direction.transform_vector(&Vector3::y_axis())), x_rotation);
        let y_axis = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), y_rotation);
        let z_axis = UnitQuaternion::from_axis_angle(&Unit::new_normalize(self.direction.transform_vector(&Vector3::z_axis())), z_rotation);
    
        self.direction = self.direction * x_axis * y_axis * z_axis
    }
    pub fn add_local_position(&mut self, pos:[f32;3]){
        let global_vector = self.direction * Vector3::new(pos[0], pos[1], pos[2]);
        self.add_global_position([global_vector[0], global_vector[1], global_vector[2]]);
    }
    pub fn cast_rays(&mut self){
        // access polygons array
        let polys = ALL_POLYGONS.lock().unwrap();
        for i in 0..self.raycasts.len() {
            // per raycast checks
            let mut has_intersected = false;
            let mut min_t = 100000000.0;

            for j in 0..polys.len() {
                // get the ray colision info for the polygon
                let ray_info = ray_poly_intersect(self.pos, &self.raycasts[i], self.direction, &polys[j]);
                
                // If it colides and the distance is less than the current closest distance
                if ray_info.did_hit && f32::abs(ray_info.t) < min_t {
                    // update current closest distance
                    min_t = f32::abs(ray_info.t);
                    has_intersected = true;

                    // If it is on a border, draw the stroke color
                    if ray_info.u < 0.05 || ray_info.v < 0.05 || ray_info.u+ray_info.v > 0.9 {
                        self.raycasts[i].return_color = [0, 0, 0, 255];
                    }
                    else {
                        // If it is not a border, make the return color the color of the polygon
                        self.raycasts[i].return_color = polys[j].color;
                    }
                }
            }

            // If it didnt hit anything, make the color the background color
            if !has_intersected {self.raycasts[i].return_color = BACKGROUND_COLOR;}
        }
    }
    pub fn update_screen(&mut self){
        for i in 0..self.raycasts.len() {
            // fill the square with the color of the raycast
            if unsafe{draw_helpers::fill_color} != self.raycasts[i].return_color {
                draw_helpers::fill(self.raycasts[i].return_color);
            }
            // draw the square
            draw_helpers::square(self.raycasts[i].screen_relative_pos[0], self.raycasts[i].screen_relative_pos[1], self.grid_resolution as u32);
        }
    }
}

pub fn ray_poly_intersect(origin: Vector3<f32>, raycast: &Raycast, camera_direction: UnitQuaternion<f32>, polygon: &Polygon) -> RaycastIntersectInfo {
    
    // get combined direction of camera & raycast
    let direction = (camera_direction * raycast.local_direction).normalize();

    let h = direction.cross(&polygon.edge_2);
    let a = polygon.edge_1.dot(&h);

    if a > -EPSILON && a < EPSILON {
        return RAY_DID_NOT_HIT;
    }

    // Get UV Coordinates and if they are < 0 or > 1, then the intersection would not be on the triangle

    let f = 1.0 / a;
    let s = origin - polygon.pos_1;
    let u = f * s.dot(&h);

    if u < 0.0 || u > 1.0 {
        return RAY_DID_NOT_HIT;
    }

    let q = s.cross(&polygon.edge_1);
    let v = f * direction.dot(&q);

    if v < 0.0 || u + v > 1.0 {
        return RAY_DID_NOT_HIT;
    }

    // if t is negative, then the polygon is facing away from the raycast and thus either backwards or behind the camera
    let t = f * polygon.edge_2.dot(&q);
    if t > EPSILON {
        return RaycastIntersectInfo { did_hit: true, color: polygon.color, u, v, t }
    }
    else {
        return RAY_DID_NOT_HIT
    }
}