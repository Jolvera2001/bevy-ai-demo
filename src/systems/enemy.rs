use std::f32::consts::PI;

use bevy::{
    prelude::{ParamSet, Query, Transform, With},
    window::Window,
};
use rand::prelude::*;

use crate::components::{
    enemy::{Enemy, EnemyState, Patrol, Role},
    player::Player,
};

pub fn enemy_state_machine(
    mut query_set: ParamSet<(
        Query<(&mut Patrol, &mut Enemy, &mut Transform)>,
        Query<&Transform, With<Player>>,
    )>,
    window: Query<&Window>,
) {
    // accessing window size
    let Ok(window) = window.get_single() else {
        return;
    };

    // accessing player
    let player_pos = {
        let player_query = query_set.p1();
        let Ok(player_transform) = player_query.get_single() else {
            return;
        };
        player_transform.translation
    };

    for (mut patrol, mut enemy, mut transform) in query_set.p0().iter_mut() {
        // math.sqrt((self.x - px) ** 2 + (self.y - py) ** 2)
        // apperently this is less overhead? I wouldn't need to use
        // .powf(2.0)
        let dx = transform.translation.x - player_pos.x;
        let dy = transform.translation.y - player_pos.y;
        let distance_to_player = (dx * dx + dy * dy).sqrt();

        const RETREAT_DISTANCE: f32 = 50.0;
        const ENGAGE_DISTANCE: f32 = 150.0;
        const OPTIMAL_DISTANCE: f32 = 125.0;
        const ENGAGE_RANGE: f32 = 20.0;
        const RETREAT_BUFFER: f32 = 90.0;
        const SPEED: f32 = 2.0;

        // state machine
        if enemy.state == EnemyState::RETREAT && distance_to_player < RETREAT_BUFFER {
            enemy.state = EnemyState::RETREAT;
        } else if distance_to_player < RETREAT_DISTANCE {
            enemy.state = EnemyState::RETREAT;
        } else if (distance_to_player - OPTIMAL_DISTANCE).abs() < ENGAGE_RANGE {
            enemy.state = EnemyState::ENGAGE;
        } else if distance_to_player < ENGAGE_DISTANCE {
            if enemy.role == Role::FLANKER {
                enemy.state = EnemyState::FLANK;
            } else {
                enemy.state = EnemyState::CHASE;
            }
        }
        {
            enemy.state = EnemyState::PATROL;
        }

        // behaviors
        match enemy.state {
            EnemyState::PATROL => {
                if (transform.translation.x - patrol.point.0).abs() < 1.0
                    && (transform.translation.y - patrol.point.1).abs() < 1.0
                {
                    patrol.point = (
                        random::<f32>() * window.width() - window.width() / 2.0,
                        random::<f32>() * window.height() - window.height() / 2.0,
                    );
                } else {
                    let dx = patrol.point.0 - transform.translation.x;
                    let dy = patrol.point.1 - transform.translation.y;

                    let speed = 2.0;

                    // (a^2 + b^2 = c^2)
                    let distance = (dx * dx + dy * dy).sqrt();

                    // normalizing distance
                    let direction = (dx / distance, dy / distance);

                    transform.translation.x += direction.0 * SPEED;
                    transform.translation.y += direction.1 * SPEED;
                }
            }
            EnemyState::RETREAT => {
                let enemy_pos = transform.translation;

                let angle = f32::atan2(enemy_pos.y - player_pos.y, enemy_pos.x - player_pos.x);

                transform.translation.x += f32::cos(angle) * SPEED;
                transform.translation.y += f32::sin(angle) * SPEED;
            }
            EnemyState::ENGAGE => {
                let enemy_pos = transform.translation;

                let angle = f32::atan2(player_pos.y - enemy_pos.y, player_pos.x - enemy_pos.x);
                let distance_error = (distance_to_player - OPTIMAL_DISTANCE) / OPTIMAL_DISTANCE;

                let adjust_speed = SPEED * 0.5 * distance_error;
                transform.translation.x += f32::cos(angle) * adjust_speed;
                transform.translation.y += f32::sin(angle) * adjust_speed;
            }
            EnemyState::CHASE => {
                let enemy_pos = transform.translation;

                let angle = f32::atan2(player_pos.y - enemy_pos.y, player_pos.x - enemy_pos.x);
                transform.translation.x += f32::cos(angle) * SPEED;
                transform.translation.y += f32::sin(angle) * SPEED;
            }
            EnemyState::FLANK => {
                let enemy_pos = transform.translation;

                let angle = f32::atan2(player_pos.y - enemy_pos.y, player_pos.x - enemy_pos.x);
                let distance_factor = (distance_to_player - OPTIMAL_DISTANCE) / OPTIMAL_DISTANCE;

                let circling_x = f32::cos(angle + PI / 2.0) * SPEED;
                let circling_y = f32::sin(angle + PI / 2.0) * SPEED;

                let radial_x = f32::cos(angle) * SPEED * distance_factor;
                let radial_y = f32::sin(angle) * SPEED * distance_factor;

                transform.translation.x += circling_x + radial_x;
                transform.translation.y += circling_y + radial_y;
            }
        }
    }
}
