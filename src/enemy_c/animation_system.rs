use super::components::{
    EnemyC, EnemyCAnimation, EnemyCHitFlash, EnemyCSpawnPause, EnemyCState, EnemyCStateKind,
};
use super::config::ENEMY_C_CONFIG;
use crate::assets::GameAssets;
use bevy::prelude::*;

pub fn enemy_c_animation_system(
    time: Res<Time>,
    mut query: Query<
        (
            &mut Sprite,
            &mut EnemyCAnimation,
            &EnemyCState,
            Option<&EnemyCHitFlash>,
            Option<&EnemyCSpawnPause>,
        ),
        With<EnemyC>,
    >,
    game_assets: Res<GameAssets>,
) {
    let delta = time.delta_secs();
    let frame_time = ENEMY_C_CONFIG.run_frame_time;
    for (mut sprite, mut anim, state, hit_flash_opt, spawn_pause) in query.iter_mut() {
        sprite.flip_x = state.facing_right;

        let run_frames = [
            &game_assets.enemy_c_run_a,
            &game_assets.enemy_c_run_b,
            &game_assets.enemy_c_run_c,
        ];

        if let Some(pause) = spawn_pause {
            if pause.timer > 0.0 {
                anim.frame = 0;
                anim.timer = frame_time;
                let desired = run_frames[0];
                if sprite.image != *desired {
                    sprite.image = desired.clone();
                }
                sprite.color = sprite.color.with_alpha(1.0);
                continue;
            }
        }

        let mut apply_hit_flash = false;

        match state.state {
            EnemyCStateKind::Running => {
                anim.timer -= delta;
                if anim.timer <= 0.0 {
                    anim.frame = (anim.frame + 1) % run_frames.len();
                    anim.timer = frame_time;
                }
                let desired = run_frames[anim.frame];
                if sprite.image != *desired {
                    sprite.image = desired.clone();
                }
                apply_hit_flash = hit_flash_opt.is_some();
            }
            EnemyCStateKind::JumpWindup => {
                anim.frame = 0;
                anim.timer = frame_time;
                let desired = run_frames[0];
                if sprite.image != *desired {
                    sprite.image = desired.clone();
                }
                apply_hit_flash = hit_flash_opt.is_some();
            }
            EnemyCStateKind::Jumping => {
                anim.frame = 0;
                anim.timer = frame_time;
                if sprite.image != game_assets.enemy_c_jump {
                    sprite.image = game_assets.enemy_c_jump.clone();
                }
            }
            EnemyCStateKind::Dying => {
                anim.frame = 0;
                if sprite.image != game_assets.enemy_c_hit {
                    sprite.image = game_assets.enemy_c_hit.clone();
                }
            }
        }

        if apply_hit_flash {
            sprite.color = sprite.color.with_alpha(0.85);
        } else {
            sprite.color = sprite.color.with_alpha(1.0);
        }
    }
}
