use bevy::{
    prelude::{Query, Transform},
    window::Window,
};
use rand::prelude::*;

use crate::components::enemy::{Enemy, EnemyState, Patrol};

pub fn enemy_state_machine(
    mut query: Query<(&mut Patrol, &mut Enemy, &mut Transform)>,
    window: Query<&Window>,
) {
    // accessing window size
    let Ok(window) = window.get_single() else {
        return;
    };

    for (mut patrol, enemy, mut transform) in query.iter_mut() {
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
            _ => (),
        }
    }
}
