use bevy::{
    app::{App, Startup, Update},
    color::Color,
    math::{Vec2, Vec3},
    pbr::PointLight,
    prelude::{
        Camera3d, Commands, PerspectiveProjection, Projection,
        Transform,
    },
    sprite::Sprite,
    DefaultPlugins,
};

mod components;
mod systems;

use components::enemy::{Enemy, EnemyState, Patrol};
use systems::enemy::enemy_state_machine;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_def)
        .add_systems(Update, enemy_state_machine)
        .run();
}

fn spawn_def(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection::default()),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        PointLight {
            color: Color::WHITE,
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.5, 0.5),
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..Default::default()
        },
        Enemy {
            state: EnemyState::PATROL,
        },
        Patrol { point: (0.0, 0.0) },
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}
