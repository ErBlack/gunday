use super::components::*;
use super::config::BOSS_SETTINGS;
use super::events::*;
use crate::assets::GameAssets;
use crate::audio::{SfxEmitters, play_sfx_once};
use crate::constants::{PROJECTILE_SIZE, Z_PROJECTILES};
use crate::effects::explosion_anim::spawn_explosion_d;
use crate::projectile::components::{PlayerProjectile, Projectile, swept_projectile_hit_center};
use bevy::prelude::*;

pub fn boss_head_hit_system(
    mut commands: Commands,
    audio: Res<BossAudio>,
    assets: Res<GameAssets>,
    emitters: Res<SfxEmitters>,
    mut bosses: Query<(
        Entity,
        &mut BossStage,
        &mut BossStage2State,
        &BossParts,
    )>,
    head_q: Query<&GlobalTransform, With<BossHead>>,
    projectiles: Query<
        (Entity, &Transform, &Projectile),
        (With<Projectile>, With<PlayerProjectile>),
    >,
    mut ev_defeat: EventWriter<BossDefeatedEvent>,
) {
    for (boss_e, mut stage, mut s2, parts) in bosses.iter_mut() {
        let Some(head_e) = parts.get(BossPartKind::Head) else {
            continue;
        };
        let Ok(head_tf) = head_q.get(head_e) else {
            continue;
        };
    let size = BOSS_SETTINGS.head.size;
        let center = head_tf.translation().truncate();
        let min = center - size * 0.5;
        for (proj_e, proj_tf, projectile) in projectiles.iter() {
            if swept_projectile_hit_center(
                projectile.previous_translation,
                proj_tf.translation.truncate(),
                Vec2::splat(PROJECTILE_SIZE),
                min,
                size,
            )
            .is_some()
            {
                s2.head_hp = s2.head_hp.saturating_sub(1);
                commands.entity(proj_e).despawn();
                play_sfx_once(&mut commands, emitters.boss_hit, audio.hit.clone());
                if s2.head_hp == 0 {
                    let mut pos = head_tf.translation();
                    pos.y += BOSS_SETTINGS.head.explosion_offset_y;
                    let z = Z_PROJECTILES + 0.5;
                    spawn_explosion_d(&mut commands, &assets, Vec3::new(pos.x, pos.y, z));
                    stage.0 = BossStageKind::Exploding;
                    commands
                        .entity(boss_e)
                        .insert(BossExplodingState::default());
                    ev_defeat.write(BossDefeatedEvent);
                }
                break;
            }
        }
    }
}
