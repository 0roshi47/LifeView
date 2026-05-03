#import bevy_sprite::mesh2d_functions::{get_world_from_local, mesh2d_position_local_to_clip}

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) i_pos_size: vec3<f32>,
    @location(4) i_state: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) state: f32,
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
    out.state = v.i_state;
    return out;
}

fn lerp_color(state: f32) -> vec4<f32> {
    let c0 = vec4<f32>(0.001, 0.000, 0.014, 1.0);
    let c1 = vec4<f32>(0.243, 0.048, 0.424, 1.0);
    let c2 = vec4<f32>(0.524, 0.041, 0.468, 1.0);
    let c3 = vec4<f32>(0.734, 0.127, 0.290, 1.0);
    let c4 = vec4<f32>(0.929, 0.304, 0.122, 1.0);
    let c5 = vec4<f32>(0.987, 0.608, 0.089, 1.0);
    let c6 = vec4<f32>(0.988, 0.998, 0.645, 1.0);
    let colors = array<vec4<f32>, 7>(c0, c1, c2, c3, c4, c5, c6);
    let ratio = clamp(state, 0.0, 1.0) * 6.0;
    let low = u32(floor(ratio));
    let high = min(low + 1u, 6u);
    return mix(colors[low], colors[high], ratio - floor(ratio));
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return lerp_color(in.state);
}
