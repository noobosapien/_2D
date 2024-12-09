use bevy::{ prelude::*, sprite::Wireframe2dPlugin };
use shapes::{ setup_2d, toggle_wireframe };
use cursor::draw_cursor;
use bloom::{ setup_bloom, update_bloom_settings };

mod cursor;
mod shapes;
mod bloom;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, #[cfg(not(target_arch = "wasm32"))] Wireframe2dPlugin))
        // .add_systems(Startup, setup_2d)
        .add_systems(Startup, setup_bloom);

    #[cfg(not(target_arch = "wasm32"))]
    app
        // .add_systems(Update, (toggle_wireframe, draw_cursor))
        .add_systems(Update, update_bloom_settings);
    app.run();
}
