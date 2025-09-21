use super::robot_components::*;
use crate::assets::GameAssets;
use crate::audio::{SfxEmitters, play_sfx_once};
use crate::enemy_a::ENEMY_A_CONFIG;
use crate::projectile::projectile_spawning_system::spawn_enemy_projectile;
use bevy::prelude::*;

pub fn enemy_robot_projectile_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&Transform, &mut EnemyRobotState, &mut EnemyShootTimer), With<EnemyRobot>>,
    assets: Res<GameAssets>,
    emitters: Res<SfxEmitters>,
) {
    let projectile_offset = ENEMY_A_CONFIG.projectile_spawn_offset;
    for (transform, mut state, mut shoot_timer) in query.iter_mut() {
        if state.state == EnemyRobotStateKind::Shooting {
            shoot_timer.timer += time.delta_secs();

            if !shoot_timer.fired && shoot_timer.timer >= shoot_timer.fire_delay {
                let dir = if state.facing_right {
                    Vec2::X
                } else {
                    -Vec2::X
                };
                spawn_enemy_projectile(
                    &mut commands,
                    &assets,
                    transform.translation + projectile_offset,
                    dir,
                );
                play_sfx_once(
                    &mut commands,
                    emitters.enemy_shoot,
                    assets.enemy_shoot_sfx.clone(),
                );
                shoot_timer.fired = true;
            }

            if shoot_timer.timer >= shoot_timer.pose_duration {
                shoot_timer.timer = 0.0;
                shoot_timer.fired = false;
                state.state = EnemyRobotStateKind::Running;
            }
        }
    }
}
