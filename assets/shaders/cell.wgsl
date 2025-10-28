#import bevy_sprite::mesh2d_vertex_output::VertexOutput
// we can import items from shader modules in the assets folder with a quoted path
//#import "shaders/custom_material_import.wgsl"::COLOR_MULTIPLIER

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> cell_state: f32;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(cell_state, 0.0, 1.0, 1.0);;
}
