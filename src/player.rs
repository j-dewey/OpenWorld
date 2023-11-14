use cgmath::InnerSpace;
use crate::{render::{camera::Camera, voxel::VOXEL_WIDTH}, physics::{PhysicsData, PhysicsObject}, blocks::CHUNK_WIDTH};
use std::f64::consts::FRAC_PI_8;

pub struct Player{
    camera: Camera,
    position: cgmath::Point3<f32>,
    speed: f32,
    rotation_speed: f32,
    pd: PhysicsData,
    pub physics_on: bool
}

impl Player{
    pub fn new(scrn_width: u32, scrn_height: u32) -> Self{
        let position = cgmath::Point3::new(0.0,10.0,-0.0);
        let pd = PhysicsData{
            width: VOXEL_WIDTH,
            depth: VOXEL_WIDTH,
            height: VOXEL_WIDTH * 2.0,
            mass: 10.0,
            position: position,
            rotation: cgmath::Rad(0.0),
            force: cgmath::Vector3 { x: 0.0, y: 0.0, z: 0.0 }
        };
        Self{
            camera: Camera::new(position, 0.0, 0.0, scrn_height, scrn_height),
            position: position,
            speed: 5.0,
            rotation_speed: (FRAC_PI_8 / 4.0f64) as f32 * 10.0,
            pd,
            physics_on: false
        }
    }

    pub fn handle_input(&mut self, movement: [i32; 3], rotation: [i32; 2], dt: f32){
        let forward = movement[0] as f32 * self.speed * dt;
        let strafe = movement[1] as f32 * self.speed * dt;
        let dyaw = rotation[0] as f32 * self.rotation_speed * dt;
        let dpitch = rotation[1] as f32 * self.rotation_speed * dt;
        let fly = movement[2] as f32 * dt * self.speed;
        
        let (yaw_sin, yaw_cos) = self.camera.yaw.0.sin_cos();
        let forward_dir = cgmath::Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right_dir = cgmath::Vector3::new(-yaw_sin, 0.0, yaw_cos);
        
        let up_down = cgmath::Vector3::new(0.0, fly, 0.0);
        let d_forward = forward * forward_dir;
        let d_right = strafe * right_dir;
        let d_total = up_down + d_forward + d_right;

        self.camera.r#move(d_total);
        self.pd.r#move(d_total);
        self.camera.rotate(dyaw, dpitch);
    }

    // getters
    pub fn get_camera_ref_mut(&mut self) -> &mut Camera{
        &mut self.camera
    }
}

impl PhysicsObject for Player{
    fn get_data(&self) -> &PhysicsData { &self.pd }
    fn get_data_mut(&mut self) -> &mut PhysicsData { &mut self.pd }
    fn update_position(&mut self, new_position: cgmath::Point3<f32>) {
        self.pd.position = new_position;
        self.camera.set_pos(new_position);
    }
}