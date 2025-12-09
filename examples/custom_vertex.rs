use bevy::{
    asset::RenderAssetUsages,
    camera::primitives::Aabb,
    mesh::{Indices, MeshVertexAttribute, MeshVertexBufferLayoutRef, PrimitiveTopology},
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::render_resource::{
        AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError, VertexFormat,
    },
    shader::ShaderRef,
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<ChunkMaterial>::default())
        .add_systems(Startup, setup)
        .run()
}

#[derive(Resource, Reflect)]
pub struct GlobalChunkMaterial(pub Handle<ChunkMaterial>);

// A "high" random id should be used for custom attributes to ensure consistent sorting and avoid collisions with other attributes.
// See the MeshVertexAttribute docs for more info.
pub const ATTRIBUTE_VOXEL: MeshVertexAttribute =
    MeshVertexAttribute::new("Voxel", 988540919, VertexFormat::Uint32);

// This is the struct that will be passed to your shader
#[derive(Asset, Reflect, AsBindGroup, Debug, Clone)]
pub struct ChunkMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material for ChunkMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/chunk.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/chunk.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }

    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout
            .0
            .get_layout(&[ATTRIBUTE_VOXEL.at_shader_location(0)])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

fn setup(
    mut commands: Commands,
    mut chunk_materials: ResMut<Assets<ChunkMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Create and save a handle to the mesh.
    let cube_mesh_handle: Handle<Mesh> = meshes.add(create_cube_mesh());

    // Render the mesh with the custom texture, and add the marker.
    commands.spawn((
        Aabb::from_min_max(Vec3::ZERO, Vec3::splat(32.0)),
        Mesh3d(cube_mesh_handle),
        MeshMaterial3d(chunk_materials.add(ChunkMaterial {
            color: LinearRgba::WHITE,
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn create_cube_mesh() -> Mesh {
    // Keep the mesh data accessible in future frames to be able to mutate it in toggle_texture.
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        ATTRIBUTE_VOXEL,
        vec![
            vertex_pack(0, 1, 0),
            vertex_pack(1, 1, 0),
            vertex_pack(1, 1, 1),
        ],
    )
    .with_inserted_indices(Indices::U32(vec![0, 2, 1]))
}

fn vertex_pack(x: u32, y: u32, z: u32) -> u32 {
    pack3_u6(x, y, z)
}

fn pack3_u6(a: u32, b: u32, c: u32) -> u32 {
    assert!(a <= 32 && b <= 32 && c <= 32, "values must be <= 32");
    let mask = 0x3F; // 6 bits = 0b11_1111 = 63
    (a & mask) | ((b & mask) << 6) | ((c & mask) << 12)
}
