use super::infected::{InfectedEnemyConfig, InfectedEnemyRobot, InfectedTransformAnim};
use super::robot_components::*;
use super::robot_components::{ENEMY_ROBOT_HEIGHT, ENEMY_ROBOT_WIDTH};
use crate::assets::GameAssets;
use crate::audio::{SfxEmitters, play_sfx_once};
use crate::constants::PROJECTILE_SIZE;
use crate::player::components::Player;
use crate::projectile::components::{
    PlayerProjectile, Projectile, ProjectileHitEvent, swept_projectile_hit_center,
};
use bevy::prelude::*;
use std::collections::HashSet;

pub fn enemy_robot_hit_system(
    mut commands: Commands,
    mut enemies: Query<
        (
            Entity,
            &Transform,
            &mut EnemyRobotState,
            Option<&EnemySpawnProtection>,
            Option<&InfectedEnemyRobot>,
        ),
        (With<EnemyRobot>, Without<Projectile>),
    >,
    projectiles: Query<
        (Entity, &Transform, &Projectile),
        (With<Projectile>, With<PlayerProjectile>),
    >,
    mut hit_writer: EventWriter<ProjectileHitEvent>,
    player_q: Query<&Transform, With<Player>>,
    infected_config: Res<InfectedEnemyConfig>,
    assets: Res<GameAssets>,
    emitters: Res<SfxEmitters>,
) {
    let projectile_data: Vec<(Entity, Vec2, Vec2)> = projectiles
        .iter()
        .map(|(entity, transform, projectile)| {
            (
                entity,
                projectile.previous_translation,
                Vec2::new(transform.translation.x, transform.translation.y),
            )
        })
        .collect();
    if projectile_data.is_empty() {
        return;
    }

    let mut consumed: HashSet<Entity> = HashSet::new();
    let assets = assets.as_ref();
    let emitters = emitters.as_ref();

    for (enemy_entity, enemy_transform, mut state, protection, infected) in enemies.iter_mut() {
        if consumed.len() == projectile_data.len() {
            break;
        }

        if state.state == EnemyRobotStateKind::Hit {
            continue;
        }
        if protection.is_some() {
            continue;
        }
        let enemy_pos = enemy_transform.translation;
        let enemy_rect_min = Vec2::new(
            enemy_pos.x - ENEMY_ROBOT_WIDTH / 2.0,
            enemy_pos.y - ENEMY_ROBOT_HEIGHT / 2.0,
        );
        let enemy_rect_size = Vec2::new(ENEMY_ROBOT_WIDTH, ENEMY_ROBOT_HEIGHT);
        for (proj_entity, start_center, end_center) in projectile_data.iter() {
            if consumed.contains(proj_entity) {
                continue;
            }
            if let Some(hit_center) = swept_projectile_hit_center(
                *start_center,
                *end_center,
                Vec2::splat(PROJECTILE_SIZE),
                enemy_rect_min,
                enemy_rect_size,
            ) {
                state.state = EnemyRobotStateKind::Hit;
                let hit_pos = Vec3::new(hit_center.x, hit_center.y, enemy_transform.translation.z);
                hit_writer.write(ProjectileHitEvent { position: hit_pos });
                commands.entity(*proj_entity).despawn();
                if infected.is_some() {
                    let baseline = enemy_transform.translation.y - ENEMY_ROBOT_HEIGHT * 0.5;
                    commands
                        .entity(enemy_entity)
                        .insert(InfectedTransformAnim::new(
                            baseline,
                            state.facing_right,
                            infected_config.transform_frame_times,
                        ));
                    play_sfx_once(
                        &mut commands,
                        emitters.enemy_death,
                        assets.enemy_death_sfx.clone(),
                    );
                    play_sfx_once(
                        &mut commands,
                        emitters.enemy_transform,
                        assets.enemy_transform_sfx.clone(),
                    );
                } else {
                    let dir = if let Ok(ptf) = player_q.single() {
                        if ptf.translation.x < enemy_transform.translation.x {
                            1.0
                        } else {
                            -1.0
                        }
                    } else {
                        1.0
                    };
                    commands
                        .entity(enemy_entity)
                        .insert(EnemyDespawnTimer::default());
                    commands
                        .entity(enemy_entity)
                        .insert(EnemyDeathBlink::new(dir));
                    play_sfx_once(
                        &mut commands,
                        emitters.enemy_death,
                        assets.enemy_death_sfx.clone(),
                    );
                }
                consumed.insert(*proj_entity);
                break;
            }
        }
    }
}
