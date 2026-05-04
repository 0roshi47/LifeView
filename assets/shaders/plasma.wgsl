#import bevy_sprite::mesh2d_functions::{get_world_from_local, mesh2d_position_local_to_clip}

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) i_pos_size: vec3<f32>,
    @location(4) i_rgb: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) r: f32,
    @location(1) g: f32,
    @location(2) b: f32,
};

@vertex
fn vertex(v: Vertex) -> VertexOutput {
    let world_pos = vec3<f32>(
        v.position.x * v.i_pos_size.z + v.i_pos_size.x,
        v.position.y * v.i_pos_size.z + v.i_pos_size.y,
        0.0
    );
    var out: VertexOutput;
    out.clip_position = mesh2d_position_local_to_clip(
        get_world_from_local(0u),
        vec4<f32>(world_pos, 1.0)
    );
    out.r = v.i_rgb.x;
    out.g = v.i_rgb.y;
    out.b = v.i_rgb.z;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.r, in.g, in.b, 1.0);
}
