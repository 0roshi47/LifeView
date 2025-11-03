#import bevy_pbr::{
    forward_io::VertexOutput,
    mesh_view_bindings::view,
    pbr_types::{STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT, PbrInput, pbr_input_new},
    pbr_functions as fns,
    pbr_bindings,
}
#import bevy_core_pipeline::tonemapping::tone_mapping

struct Cell {
	state: f32
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<storage> cells: vec<Cell>;

// struct VertexOutput {
//     @builtin(position) position: vec4<f32>,
//     @location(0) tag: u32,
// };

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(1.0/mesh.world_position.x, 1.0/mesh.world_position.y, 0.0, 1.0);
}
