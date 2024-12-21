use bevy::{
    app::{App, PluginGroup, Startup, Update}, asset::{AssetServer, Handle}, color::Color, image::Image, math::Vec2, prelude::{
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

    let player_texture = asset_server.load::<Image>("sprites/player.png");

    spawn_enemies(&mut commands, &asset_server, 4, 3);
    commands.spawn((
        Sprite {
            image: player_texture.clone(),
            color: Color::WHITE,
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        },
        Player::default(),
        Transform::from_xyz(-1.0, -1.0, 0.0),
    ));
}

fn spawn_enemy(
    commands: &mut Commands,
    texture: Handle<Image>,
    role: Role,
    position: Vec2
) {
    commands.spawn((
        Sprite {
            image: texture,
            color: Color::WHITE,
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        },
        Enemy {
            role,
            state: EnemyState::PATROL,
        },
        Patrol { point: (0.0, 0.0) },
        Transform::from_xyz(position.x, position.y, 0.0),
    ))
    .with_children(|parent| {
        parent.spawn((
            EnemyStatus,
            Text2d::new("PATROL"),
            TextLayout::new_with_justify(JustifyText::Center),
            Transform::from_xyz(0.0, 30.0, 0.0),
        ));
    });
}

fn spawn_enemies(
    commands: &mut Commands,
    asset_server: &AssetServer,
    num_engagers: u32,
    num_flankers: u32,
) {
    let engager_texture = asset_server.load::<Image>("sprites/red-block.png");
    let flanker_texture = asset_server.load::<Image>("sprites/flanker.png");
    
    for i in 0..num_engagers {
        let pos = Vec2::new(
            (i as f32 * 100.0) - 200.0,
            100.0
        );
        spawn_enemy(commands, engager_texture.clone(), Role::ENGAGER, pos);
    }
    
    for i in 0..num_flankers {
        let pos = Vec2::new(
            (i as f32 * 100.0) - 200.0,
            200.0
        );
        spawn_enemy(commands, flanker_texture.clone(), Role::FLANKER, pos);
    }
}
