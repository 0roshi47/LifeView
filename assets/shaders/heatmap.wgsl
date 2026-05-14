#import bevy_sprite::mesh2d_functions::{get_world_from_local, mesh2d_position_local_to_clip}

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@group(2) @binding(0) var grid_data: texture_2d<f32>;
@group(2) @binding(1) var grid_sampler: sampler;
@group(2) @binding(2) var gradient: texture_2d<f32>;
@group(2) @binding(3) var gradient_sampler: sampler;

@vertex
fn vertex(v: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh2d_position_local_to_clip(
        get_world_from_local(0u),
        vec4<f32>(v.position, 1.0)
    );
    out.uv = v.uv;
    return out;
}

fn catmull_rom(t: f32, p0: f32, p1: f32, p2: f32, p3: f32) -> f32 {
    return 0.5 * (2.0 * p1 + (-p0 + p2) * t
        + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t * t
        + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t * t * t);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let size = vec2<f32>(f32(textureDimensions(grid_data).x), f32(textureDimensions(grid_data).y));
    let gx = in.uv.x * size.x;
    let gy = in.uv.y * size.y;
    let ix = floor(gx);
    let iy = floor(gy);
    var fx = gx - ix;
    var fy = gy - iy;
    if (ix < 1.0 || ix > size.x - 2.0 || iy < 1.0 || iy > size.y - 2.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    var col: array<f32, 4>;
    for (var r = 0u; r < 4u; r = r + 1u) {
        var yy = iy + f32(r) - 1.0;
        var row: array<f32, 4>;
        for (var c = 0u; c < 4u; c = c + 1u) {
            var xx = ix + f32(c) - 1.0;
            let suv = vec2<f32>((xx + 0.5) / size.x, (yy + 0.5) / size.y);
            row[c] = textureSampleLevel(grid_data, grid_sampler, suv, 0.0).r;
        }
        col[r] = catmull_rom(fx, row[0], row[1], row[2], row[3]);
    }
    let val = catmull_rom(fy, col[0], col[1], col[2], col[3]);
    let color = textureSample(gradient, gradient_sampler, vec2<f32>(clamp(val, 0.0, 1.0), 0.5));
    return vec4<f32>(color.rgb, 1.0);
}
