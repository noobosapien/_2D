use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{ mesh::{ MeshVertexAttribute, MeshVertexBufferLayoutRef }, render_resource::* },
    sprite::{ Material2d, Material2dKey, Material2dPlugin },
};

const SHADER_ASSET_PATH: &str = "shaders/custom_gltf_2d.wgsl";

pub const ATTRIBUTE_BARYCENTRIC: MeshVertexAttribute = MeshVertexAttribute::new(
    "Barycentric",
    2137464976,
    VertexFormat::Float32x3
);

pub fn setup_gltf(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<CustomMaterial>>
) {
    let mesh = asset_server.load(
        (GltfAssetLabel::Primitive { mesh: 0, primitive: 0 }).from_asset("barycentric.gltf")
    );

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(CustomMaterial {})),
        Transform::from_scale(150.0 * Vec3::ONE),
    ));

    commands.spawn(Camera2d);
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {}

impl Material2d for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<Self>
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(
            &[
                Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
                Mesh::ATTRIBUTE_COLOR.at_shader_location(1),
                ATTRIBUTE_BARYCENTRIC.at_shader_location(2),
            ]
        )?;

        descriptor.vertex.buffers = vec![vertex_layout];

        Ok(())
    }
}
