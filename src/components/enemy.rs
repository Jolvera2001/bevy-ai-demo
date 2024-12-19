use bevy::prelude::Component;

#[derive(PartialEq, Eq)]
pub enum EnemyState {
    CHASE,
    ENGAGE,
    FLANK,
    PATROL,
    RETREAT,
}

#[derive(PartialEq, Eq)]
pub enum Role {
    FLANKER,
    ENGAGER,
}

#[derive(Component)]
pub struct Enemy {
    pub state: EnemyState,
    pub role: Role,
}

#[derive(Component)]
pub struct Patrol {
    pub point: (f32, f32),
}
