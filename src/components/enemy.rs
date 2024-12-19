use bevy::prelude::Component;

pub enum EnemyState {
    CHASE,
    ENGAGE,
    FLANK,
    PATROL,
    RETREAT,
}

#[derive(Component)]
pub struct Enemy {
    pub state: EnemyState,
}

#[derive(Component)]
pub struct Patrol {
    pub point: (f32, f32),
}
