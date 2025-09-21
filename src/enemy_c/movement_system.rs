use super::components::{EnemyC, EnemyCHitFlash, EnemyCState, EnemyCStateKind, EnemyCVelocity};
use super::config::ENEMY_C_CONFIG;
use crate::constants::{DEFAULT_GRAVITY, GROUND_RECT_HEIGHT, SCREEN_HEIGHT};
use bevy::prelude::*;

pub fn enemy_c_movement_system(
    time: Res<Time>,
    mut commands: Commands,
    mut enemies: Query<
        (
            Entity,
            &mut Transform,
            &mut EnemyCState,
            &mut EnemyCVelocity,
            Option<Mut<EnemyCHitFlash>>,
        ),
        With<EnemyC>,
    >,
) {
    let delta = time.delta_secs();
    if delta <= 0.0 {
        return;
    }

    let ground = GROUND_RECT_HEIGHT - SCREEN_HEIGHT * 0.5;
    let death_ground_offset = ENEMY_C_CONFIG.death_ground_offset;

    for (entity, mut transform, mut state, mut velocity, hit_flash_opt) in enemies.iter_mut() {
        state.time_in_state += delta;

        if let Some(mut hit_flash) = hit_flash_opt {
            hit_flash.timer -= delta;
            if hit_flash.timer <= 0.0 {
                commands.entity(entity).remove::<EnemyCHitFlash>();
            }
        }

        match state.state {
            EnemyCStateKind::Running => {
                transform.translation.y = ground;
                velocity.velocity.y = 0.0;
            }
            EnemyCStateKind::JumpWindup => {
                transform.translation.y = ground;
                velocity.velocity.y = 0.0;
                velocity.velocity.x = 0.0;
            }
            EnemyCStateKind::Jumping => {
                velocity.velocity.y += DEFAULT_GRAVITY * delta;
            }
            EnemyCStateKind::Dying => {
                velocity.velocity = Vec2::ZERO;
                transform.translation.y = ground - death_ground_offset;
            }
        }

        transform.translation.x += velocity.velocity.x * delta;
        transform.translation.y += velocity.velocity.y * delta;

        if transform.translation.y <= ground {
            transform.translation.y = ground;
            if matches!(state.state, EnemyCStateKind::Jumping) {
                state.state = EnemyCStateKind::Running;
                state.time_in_state = 0.0;
                velocity.velocity.y = 0.0;
            }
        }
    }
}
