use bevy::{
    core_pipeline::{ bloom::{ Bloom, BloomCompositeMode }, tonemapping::Tonemapping },
    prelude::*,
};

pub fn setup_bloom(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    _asset_server: Res<AssetServer>
) {
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true,
            ..default()
        },
        Tonemapping::TonyMcMapface,
        Bloom::default(),
    ));

    //Circle mesh
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(100.0))),
        MeshMaterial2d(materials.add(Color::srgb(7.5, 0.0, 7.5))),
        Transform::from_translation(Vec3::new(-200.0, 0.0, 0.0)),
    ));

    //Hexagon mesh
    commands.spawn((
        Mesh2d(meshes.add(RegularPolygon::new(100.0, 6))),
        MeshMaterial2d(materials.add(Color::srgb(6.25, 9.4, 9.1))),
        Transform::from_translation(Vec3::new(200.0, 0.0, 0.0)),
    ));

    //UI
    commands.spawn((
        Text::default(),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

pub fn update_bloom_settings(
    camera: Single<(Entity, Option<&mut Bloom>), With<Camera>>,
    mut text: Single<&mut Text>,
    mut commands: Commands,
    keycode: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    let bloom = camera.into_inner();

    match bloom {
        (entity, Some(mut bloom)) => {
            text.0 = "Bloom (Toggle: Space)\n".to_string();
            text.push_str(&format!("(Q/A) Instensity: {}\n", bloom.intensity));
            text.push_str(&format!("(W/S) Low-frequency boost: {}\n", bloom.low_frequency_boost));
            text.push_str(
                &format!(
                    "(E/D) Low-frequency boost curvature: {}\n",
                    bloom.low_frequency_boost_curvature
                )
            );
            text.push_str(&format!("(R/F) High-pass frequency: {}\n", bloom.high_pass_frequency));
            text.push_str(
                &format!("(T/G) Mode: {}\n", match bloom.composite_mode {
                    BloomCompositeMode::EnergyConserving => "Energy-conserving",
                    BloomCompositeMode::Additive => "Additive",
                })
            );
            text.push_str(&format!("(Y/H) Threshold: {}\n", bloom.prefilter.threshold));
            text.push_str(
                &format!("(U/J) Threshold softness: {}\n", bloom.prefilter.threshold_softness)
            );

            if keycode.just_pressed(KeyCode::Space) {
                commands.entity(entity).remove::<Bloom>();
            }

            let dt = time.delta_secs();

            if keycode.pressed(KeyCode::KeyA) {
                bloom.intensity -= dt / 10.0;
            }

            if keycode.pressed(KeyCode::KeyQ) {
                bloom.intensity += dt / 10.0;
            }

            bloom.intensity = bloom.intensity.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::KeyS) {
                bloom.low_frequency_boost -= dt / 10.0;
            }

            if keycode.pressed(KeyCode::KeyW) {
                bloom.low_frequency_boost += dt / 10.0;
            }

            bloom.low_frequency_boost = bloom.low_frequency_boost.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::KeyD) {
                bloom.low_frequency_boost_curvature -= dt / 10.0;
            }

            if keycode.pressed(KeyCode::KeyE) {
                bloom.low_frequency_boost_curvature += dt / 10.0;
            }

            bloom.low_frequency_boost_curvature = bloom.low_frequency_boost_curvature.clamp(
                0.0,
                1.0
            );

            if keycode.pressed(KeyCode::KeyF) {
                bloom.high_pass_frequency -= dt / 10.0;
            }

            if keycode.pressed(KeyCode::KeyR) {
                bloom.high_pass_frequency += dt / 10.0;
            }

            bloom.high_pass_frequency = bloom.high_pass_frequency.clamp(0.0, 1.0);

            if keycode.pressed(KeyCode::KeyG) {
                bloom.composite_mode = BloomCompositeMode::Additive;
            }

            if keycode.pressed(KeyCode::KeyT) {
                bloom.composite_mode = BloomCompositeMode::EnergyConserving;
            }

            if keycode.pressed(KeyCode::KeyH) {
                bloom.prefilter.threshold -= dt;
            }

            if keycode.pressed(KeyCode::KeyY) {
                bloom.prefilter.threshold += dt;
            }

            bloom.prefilter.threshold = bloom.prefilter.threshold.max(0.0);

            if keycode.pressed(KeyCode::KeyJ) {
                bloom.prefilter.threshold_softness -= dt / 10.0;
            }

            if keycode.pressed(KeyCode::KeyU) {
                bloom.prefilter.threshold_softness += dt / 10.0;
            }

            bloom.prefilter.threshold_softness = bloom.prefilter.threshold_softness.clamp(0.0, 1.0);
        }

        (entity, None) => {
            text.0 = "Bloom: off (Toggle: Space)".to_string();

            if keycode.just_pressed(KeyCode::Space) {
                commands.entity(entity).insert(Bloom::default());
            }
        }
    }
}
