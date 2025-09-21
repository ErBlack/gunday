use super::components::*;
use super::config::BOSS_SETTINGS;
use super::events::*;
use crate::assets::GameAssets;
use crate::audio::{SfxEmitters, play_sfx_once};
use crate::constants::{PROJECTILE_SIZE, Z_PROJECTILES};
use crate::effects::explosion_anim::spawn_explosion_c;
use crate::projectile::components::{PlayerProjectile, Projectile, swept_projectile_hit_center};
use bevy::prelude::*;

pub fn boss_spine_hit_system(
    mut commands: Commands,
    audio: Res<BossAudio>,
    assets: Res<GameAssets>,
    emitters: Res<SfxEmitters>,
    mut bosses: Query<(
        Entity,
        &GlobalTransform,
        &mut BossStage,
        &BossParts,
    )>,
    mut spine_q: Query<(
        &mut BossSpine,
        &GlobalTransform,
        &Transform,
        Option<&mut BossSpineHitAnimation>,
    )>,
    mut shooting_q: Query<&mut BossStage1ShootingState, With<Boss>>,
    cannon_state_q: Query<&BossGunRotation>,
    projectiles: Query<
        (Entity, &Transform, &Projectile),
        (With<Projectile>, With<PlayerProjectile>),
    >,
    mut ev_stage: EventWriter<BossStageTransitionEvent>,
) {
    for (boss_e, _boss_gtf, mut stage, parts) in bosses.iter_mut() {
        let Some(spine_e) = parts.get(BossPartKind::Spine) else {
            continue;
        };
        let Ok((mut spine, spine_gtf, spine_tf, anim_opt)) = spine_q.get_mut(spine_e) else {
            continue;
        };
        let size = BOSS_SETTINGS.spine.size;
        let center = spine_gtf.translation().truncate();
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
                spine.hp = spine.hp.saturating_sub(1);
                commands.entity(proj_e).despawn();
                play_sfx_once(&mut commands, emitters.boss_hit, audio.hit.clone());
                let current_rotation = spine_tf.rotation.to_euler(EulerRot::XYZ).2;
                let stored_gun_angle = parts
                    .get(BossPartKind::Cannon)
                    .and_then(|c| cannon_state_q.get(c).ok())
                    .map(|g| g.current_angle);

                if let Some(mut anim) = anim_opt {
                    anim.timer = BOSS_SETTINGS.spine.hit_animation_duration;
                    if let Some(angle) = stored_gun_angle {
                        anim.stored_gun_angle = Some(angle);
                    }
                    anim.cannon_entity = parts.get(BossPartKind::Cannon);
                } else {
                    commands.entity(spine_e).insert(BossSpineHitAnimation {
                        timer: BOSS_SETTINGS.spine.hit_animation_duration,
                        original_rotation: current_rotation,
                        stored_gun_angle,
                        cannon_entity: parts.get(BossPartKind::Cannon),
                    });
                }
                if let Ok(mut shooting_state) = shooting_q.get_mut(boss_e) {
                    shooting_state.aim_cooldown = shooting_state
                        .aim_cooldown
                        .max(BOSS_SETTINGS.stage1.shooting.aim_cooldown_hit_penalty);
                    if let Some(target) = shooting_state.locked_target {
                        shooting_state.target = target;
                    }
                }
                if spine.hp == 0 {
                    let burst_total = BOSS_SETTINGS.transition.explosion_burst_count.max(1);
                    let interval = BOSS_SETTINGS.transition.explosion_interval.max(0.01);
                    let mut pos = spine_gtf.translation();
                    pos.y += BOSS_SETTINGS.spine.explosion_offset_y;
                    let z = Z_PROJECTILES + 0.5;
                    let explosion_pos = Vec3::new(pos.x, pos.y, z);
                    spawn_explosion_c(&mut commands, &assets, explosion_pos);
                    play_sfx_once(
                        &mut commands,
                        emitters.enemy_explosion,
                        assets.enemy_explosion_sfx.clone(),
                    );
                    if burst_total > 1 {
                        commands.entity(boss_e).insert(BossTransitionExplosionQueue {
                            position: explosion_pos,
                            remaining: burst_total - 1,
                            total: burst_total,
                            timer: interval,
                            interval,
                        });
                    }
                    stage.0 = BossStageKind::TransitionToStage2;
                    ev_stage.write(BossStageTransitionEvent);
                }
                break;
            }
        }
    }
}
