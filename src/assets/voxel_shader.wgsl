// Vertex shader

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0) // 1.
var<uniform> camera: CameraUniform;


struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) block_id: u32,
    @location(3) direction: f32
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) direction: i32,
    @location(1) color: vec4<f32>
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    // make walls darker
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.direction = i32(model.direction);
    out.color = vec4<f32>(
        model.color[0]*model.direction, 
        model.color[1]*model.direction,
        model.color[2]*model.direction,
        1.0
    );
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}