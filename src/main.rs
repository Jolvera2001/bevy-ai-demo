use bevy::{
    app::{App, PluginGroup, Startup, Update},
    asset::AssetServer,
    color::Color,
    image::Image,
    math::Vec2,
    prelude::{
        Camera2d, Camera2dBundle, Commands, ImagePlugin, OrthographicProjection,
        PerspectiveProjection, Res, Transform,
    },
    sprite::Sprite,
    DefaultPlugins,
};

mod components;
mod systems;

use components::enemy::{Enemy, EnemyState, Patrol};
use components::game::GameState;
use systems::enemy::enemy_state_machine;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(GameState {
            time: 0.00,
            score: 0,
            level: 0,
            game_over: false,
            title: true,
        })
        .add_systems(Startup, spawn_def)
        .add_systems(Update, enemy_state_machine)
        .run();
}

fn spawn_def(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());

    let enemy_texture = asset_server.load::<Image>("sprites/red-block.png");

    commands.spawn((
        Sprite {
            image: enemy_texture,
            color: Color::WHITE,
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        },
        Enemy {
            state: EnemyState::PATROL,
        },
        Patrol { point: (0.0, 0.0) },
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}
