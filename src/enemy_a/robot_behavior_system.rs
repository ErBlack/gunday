use super::robot_components::*;
use crate::constants::SCREEN_WIDTH;
use crate::enemy_a::ENEMY_A_CONFIG;
use bevy::prelude::*;

pub fn enemy_robot_behavior_system(
    time: Res<Time>,
    camera_q: Query<&GlobalTransform, (With<Camera>, Without<crate::player::components::Player>)>,
    mut enemies: Query<
        (&mut Transform, &mut EnemyRobotState, &mut EnemyShootTimer),
        With<EnemyRobot>,
    >,
) {
    let Ok(cam_tf) = camera_q.single() else {
        return;
    };
    let cam_x = cam_tf.translation().x;
    let half_w = SCREEN_WIDTH * 0.5;
    let config = &ENEMY_A_CONFIG;

    for (mut transform, mut state, mut shoot_timer) in enemies.iter_mut() {
        match state.state {
            EnemyRobotStateKind::Running => {
                let dir = if state.facing_right { 1.0 } else { -1.0 };
                let dx = dir * config.run_speed * time.delta_secs();
                transform.translation.x += dx;
                state.distance_run += dx.abs();
                let on_screen = transform.translation.x > cam_x - half_w
                    && transform.translation.x < cam_x + half_w;
                if state.distance_run >= config.run_distance_before_shoot && on_screen {
                    state.state = EnemyRobotStateKind::Shooting;
                    state.distance_run = 0.0;
                    shoot_timer.timer = 0.0;
                    shoot_timer.fired = false;
                }
            }
            EnemyRobotStateKind::Shooting => {}
            EnemyRobotStateKind::Hit => {}
        }
    }
}
