use bevy::prelude::Component;

pub enum EnemyState {
    PATROL,
    CHASE,
    FLANK,
    ENGAGE,
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
