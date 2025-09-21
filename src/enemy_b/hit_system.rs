use super::components::*;
use crate::assets::GameAssets;
use crate::audio::{SfxEmitters, play_sfx_once};
use crate::constants::PROJECTILE_SIZE;
use crate::player::components::Player;
use crate::projectile::components::{PlayerProjectile, Projectile, swept_projectile_hit_center};
use bevy::prelude::*;
use std::collections::HashSet;

pub fn enemy_b_hit_system(
    mut commands: Commands,
    mut q: Query<
        (
            Entity,
            &Transform,
            &mut EnemyBState,
            Option<&EnemyBSpawnProtection>,
        ),
        With<EnemyB>,
    >,
    projectiles: Query<
        (Entity, &Transform, &Projectile),
        (With<Projectile>, With<PlayerProjectile>),
    >,
    player_q: Query<&Transform, With<Player>>,
    assets: Res<GameAssets>,
    emitters: Res<SfxEmitters>,
) {
    let assets = assets.as_ref();
    let emitters = emitters.as_ref();
    let mut consumed: HashSet<Entity> = HashSet::new();

    for (enemy_e, enemy_tf, mut st, protection) in q.iter_mut() {
        if st.state == EnemyBStateKind::Hit {
            continue;
        }
        if protection.is_some() {
            continue;
        }
        let enemy_min = Vec2::new(
            enemy_tf.translation.x - ENEMY_B_WIDTH / 2.0,
            enemy_tf.translation.y - ENEMY_B_HEIGHT / 2.0,
        );
        let enemy_size = Vec2::new(ENEMY_B_WIDTH, ENEMY_B_HEIGHT);
        for (proj_e, proj_tf, projectile) in projectiles.iter() {
            if consumed.contains(&proj_e) {
                continue;
            }
            if let Some(_hit_center) = swept_projectile_hit_center(
                projectile.previous_translation,
                Vec2::new(proj_tf.translation.x, proj_tf.translation.y),
                Vec2::splat(PROJECTILE_SIZE),
                enemy_min,
                enemy_size,
            ) {
                st.state = EnemyBStateKind::Hit;
                commands.entity(proj_e).despawn();
                consumed.insert(proj_e);
                let dir = if let Ok(ptf) = player_q.single() {
                    if ptf.translation.x < enemy_tf.translation.x {
                        1.0
                    } else {
                        -1.0
                    }
                } else {
                    1.0
                };
                commands.entity(enemy_e).insert(EnemyBDespawnTimer::new());
                commands.entity(enemy_e).insert(EnemyBDeathBlink::new(dir));
                play_sfx_once(
                    &mut commands,
                    emitters.enemy_death,
                    assets.enemy_death_sfx.clone(),
                );
                break;
            }
        }
    }
}

pub fn enemy_b_despawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut q: Query<(Entity, &mut EnemyBDespawnTimer)>,
) {
    for (e, mut t) in q.iter_mut() {
        t.timer -= time.delta_secs();
        if t.timer <= 0.0 {
            commands.entity(e).despawn();
        }
    }
}

pub fn enemy_b_spawn_protection_system(
    time: Res<Time>,
    mut q: Query<(Entity, &mut EnemyBSpawnProtection)>,
    mut commands: Commands,
) {
    for (e, mut p) in q.iter_mut() {
        p.timer -= time.delta_secs();
        if p.timer <= 0.0 {
            commands.entity(e).remove::<EnemyBSpawnProtection>();
        }
    }
}
