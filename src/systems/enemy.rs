use bevy::{
    prelude::{Query, Transform},
    window::Window,
};
use rand::prelude::*;

use crate::components::enemy::{Enemy, EnemyState, Patrol, Role};

pub fn enemy_state_machine(
    mut query: Query<(&mut Patrol, &mut Enemy, &mut Transform)>,
    window: Query<&Window>,
) {
    // accessing window size
    let Ok(window) = window.get_single() else {
        return;
    };

    for (mut patrol, mut enemy, mut transform) in query.iter_mut() {
        // Arbitrary number for now
        const DISTANCE_TO_PLAYER: f32 = 0.0;

        const RETREAT_DISTANCE: f32 = 50.0;
        const ENGAGE_DISTANCE: f32 = 150.0;
        const OPTIMAL_DISTANCE: f32 = 125.0;
        const ENGAGE_RANGE: f32 = 20.0;
        const RETREAT_BUFFER: f32 = 90.0;

        // state machine
        if enemy.state == EnemyState::RETREAT && DISTANCE_TO_PLAYER < RETREAT_BUFFER {
            enemy.state = EnemyState::RETREAT;
        } else if DISTANCE_TO_PLAYER < RETREAT_DISTANCE {
            enemy.state = EnemyState::RETREAT;
        } else if (DISTANCE_TO_PLAYER - OPTIMAL_DISTANCE).abs() < ENGAGE_RANGE {
            enemy.state = EnemyState::ENGAGE;
        } else if DISTANCE_TO_PLAYER < ENGAGE_DISTANCE {
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

                    transform.translation.x += direction.0 * speed;
                    transform.translation.y += direction.1 * speed;
                }
            }
            EnemyState::RETREAT => {},
            EnemyState::ENGAGE => {},
            EnemyState::CHASE => {},
            EnemyState::FLANK => {},
            _ => (),
        }
    }
}
