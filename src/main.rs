use bevy::{
    app::{App, Startup, Update},
    prelude::{Commands, Transform},
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
        Enemy {
            state: EnemyState::PATROL,
        },
        Patrol { point: (0.0, 0.0) },
        Transform::from_xyz(10.0, 20.0, 0.0),
    ));
}
