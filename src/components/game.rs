use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GameState {
    pub time: f32,
    pub score: i32,
    pub level: i32,
    pub game_over: bool,
    pub title: bool,
}
