// For 2d replace `bevy_pbr::mesh_functions` with `bevy_sprite::mesh2d_functions`
// and `mesh_position_local_to_clip` with `mesh2d_position_local_to_clip`.
#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

struct ChunkMaterial {
    color: vec4<f32>,
};
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material: ChunkMaterial;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) vertex_pack: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) blend_color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    let packed = vertex.vertex_pack;
    let mask: u32 = (1u << 6u) - 1u; // 0x3Fu
    let x: f32 = f32(packed & mask);
    let y: f32 = f32((packed >> 6u) & mask);
    let z: f32 = f32((packed >> 12u) & mask);

    let local_position = vec4<f32>(x, y, z, 1.0);
    out.clip_position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        local_position,
    );
    out.blend_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    return out;
}

struct FragmentInput {
    @location(0) blend_color: vec4<f32>,
};

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    return input.blend_color;
}