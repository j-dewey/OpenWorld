use cgmath;
use cgmath::{InnerSpace, SquareMatrix}; // for vector3.normalize() and Matrix4::identity
use std::f32::consts::{FRAC_PI_2, PI};
use wgpu::util::DeviceExt;

use super::shader::Uniform;

/*
* Okay so this is a lot of linear algebra math
* And I don't understand it super well
* So uh, don't touch it unless you are absoloutley
* Sure what you are doing 
*/

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);
// i think this is like a clamp so the user can't look 180 degs
pub const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001; 

// a basic camera object
pub struct Camera{
    position: cgmath::Point3<f32>,
    pub yaw: cgmath::Rad<f32>,
    pub pitch: cgmath::Rad<f32>,
    projection: Projection
}

pub struct Projection {
    aspect: f32,
    fovy: cgmath::Rad<f32>,
    znear: f32,
    zfar: f32
}

// this is the object actually passed to the gpu
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4]
}

impl Camera{
    pub fn new(position: cgmath::Point3<f32>, yaw: f32, pitch: f32, width: u32, height: u32) -> Self{
        Self{
            position: position,
            yaw: cgmath::Rad(yaw),
            pitch: cgmath::Rad(pitch),
            // pi/4 is equal to 45 degrees
            // for znear, don't use too low a value or the depth buffer breaks.
            // thank U/ProPuke for saying this in a post on reddit
            projection: Projection::new(width, height, PI/4.0, 0.1, 100.0)
        }
    }

    pub fn calc_view(&self) -> cgmath::Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        cgmath::Matrix4::look_to_rh(
            self.position,
            cgmath::Vector3::new(
                cos_pitch * cos_yaw,
                sin_pitch,
                cos_pitch * sin_yaw
            ).normalize(),
            cgmath::Vector3::unit_y(),
        )
    }

    pub fn calc_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = self.calc_view();
        let proj = self.projection.calc_matrix();
        // fun fact: matrix multiplication isn't communative
        OPENGL_TO_WGPU_MATRIX * proj * view
    }

    pub fn create_uniform(&self) -> CameraUniform{
        let view = self.calc_view();
        let proj = self.projection.calc_matrix();
        let matrix: [[f32;4]; 4] = (OPENGL_TO_WGPU_MATRIX * view /* proj*/).into();
        println!("Yaw: {:?} \nPitch {:?} \nPosition {:?}", self.yaw, self.pitch, self.position);
        println!("Camera Matrix: {:?}", matrix);
        println!("Projection Matric: {:?}\n", proj);
        CameraUniform { 
            view_proj: self.calc_matrix().into()
        }
    }

    pub fn r#move(&mut self, movement: cgmath::Vector3<f32>){
        self.position += movement;
    }

    pub fn rotate(&mut self, dyaw: f32, dpitch: f32){
        self.yaw += cgmath::Rad(dyaw);
        // can't look 360 around vertical
        if self.pitch + cgmath::Rad(dpitch) < cgmath::Rad(SAFE_FRAC_PI_2) && self.pitch + cgmath::Rad(dpitch) > -cgmath::Rad(SAFE_FRAC_PI_2){
            self.pitch += cgmath::Rad(dpitch);
        }
    }

    pub fn set_pos(&mut self, new_pos: cgmath::Point3<f32>){
        self.position = new_pos;
    }
}

impl Projection {
    pub fn new( width: u32, height: u32, fovy: f32, znear: f32, zfar: f32 ) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: cgmath::Rad(fovy),
            znear,
            zfar
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> cgmath::Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * cgmath::perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }
}

impl CameraUniform{
    fn new() -> Self{
        Self { 
            view_proj: cgmath::Matrix4::identity().into()
        }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.calc_matrix().into();
    }

    pub fn get_bind_group(&self, device: &wgpu::Device) -> wgpu::BindGroup{
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                    label: Some("camera buffer"),
                    contents: bytemuck::cast_slice(
                        &self.view_proj
                    ),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                }
            );
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None
                }
            ],
            label: Some("camera_bind_group_layout")
        });
        // create the bind group
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        })
    }
}

impl Uniform for CameraUniform{
    fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None
                }
            ],
            label: Some("camera_bind_group_layout")
        })
    }
}