use super::infected::InfectedTransformAnim;
use super::robot_components::*;
use crate::assets::GameAssets;
use crate::enemy_a::ENEMY_A_CONFIG;
use bevy::prelude::*;

pub fn enemy_robot_animation_system(
    time: Res<Time>,
    mut query: Query<
        (
            &mut Sprite,
            &mut EnemyRunAnim,
            &EnemyRobotState,
            Option<&InfectedTransformAnim>,
        ),
        With<EnemyRobot>,
    >,
    game_assets: Res<GameAssets>,
) {
    let frame_time = ENEMY_A_CONFIG.run_frame_time;
    for (mut sprite, mut anim, state, infected_anim) in query.iter_mut() {
        if infected_anim.is_some() {
            continue;
        }
        sprite.flip_x = state.facing_right;
        match state.state {
            EnemyRobotStateKind::Running => {
                anim.timer -= time.delta_secs();
                if anim.timer <= 0.0 {
                    anim.frame = if anim.frame >= 4 { 1 } else { anim.frame + 1 };
                    anim.timer = frame_time;
                }
                let handle = match anim.frame {
                    1 => &game_assets.enemy_a_run_a,
                    2 => &game_assets.enemy_a_run_b,
                    3 => &game_assets.enemy_a_run_c,
                    4 => &game_assets.enemy_a_run_d,
                    _ => &game_assets.enemy_a_run_a,
                };
                if sprite.image != *handle {
                    sprite.image = handle.clone();
                }
            }
            EnemyRobotStateKind::Shooting => {
                if sprite.image != game_assets.enemy_a_shoot {
                    sprite.image = game_assets.enemy_a_shoot.clone();
                }
            }
            EnemyRobotStateKind::Hit => {
                if sprite.image != game_assets.enemy_a_hit {
                    sprite.image = game_assets.enemy_a_hit.clone();
                }
            }
        }
    }
}
