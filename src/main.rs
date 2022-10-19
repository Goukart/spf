use bevy::{prelude::*, render::color::HexColorError};
use bevy_rapier3d::prelude::*;
// use avatar::Avatar;
// mod avatar;
mod bullet;


fn main() {
    /*
    let mut blue = avatar::Avatar::new(
        Color::MIDNIGHT_BLUE,
        String::from("blue"),
        Vec3 { x: 1., y: 1., z: 1. },
    );*/

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(print_ball_altitude)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn()
        .insert(Collider::cuboid(100.0, 0.1, 100.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}
