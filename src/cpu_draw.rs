use bevy::color::{ color_difference::EuclideanDistance, palettes::css };
use bevy::prelude::*;
use bevy::render::{
    render_asset::RenderAssetUsages,
    render_resource::{ Extent3d, TextureDimension, TextureFormat },
};
use rand::Rng;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

#[derive(Resource)]
pub struct MyProcGenImage(Handle<Image>);

pub fn setup_cpu_draw(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2d);

    let mut image = Image::new_fill(
        Extent3d {
            width: IMAGE_WIDTH,
            height: IMAGE_HEIGHT,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &css::BEIGE.to_u8_array(),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
    );

    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let center = Vec2::new((IMAGE_WIDTH as f32) / 2.0, (IMAGE_HEIGHT as f32) / 2.0);
            let max_radius = (IMAGE_HEIGHT.min(IMAGE_WIDTH) as f32) / 2.0;
            let r = Vec2::new(x as f32, y as f32).distance(center);
            let a = 1.0 - (r / max_radius).clamp(0.0, 1.0);

            let pixel_bytes = image.pixel_bytes_mut(UVec3::new(x, y, 0)).unwrap();

            pixel_bytes[3] = (a * (u8::MAX as f32)) as u8;
        }
    }

    let handle = images.add(image);
    commands.spawn(Sprite::from_image(handle.clone()));
    commands.insert_resource(MyProcGenImage(handle));
}

pub fn draw_cpu_draw(
    my_handle: Res<MyProcGenImage>,
    mut images: ResMut<Assets<Image>>,
    mut i: Local<u32>,
    mut draw_color: Local<Color>
) {
    let mut rng = rand::thread_rng();

    if *i == 0 {
        *draw_color = Color::linear_rgb(rng.gen(), rng.gen(), rng.gen());
    }

    let image = images.get_mut(&my_handle.0).expect("Image not found");

    let center = Vec2::new((IMAGE_WIDTH as f32) / 2.0, (IMAGE_HEIGHT as f32) / 2.0);
    let max_radius = (IMAGE_HEIGHT.min(IMAGE_WIDTH) as f32) / 2.0;
    let rot_speed = 0.0123;
    let period = 0.12345;

    let r = ops::sin((*i as f32) * period) * max_radius;
    let xy = Vec2::from_angle((*i as f32) * rot_speed) * r + center;
    let (x, y) = (xy.x as u32, xy.y as u32);

    let old_color = image.get_color_at(x, y).unwrap();

    let tolerance = 1.0 / 255.0;

    if old_color.distance(&draw_color) <= tolerance {
        *draw_color = Color::linear_rgb(rng.gen(), rng.gen(), rng.gen());
    }

    image.set_color_at(x, y, draw_color.with_alpha(old_color.alpha())).unwrap();
    *i += 1;
}
