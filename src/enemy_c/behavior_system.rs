use super::components::{
    EnemyC, EnemyCJumpController, EnemyCSpawnPause, EnemyCState, EnemyCStateKind, EnemyCVelocity,
};
use super::config::ENEMY_C_CONFIG;
use crate::player::components::Player;
use bevy::prelude::*;

pub fn enemy_c_behavior_system(
    mut commands: Commands,
    time: Res<Time>,
    player_q: Query<&Transform, With<Player>>,
    mut enemies: Query<
        (
            Entity,
            &Transform,
            &mut EnemyCState,
            &mut EnemyCJumpController,
            &mut EnemyCVelocity,
            Option<&mut EnemyCSpawnPause>,
        ),
        With<EnemyC>,
    >,
) {
    let delta = time.delta_secs();
    if delta <= 0.0 {
        return;
    }

    let player_tf = player_q.iter().next().map(|tf| tf.translation);

    for (entity, transform, mut state, mut jump, mut velocity, spawn_pause) in enemies.iter_mut() {
        if matches!(state.state, EnemyCStateKind::Dying) {
            continue;
        }

        let facing_dir = if state.facing_right { 1.0 } else { -1.0 };

        let mut pause_active = false;
        let mut pause_finished = false;
        if let Some(mut pause) = spawn_pause {
            pause.timer -= delta;
            if pause.timer > 0.0 {
                velocity.velocity = Vec2::ZERO;
                state.time_in_state = 0.0;
                pause_active = true;
            } else {
                pause_finished = true;
            }
        }

        if pause_finished {
            commands.entity(entity).remove::<EnemyCSpawnPause>();
        }

        if pause_active {
            continue;
        }

        match state.state {
            EnemyCStateKind::Running => {
                jump.cooldown = (jump.cooldown - delta).max(0.0);
                velocity.velocity.x = facing_dir * ENEMY_C_CONFIG.run_speed;
                velocity.velocity.y = 0.0;

                if let Some(player_translation) = player_tf {
                    let forward_distance =
                        (player_translation.x - transform.translation.x) * facing_dir;
                    if jump.cooldown <= 0.0
                        && forward_distance >= 0.0
                        && forward_distance <= ENEMY_C_CONFIG.jump_trigger_distance
                    {
                        state.state = EnemyCStateKind::JumpWindup;
                        state.time_in_state = 0.0;
                        velocity.velocity.x = 0.0;
                    }
                }
            }
            EnemyCStateKind::JumpWindup => {
                velocity.velocity.x = 0.0;
                velocity.velocity.y = 0.0;
                if state.time_in_state >= ENEMY_C_CONFIG.jump_windup_duration {
                    velocity.velocity.x = facing_dir * ENEMY_C_CONFIG.jump_horizontal_speed;
                    velocity.velocity.y = ENEMY_C_CONFIG.jump_vertical_speed;
                    state.state = EnemyCStateKind::Jumping;
                    state.time_in_state = 0.0;
                    jump.cooldown = ENEMY_C_CONFIG.jump_cooldown_duration;
                }
            }
            EnemyCStateKind::Jumping => {
            }
            EnemyCStateKind::Dying => {
            }
        }
    }
}
