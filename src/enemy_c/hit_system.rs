use super::components::{
    ENEMY_C_HEIGHT, ENEMY_C_WIDTH, EnemyC, EnemyCDeathBlink, EnemyCDespawnTimer, EnemyCHitFlash,
    EnemyCHitPoints, EnemyCState, EnemyCStateKind, EnemyCVelocity,
};
use super::config::ENEMY_C_CONFIG;
use crate::assets::GameAssets;
use crate::audio::{SfxEmitters, play_sfx_once};
use crate::constants::PROJECTILE_SIZE;
use crate::effects::explosion_anim::spawn_explosion_b;
use crate::player::components::Player;
use crate::projectile::components::{
    PlayerProjectile, Projectile, ProjectileHitEvent, swept_projectile_hit_center,
};
use bevy::prelude::*;
use std::collections::HashSet;

pub fn enemy_c_hit_system(
    mut commands: Commands,
    mut enemies: Query<
        (
            Entity,
            &Transform,
            &mut EnemyCState,
            &mut EnemyCHitPoints,
            &mut EnemyCVelocity,
        ),
        With<EnemyC>,
    >,
    projectiles: Query<
        (Entity, &Transform, &Projectile),
        (With<Projectile>, With<PlayerProjectile>),
    >,
    mut hit_writer: EventWriter<ProjectileHitEvent>,
    player_q: Query<&Transform, With<Player>>,
    game_assets: Res<GameAssets>,
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

    let player_x = player_q.iter().next().map(|tf| tf.translation.x);

    for (enemy_entity, transform, mut state, mut hp, mut velocity) in enemies.iter_mut() {
        if consumed.len() == projectile_data.len() {
            break;
        }

        if matches!(state.state, EnemyCStateKind::Dying) {
            continue;
        }

        let enemy_min = Vec2::new(
            transform.translation.x - ENEMY_C_WIDTH * 0.5,
            transform.translation.y,
        );
        let enemy_size = Vec2::new(ENEMY_C_WIDTH, ENEMY_C_HEIGHT);

        for (proj_entity, start_center, end_center) in projectile_data.iter() {
            if consumed.contains(proj_entity) {
                continue;
            }

            if let Some(hit_center) = swept_projectile_hit_center(
                *start_center,
                *end_center,
                Vec2::splat(PROJECTILE_SIZE),
                enemy_min,
                enemy_size,
            ) {
                consumed.insert(*proj_entity);

                hit_writer.write(ProjectileHitEvent {
                    position: Vec3::new(hit_center.x, hit_center.y, transform.translation.z),
                });
                commands.entity(*proj_entity).despawn();

                let defeated = hp.take_hit();

                if defeated {
                    state.state = EnemyCStateKind::Dying;
                    state.time_in_state = 0.0;
                    velocity.velocity.x = 0.0;
                    velocity.velocity.y = 0.0;

                    let dir = player_x
                        .map(|px| {
                            if px < transform.translation.x {
                                1.0
                            } else {
                                -1.0
                            }
                        })
                        .unwrap_or(1.0);

                    let explosion_pos = Vec3::new(
                        transform.translation.x,
                        transform.translation.y + ENEMY_C_CONFIG.explosion_vertical_offset,
                        transform.translation.z + 0.1,
                    );
                    spawn_explosion_b(&mut commands, &game_assets, explosion_pos);

                    play_sfx_once(
                        &mut commands,
                        emitters.enemy_death,
                        game_assets.enemy_c_death_sfx.clone(),
                    );

                    commands
                        .entity(enemy_entity)
                        .insert(EnemyCDespawnTimer::new())
                        .insert(EnemyCDeathBlink::new(dir))
                        .remove::<EnemyCHitFlash>();
                } else {
                    state.time_in_state = 0.0;
                    commands.entity(enemy_entity).insert(EnemyCHitFlash::new());
                }

                break;
            }
        }
    }
}
