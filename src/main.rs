use bevy::{
    app::{App, PluginGroup, Startup, Update}, asset::AssetServer, color::Color, image::Image, math::Vec2, prelude::{
        BuildChildren, Camera2d, ChildBuild, Commands, ImagePlugin, Res, Text, Transform
    }, sprite::Sprite, text::{JustifyText, Text2d, TextLayout}, DefaultPlugins
};

mod components;
mod systems;

use components::{enemy::{Enemy, EnemyState, EnemyStatus, Patrol, Role}, player::Player};
use components::game::GameState;
use systems::enemy::enemy_state_machine;
use systems::player::player_movement;

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
        .add_systems(Update, (enemy_state_machine, player_movement))
        .run();
}

fn spawn_def(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());

    let enemy_texture = asset_server.load::<Image>("sprites/red-block.png");

    commands.spawn((
        Sprite {
            image: enemy_texture.clone(),
            color: Color::WHITE,
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        },
        Enemy {
            role: Role::ENGAGER,
            state: EnemyState::PATROL,
        },
        Patrol { point: (0.0, 0.0) },
        Transform::from_xyz(1.0, 1.0, 0.0),
    ))
    .with_children(|parent| {
        parent.spawn((
            EnemyStatus,
            Text2d::new("PATROL"),
            TextLayout::new_with_justify(JustifyText::Center),
            Transform::from_xyz(0.0, 30.0, 0.0),
        ));
    });
    
    commands.spawn((
        Sprite {
            image: enemy_texture.clone(),
            color: Color::WHITE,
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        },
        Player::default(),
        Transform::from_xyz(-1.0, -1.0, 0.0),
    ));
}
