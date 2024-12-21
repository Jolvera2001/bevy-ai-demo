use bevy::{
    input::{keyboard::KeyCode, ButtonInput},
    math::Vec3,
    prelude::{Query, Res, Transform, Vec2},
    time::Time,
};

pub fn player_movement(
    mut query: Query<(&crate::components::player::Player, &mut Transform)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (player, mut transform) in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        if input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction != Vec2::ZERO {
            direction = direction.normalize();
        }

        transform.translation += Vec3::new(
            direction.x * player.speed * time.delta_secs(),
            direction.y * player.speed * time.delta_secs(),
            0.0
        )
    }
}
