use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

mod l1;
mod l2;
mod l3;

pub struct Config;
impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app.add_system(spawn.on_startup());
    }
}

fn spawn(mut commands: Commands) {
    l1::spawn(&mut commands, (-10000., -10000.));
    l2::spawn(&mut commands, (10000., 10000.));
    l3::spawn(&mut commands, (-10000., 10000.));
}

pub fn get_description(current_level: u32) -> &'static str {
    match current_level {
        1 => l1::DESC,
        2 => l2::DESC,
        3 => l3::DESC,
        _ => "Invalid Level",
    }
}

fn line(
    commands: &mut Commands,
    dimensions: (f32, f32),
    transform: (f32, f32),
    offset: (f32, f32),
    fill_color: Color,
) -> Entity {
    let shape = shapes::Rectangle {
        extents: Vec2::new(dimensions.0, dimensions.1),
        origin: shapes::RectangleOrigin::Center,
    };

    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: Transform {
                    translation: Vec3::new(transform.0 + offset.0, transform.1 + offset.1, 0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            Fill::color(fill_color),
            Stroke::new(Color::BLACK, 0.),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(dimensions.0 / 2., dimensions.1 / 2.))
        .id()
}

fn shape(
    commands: &mut Commands,
    points: &Vec<Vec2>,
    transform: (f32, f32),
    offset: (f32, f32),
    fill_color: Color,
) -> Entity {
    let shape = shapes::Polygon {
        points: points.clone(),
        closed: true,
    };

    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: Transform {
                    translation: Vec3::new(transform.0 + offset.0, transform.1 + offset.1, 0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            Fill::color(fill_color),
            Stroke::new(Color::BLACK, 0.),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::convex_hull(&points).unwrap())
        .id()
}
