use super::components::*;
use super::config::BOSS_SETTINGS;
use super::events::*;
use crate::assets::GameAssets;
use crate::audio::{play_sfx_once, SfxEmitters};
use crate::constants::{DEFAULT_GRAVITY, GROUND_RECT_HEIGHT, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::effects::explosion_anim::spawn_explosion_c;
use crate::player::components::Player;
use crate::soundtrack::{SoundtrackController, TrackSetName};
use bevy::prelude::ChildOf;
use bevy::prelude::*;

pub fn boss_transition_to_stage2_start_system(
    mut commands: Commands,
    mut ev_reader: EventReader<BossStageTransitionEvent>,
    boss_ro_q: Query<(Entity, &Transform, &BossStage, &BossParts), With<Boss>>,
    camera_q: Query<&Transform, With<crate::components::MainCamera>>,
) {
    for _ in ev_reader.read() {
        let Ok((boss_e, boss_tf, stage, parts)) = boss_ro_q.single() else {
            continue;
        };
        if stage.0 != BossStageKind::TransitionToStage2 {
            continue;
        }

        let cam_x = camera_q
            .iter()
            .next()
            .map(|t| t.translation.x)
            .unwrap_or(0.0);
        let half_w = SCREEN_WIDTH * 0.5;
        let padding = BOSS_SETTINGS.transition.edge_padding;
        let left_edge = cam_x - half_w + padding;
        let right_edge = cam_x + half_w - padding;
        let to_left = (boss_tf.translation.x - left_edge).abs();
        let to_right = (right_edge - boss_tf.translation.x).abs();
        let go_right = to_right < to_left;
        let target_edge_x = if go_right { right_edge } else { left_edge };
        let dir = if go_right { 1.0 } else { -1.0 };
        let base_vel = BOSS_SETTINGS.transition.initial_velocity;
        let initial_vel = Vec2::new(base_vel.x * dir, base_vel.y);

        if let Some(cannon_e) = parts.get(BossPartKind::Cannon) {
            commands
                .entity(cannon_e)
                .insert(DetachCannonNow { go_right });
        }

        commands.entity(boss_e).insert(BossStage2TransitionState {
            timer: 0.0,
            phase: TransitionPhase::Blast,
            velocity: initial_vel,
            target_edge_x,
            downed_wait: BOSS_SETTINGS.transition.downed_wait,
            detach_to_right: go_right,
            pending_music_eta: 0.0,
        });
        commands
            .entity(boss_e)
            .insert(BossFacing { right: !go_right });

    }
}

pub fn boss_transition_to_stage2_update_system(
    mut commands: Commands,
    time: Res<Time>,
    player_q: Query<&Transform, (With<Player>, Without<Boss>)>,
    mut boss_q: Query<
        (
            Entity,
            &mut Transform,
            &mut BossStage,
            Option<&mut BossFacing>,
            &BossParts,
            &mut BossStage2TransitionState,
        ),
        (With<Boss>, Without<BossHead>),
    >,
    cannon_status_q: Query<(Option<&DetachedCannon>, Option<&DetachCannonNow>), With<BossCannon>>,
    mut controller: ResMut<SoundtrackController>,
) {
    let Ok((boss_e, mut boss_tf, mut stage, facing_opt, parts, mut trans)) =
        boss_q.single_mut()
    else {
        return;
    };
    if stage.0 != BossStageKind::TransitionToStage2 {
        return;
    }
    let Ok(player_tf) = player_q.single() else {
        return;
    };

    trans.timer += time.delta_secs();
    match trans.phase {
        TransitionPhase::Blast => {
            trans.velocity.y +=
                DEFAULT_GRAVITY * BOSS_SETTINGS.transition.gravity_scale * time.delta_secs();
            boss_tf.translation.x += trans.velocity.x * time.delta_secs();
            boss_tf.translation.y += trans.velocity.y * time.delta_secs();

            let reached_x = if trans.velocity.x >= 0.0 {
                boss_tf.translation.x >= trans.target_edge_x
            } else {
                boss_tf.translation.x <= trans.target_edge_x
            };
            if reached_x {
                boss_tf.translation.x = trans.target_edge_x;
                trans.velocity.x = 0.0;
            }

            let ground_y =
                -(SCREEN_HEIGHT * 0.5) + GROUND_RECT_HEIGHT + BOSS_SETTINGS.transition.ground_offset;
            if boss_tf.translation.y <= ground_y {
                boss_tf.translation.y = ground_y;
                trans.velocity = Vec2::ZERO;
                trans.phase = TransitionPhase::Downed;
            }
        }
        TransitionPhase::Downed => {
            if trans.downed_wait > 0.0 {
                trans.downed_wait -= time.delta_secs();
            }
            if trans.downed_wait <= 0.0 {
                let result = controller.request_track_set(TrackSetName::BossStage2);
                trans.phase = TransitionPhase::AwaitStage2Music;
                trans.pending_music_eta = result.eta_seconds.unwrap_or(0.0).max(0.0);
            }
        }
        TransitionPhase::AwaitStage2Music => {
            if trans.pending_music_eta > 0.0 {
                trans.pending_music_eta -= time.delta_secs();
            }

            if trans.pending_music_eta <= 0.0 {
                trans.phase = TransitionPhase::TurnToPlayer;
            }
        }
        TransitionPhase::TurnToPlayer => {
            if let Some(mut facing) = facing_opt {
                facing.right = (player_tf.translation.x - boss_tf.translation.x) >= 0.0;
            }
            stage.0 = BossStageKind::Stage2;

            let detach_to_right = trans.detach_to_right;

            if let Some(cannon_e) = parts.get(BossPartKind::Cannon) {
                if let Ok((detached, pending)) = cannon_status_q.get(cannon_e) {
                    if detached.is_none() && pending.is_none() {
                        commands
                            .entity(cannon_e)
                            .insert(DetachCannonNow {
                                go_right: detach_to_right,
                            })
                            .remove::<BossGunRotation>();
                    } else {
                        commands.entity(cannon_e).remove::<BossGunRotation>();
                    }
                }
            }

            commands
                .entity(boss_e)
                .insert((BossStage2State::default(), BossStage2Pose::default()));
            commands.entity(boss_e).remove::<BossStage1ShootingState>();
            commands.entity(boss_e).remove::<BossStage1MovementState>();
            commands.entity(boss_e).remove::<BossMovementTimer>();
            commands.entity(boss_e).remove::<BossStage1State>();
            commands
                .entity(boss_e)
                .remove::<BossStage2TransitionState>();
        }
    }
}

pub fn boss_cannon_detacher_system(
    mut commands: Commands,
    mut q: Query<(Entity, &GlobalTransform, &mut Transform, &DetachCannonNow), With<BossCannon>>,
) {
    for (e, gtf, mut tr, marker) in q.iter_mut() {
        let world_tr = gtf.compute_transform();
        commands.entity(e).remove::<ChildOf>();
        *tr = world_tr;
        let base_launch = BOSS_SETTINGS.transition.cannon_launch_velocity;
        let dir = if marker.go_right { 1.0 } else { -1.0 };
        let away = Vec2::new(base_launch.x * dir, base_launch.y);
        commands
            .entity(e)
            .insert(DetachedCannon {
                velocity: away,
                angular_velocity: BOSS_SETTINGS.transition.cannon_angular_velocity * dir,
                target_angle: None,
            })
            .remove::<DetachCannonNow>();
        tr.translation.z += 0.3;
    }
}

pub fn boss_transition_explosion_queue_system(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<GameAssets>,
    emitters: Res<SfxEmitters>,
    mut q: Query<(Entity, &mut BossTransitionExplosionQueue)>,
) {
    const BURST_OFFSETS: [Vec2; 3] = [
        Vec2::new(0.0, 0.0),
        Vec2::new(22.0, 14.0),
        Vec2::new(-18.0, 26.0),
    ];

    for (entity, mut queue) in q.iter_mut() {
        if queue.remaining == 0 {
            commands
                .entity(entity)
                .remove::<BossTransitionExplosionQueue>();
            continue;
        }

        queue.timer -= time.delta_secs();
        while queue.timer <= 0.0 && queue.remaining > 0 {
            let index = (queue.total - queue.remaining) as usize;
            let offset = BURST_OFFSETS.get(index).copied().unwrap_or(Vec2::ZERO);
            let pos = Vec3::new(
                queue.position.x + offset.x,
                queue.position.y + offset.y,
                queue.position.z,
            );

            spawn_explosion_c(&mut commands, &assets, pos);
            play_sfx_once(
                &mut commands,
                emitters.enemy_explosion,
                assets.enemy_explosion_sfx.clone(),
            );

            queue.remaining -= 1;
            if queue.remaining == 0 {
                commands
                    .entity(entity)
                    .remove::<BossTransitionExplosionQueue>();
                break;
            }

            queue.timer += queue.interval;
        }
    }
}
