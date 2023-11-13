use cgmath::Point3;
use crate::render::camera::Camera;
use std::f64::consts::FRAC_PI_8;

pub struct Player{
    camera: Camera,
    position: cgmath::Point3<f32>,
    speed: f32,
    rotation_speed: f32
}

impl Player{
    pub fn new(scrn_width: u32, scrn_height: u32) -> Self{
        let position = cgmath::Point3::new(0.0,10.0,-0.0);
        Self{
            camera: Camera::new(position, 0.0, 0.0, scrn_height, scrn_height),
            position: position,
            speed: 5.0,
            rotation_speed: (FRAC_PI_8 / 4.0f64) as f32 * 10.0
        }
    }

    pub fn handle_input(&mut self, movement: [i32; 3], rotation: [i32; 2], dt: f32){
        let forward = movement[0] as f32 * self.speed * dt;
        let strafe = movement[1] as f32 * self.speed * dt;
        let dyaw = rotation[0] as f32 * self.rotation_speed * dt;
        let dpitch = rotation[1] as f32 * self.rotation_speed * dt;
        let fly = movement[2] as f32 * dt * self.speed;
        self.camera.r#move(forward, fly, strafe);
        self.camera.rotate(dyaw, dpitch);
    }

    // getters
    pub fn get_camera_ref_mut(&mut self) -> &mut Camera{
        &mut self.camera
    }
}