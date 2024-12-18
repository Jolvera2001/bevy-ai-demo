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
    let window = window.single();

    for (mut patrol, mut enemy, mut transform) in query.iter_mut() {
        match enemy.state {
            EnemyState::PATROL => {
                if transform.translation.x == patrol.point.0
                    && transform.translation.y == patrol.point.1
                {
                    patrol.point = (
                        random::<f32>() * window.width(),
                        random::<f32>() * window.height(),
                    );
                } else {
                    let x = if transform.translation.x < patrol.point.0 {
                        transform.translation.x + 1.0
                    } else {
                        transform.translation.x - 1.0
                    };
                    let y = if transform.translation.y < patrol.point.1 {
                        transform.translation.y + 1.0
                    } else {
                        transform.translation.y - 1.0
                    };
                    transform.translation = (x, y, 0.0).into();
                }
            }
            _ => (),
        }
    }
}
