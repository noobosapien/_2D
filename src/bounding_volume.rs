use bevy::{ color::palettes::css::*, math::{ bounding::*, ops, Isometry2d }, prelude::* };

#[derive(Component)]
pub struct Spin;

pub fn spin(time: Res<Time>, mut query: Query<&mut Transform, With<Spin>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_z(time.delta_secs() / 5.0);
    }
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Test {
    AabbSweep,
    CircleSweep,
    #[default]
    RayCast,
    AabbCast,
    CircleCast,
}

pub fn update_test_state(
    keycode: Res<ButtonInput<KeyCode>>,
    curr_state: Res<State<Test>>,
    mut state: ResMut<NextState<Test>>
) {
    if !keycode.just_pressed(KeyCode::Space) {
        return;
    }

    use Test::*;

    let next = match **curr_state {
        AabbSweep => CircleSweep,
        CircleSweep => RayCast,
        RayCast => AabbCast,
        AabbCast => CircleCast,
        CircleCast => AabbSweep,
    };

    state.set(next);
}

pub fn update_text(mut text: Single<&mut Text>, cur_state: Res<State<Test>>) {
    if !cur_state.is_changed() {
        return;
    }

    text.clear();

    text.push_str("Intersection test:\n");
    use Test::*;

    for &test in &[AabbSweep, CircleSweep, RayCast, AabbCast, CircleCast] {
        let s = if **cur_state == test { "*" } else { " " };
        text.push_str(&format!("{s} {test:?} {s}\n"));
    }

    text.push_str("\nPress space to cycle");
}

#[derive(Component)]
pub enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Triangle(Triangle2d),
    Line(Segment2d),
    Capsule(Capsule2d),
    Polygon(RegularPolygon),
}

pub fn render_shapes(mut gizmos: Gizmos, query: Query<(&Shape, &Transform)>) {
    let color = GRAY;

    for (shape, transform) in query.iter() {
        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
        let isometry = Isometry2d::new(translation, Rot2::radians(rotation));

        match shape {
            Shape::Rectangle(r) => {
                gizmos.primitive_2d(r, isometry, color);
            }

            Shape::Circle(c) => {
                gizmos.primitive_2d(c, isometry, color);
            }

            Shape::Triangle(t) => {
                gizmos.primitive_2d(t, isometry, color);
            }

            Shape::Line(l) => {
                gizmos.primitive_2d(l, isometry, color);
            }

            Shape::Capsule(c) => {
                gizmos.primitive_2d(c, isometry, color);
            }

            Shape::Polygon(p) => {
                gizmos.primitive_2d(p, isometry, color);
            }
        }
    }
}

#[derive(Component)]
pub enum DesiredVolume {
    Aabb,
    Circle,
}

#[derive(Component, Debug)]
pub enum CurrentVolume {
    Aabb(Aabb2d),
    Circle(BoundingCircle),
}

pub fn update_volumes(
    mut commands: Commands,
    query: Query<
        (Entity, &DesiredVolume, &Shape, &Transform),
        Or<(Changed<DesiredVolume>, Changed<Shape>, Changed<Transform>)>
    >
) {
    for (entity, desired_volume, shape, transform) in query.iter() {
        let translation = transform.translation.xy();
        let rotation = transform.rotation.to_euler(EulerRot::YXZ).2;
        let isometry = Isometry2d::new(translation, Rot2::radians(rotation));

        match desired_volume {
            DesiredVolume::Aabb => {
                let aabb = match shape {
                    Shape::Rectangle(r) => r.aabb_2d(isometry),
                    Shape::Circle(c) => c.aabb_2d(isometry),
                    Shape::Triangle(t) => t.aabb_2d(isometry),
                    Shape::Line(l) => l.aabb_2d(isometry),
                    Shape::Capsule(c) => c.aabb_2d(isometry),
                    Shape::Polygon(p) => p.aabb_2d(isometry),
                };

                commands.entity(entity).insert(CurrentVolume::Aabb(aabb));
            }

            DesiredVolume::Circle => {
                let circle = match shape {
                    Shape::Rectangle(r) => r.bounding_circle(isometry),
                    Shape::Circle(c) => c.bounding_circle(isometry),
                    Shape::Triangle(t) => t.bounding_circle(isometry),
                    Shape::Line(l) => l.bounding_circle(isometry),
                    Shape::Capsule(c) => c.bounding_circle(isometry),
                    Shape::Polygon(p) => p.bounding_circle(isometry),
                };

                commands.entity(entity).insert(CurrentVolume::Circle(circle));
            }
        }
    }
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct Intersects(bool);

pub fn render_volumes(mut gizmos: Gizmos, query: Query<(&CurrentVolume, &Intersects)>) {
    for (volume, intersects) in query.iter() {
        let color = if **intersects { AQUA } else { ORANGE_RED };

        match volume {
            CurrentVolume::Aabb(a) => {
                gizmos.rect_2d(a.center(), a.half_size() * 2.0, color);
            }

            CurrentVolume::Circle(c) => {
                gizmos.circle_2d(c.center(), c.radius(), color);
            }
        }
    }
}

const OFFSET_X: f32 = 125.0;
const OFFSET_Y: f32 = 75.0;

pub fn setup_bv(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Transform::from_xyz(-OFFSET_X, OFFSET_Y, 0.0),
        Shape::Circle(Circle::new(45.0)),
        DesiredVolume::Aabb,
        Intersects::default(),
    ));

    commands.spawn((
        Transform::from_xyz(0.0, OFFSET_Y, 0.0),
        Shape::Rectangle(Rectangle::new(80.0, 80.0)),
        Spin,
        DesiredVolume::Circle,
        Intersects::default(),
    ));

    commands.spawn((
        Transform::from_xyz(OFFSET_X, OFFSET_Y, 0.0),
        Shape::Triangle(
            Triangle2d::new(Vec2::new(-40.0, -40.0), Vec2::new(-20.0, -40.0), Vec2::new(40.0, 50.0))
        ),
        Spin,
        DesiredVolume::Aabb,
        Intersects::default(),
    ));

    commands.spawn((
        Transform::from_xyz(-OFFSET_X, -OFFSET_Y, 0.0),
        Shape::Line(Segment2d::new(Dir2::from_xy(1.0, 0.3).unwrap(), 90.0)),
        Spin,
        DesiredVolume::Circle,
        Intersects::default(),
    ));

    commands.spawn((
        Transform::from_xyz(0.0, -OFFSET_Y, 0.0),
        Shape::Capsule(Capsule2d::new(25.0, 50.0)),
        Spin,
        DesiredVolume::Aabb,
        Intersects::default(),
    ));

    commands.spawn((
        Transform::from_xyz(0.0, -OFFSET_Y, 0.0),
        Shape::Polygon(RegularPolygon::new(25.0, 6)),
        Spin,
        DesiredVolume::Circle,
        Intersects::default(),
    ));

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

pub fn draw_filled_circle(gizmos: &mut Gizmos, position: Vec2, color: Srgba) {
    for r in [1.0, 2.0, 3.0] {
        gizmos.circle_2d(position, r, color);
    }
}

pub fn draw_ray(gizmos: &mut Gizmos, ray: &RayCast2d) {
    gizmos.line_2d(ray.ray.origin, ray.ray.origin + *ray.ray.direction * ray.max, WHITE);

    draw_filled_circle(gizmos, ray.ray.origin, FUCHSIA);
}

pub fn get_and_draw_ray(gizmos: &mut Gizmos, time: &Time) -> RayCast2d {
    let ray = Vec2::new(ops::cos(time.elapsed_secs()), ops::sin(time.elapsed_secs()));
    let dist = 150.0 + ops::sin(0.5 * time.elapsed_secs()).abs() * 500.0;

    let aabb_ray = Ray2d {
        origin: ray * 250.0,
        direction: Dir2::new_unchecked(-ray),
    };

    let ray_cast = RayCast2d::from_ray(aabb_ray, dist - 20.0);

    draw_ray(gizmos, &ray_cast);
    ray_cast
}

pub fn ray_cast_system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut volumes: Query<(&CurrentVolume, &mut Intersects)>
) {
    let ray_cast = get_and_draw_ray(&mut gizmos, &time);

    for (volume, mut intersects) in volumes.iter_mut() {
        let toi = match volume {
            CurrentVolume::Aabb(a) => ray_cast.aabb_intersection_at(a),
            CurrentVolume::Circle(c) => ray_cast.circle_intersection_at(c),
        };

        **intersects = toi.is_some();

        if let Some(toi) = toi {
            draw_filled_circle(
                &mut gizmos,
                ray_cast.ray.origin + *ray_cast.ray.direction * toi,
                LIME
            );
        }
    }
}

pub fn aabb_cast_system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut volumes: Query<(&CurrentVolume, &mut Intersects)>
) {
    let ray_cast = get_and_draw_ray(&mut gizmos, &time);
    let aabb_cast = AabbCast2d {
        aabb: Aabb2d::new(Vec2::ZERO, Vec2::splat(15.0)),
        ray: ray_cast,
    };

    for (volume, mut intersects) in volumes.iter_mut() {
        let toi = match *volume {
            CurrentVolume::Aabb(a) => aabb_cast.aabb_collision_at(a),
            CurrentVolume::Circle(_) => None,
        };

        **intersects = toi.is_some();

        if let Some(toi) = toi {
            gizmos.rect_2d(
                aabb_cast.ray.ray.origin + *aabb_cast.ray.ray.direction * toi,
                aabb_cast.aabb.half_size() * 2.0,
                LIME
            );
        }
    }
}

pub fn bounding_circle_cast_system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut volumes: Query<(&CurrentVolume, &mut Intersects)>
) {
    let ray_cast = get_and_draw_ray(&mut gizmos, &time);
    let circle_cast = BoundingCircleCast {
        circle: BoundingCircle::new(Vec2::ZERO, 15.0),
        ray: ray_cast,
    };

    for (volume, mut intersects) in volumes.iter_mut() {
        let toi = match *volume {
            CurrentVolume::Aabb(_) => None,
            CurrentVolume::Circle(c) => circle_cast.circle_collision_at(c),
        };

        **intersects = toi.is_some();

        if let Some(toi) = toi {
            gizmos.circle_2d(
                circle_cast.ray.ray.origin + *circle_cast.ray.ray.direction * toi,
                circle_cast.circle.radius(),
                LIME
            );
        }
    }
}

pub fn get_intersection_position(time: &Time) -> Vec2 {
    let x = ops::cos(0.8 * time.elapsed_secs()) * 250.0;
    let y = ops::sin(0.4 * time.elapsed_secs()) * 100.0;

    Vec2::new(x, y)
}

pub fn aabb_intersection_system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut volumes: Query<(&CurrentVolume, &mut Intersects)>
) {
    let center = get_intersection_position(&time);
    let aabb = Aabb2d::new(center, Vec2::splat(50.0));
    gizmos.rect_2d(center, aabb.half_size() * 2.0, YELLOW);

    for (volume, mut intersects) in volumes.iter_mut() {
        let hit = match volume {
            CurrentVolume::Aabb(a) => aabb.intersects(a),
            CurrentVolume::Circle(c) => aabb.intersects(c),
        };

        **intersects = hit;
    }
}

pub fn circle_intersection_system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut volumes: Query<(&CurrentVolume, &mut Intersects)>
) {
    let center = get_intersection_position(&time);
    let circle = BoundingCircle::new(center, 50.0);
    gizmos.circle_2d(center, circle.radius(), YELLOW);

    for (volume, mut intersects) in volumes.iter_mut() {
        let hit = match volume {
            CurrentVolume::Aabb(a) => circle.intersects(a),
            CurrentVolume::Circle(c) => circle.intersects(c),
        };

        **intersects = hit;
    }
}
