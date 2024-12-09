use bevy::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::{ Wireframe2dConfig, Wireframe2dPlugin };

const X_EXTENT: f32 = 900.0;

pub fn setup_2d(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(Camera2d);

    let shapes = [
        meshes.add(Circle::new(50.0)),
        meshes.add(CircularSector::new(50.0, 1.0)),
        meshes.add(CircularSegment::new(50.0, 1.25)),
        meshes.add(Ellipse::new(25.0, 50.0)),
        meshes.add(Annulus::new(25.0, 50.0)),
        meshes.add(Capsule2d::new(25.0, 50.0)),
        meshes.add(Rhombus::new(75.0, 100.0)),
        meshes.add(Rectangle::new(50.0, 100.0)),
        meshes.add(RegularPolygon::new(50.0, 6)),
        meshes.add(
            Triangle2d::new(Vec2::Y * 50.0, Vec2::new(-50.0, -50.0), Vec2::new(50.0, -50.0))
        ),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        let color = Color::hsl((360.0 * (i as f32)) / (num_shapes as f32), 0.95, 0.7);

        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(
                -X_EXTENT / 2.0 + ((i as f32) / ((num_shapes - 1) as f32)) * X_EXTENT,
                0.0,
                0.0
            ),
        ));
    }

    #[cfg(not(target_arch = "wasm32"))]
    commands.spawn((
        Text::new("Press space to toggle wireframes"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

#[cfg(not(target_arch = "wasm32"))]
pub fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}
