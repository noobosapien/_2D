use bevy::{ gltf::GltfPlugin, prelude::*, sprite::{ Material2dPlugin, Wireframe2dPlugin } };
use shapes::{ setup_2d, toggle_wireframe };
use cursor::draw_cursor;
use bloom::{ setup_bloom, update_bloom_settings };
use bounding_volume::{
    Test,
    setup_bv,
    update_text,
    spin,
    update_volumes,
    update_test_state,
    render_shapes,
    aabb_intersection_system,
    circle_intersection_system,
    ray_cast_system,
    aabb_cast_system,
    bounding_circle_cast_system,
    render_volumes,
};
use cpu_draw::{ setup_cpu_draw, draw_cpu_draw };
use custom_gltf_vertex_attributes::{ ATTRIBUTE_BARYCENTRIC, setup_gltf, CustomMaterial };

mod cursor;
mod shapes;
mod bloom;
mod bounding_volume;
mod cpu_draw;
mod custom_gltf_vertex_attributes;

fn gltf_init(app: &mut App) {
    app.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0 / 5.0f32,
    });

    app.add_plugins((
        DefaultPlugins.set(
            GltfPlugin::default().add_custom_vertex_attribute("_BARYCENTRIC", ATTRIBUTE_BARYCENTRIC)
        ),
        Material2dPlugin::<CustomMaterial>::default(),
    ));

    app.add_systems(Startup, setup_gltf);
}

fn cpu_draw_init(app: &mut App) {
    app.add_plugins(DefaultPlugins);
    app.insert_resource(Time::<Fixed>::from_hz(1024.0));
    app.add_systems(Startup, setup_cpu_draw);
    app.add_systems(FixedUpdate, draw_cpu_draw);
}

fn bounding_volume_init(app: &mut App) {
    app.add_plugins(DefaultPlugins);
    app.init_state::<Test>();

    app.add_systems(Startup, setup_bv);
    app.add_systems(Update, (update_text, spin, update_volumes, update_test_state));
    app.add_systems(PostUpdate, (
        render_shapes,
        (
            aabb_intersection_system.run_if(in_state(Test::AabbSweep)),
            circle_intersection_system.run_if(in_state(Test::CircleSweep)),
            ray_cast_system.run_if(in_state(Test::RayCast)),
            aabb_cast_system.run_if(in_state(Test::AabbCast)),
            bounding_circle_cast_system.run_if(in_state(Test::CircleCast)),
        ),
        render_volumes,
    ));
}

fn cursor_bloom_init(app: &mut App) {
    app.add_plugins((DefaultPlugins, #[cfg(not(target_arch = "wasm32"))] Wireframe2dPlugin))
        // .add_systems(Startup, setup_2d)
        .add_systems(Startup, setup_bloom);

    #[cfg(not(target_arch = "wasm32"))]
    app
        // .add_systems(Update, (toggle_wireframe, draw_cursor))
        .add_systems(Update, update_bloom_settings);
}

fn main() {
    let mut app = App::new();

    // cursor_bloom_init(&mut app);
    // bounding_volume_init(&mut app);
    // cpu_draw_init(&mut app);
    gltf_init(&mut app);

    app.run();
}
