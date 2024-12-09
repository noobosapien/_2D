use bevy::{ prelude::*, sprite::Wireframe2dPlugin };
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

mod cursor;
mod shapes;
mod bloom;
mod bounding_volume;

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

fn main() {
    let mut app = App::new();

    // app.add_plugins((DefaultPlugins, #[cfg(not(target_arch = "wasm32"))] Wireframe2dPlugin))
    //     // .add_systems(Startup, setup_2d)
    //     .add_systems(Startup, setup_bloom);

    // #[cfg(not(target_arch = "wasm32"))]
    // app
    //     // .add_systems(Update, (toggle_wireframe, draw_cursor))
    //     .add_systems(Update, update_bloom_settings);

    bounding_volume_init(&mut app);

    app.run();
}
