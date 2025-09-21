use super::components::*;
use super::config::BOSS_SETTINGS;
use super::util::{approach_angle, shortest_angle_diff};
use crate::assets::GameAssets;
use crate::audio::{SfxEmitters, play_sfx_once};
use crate::effects::explosion_anim::{spawn_explosion_c, spawn_explosion_d};
use crate::systems::PlayerControl;
use bevy::prelude::*;

const PRE_EXPLOSION_COUNT: usize = 6;
const PRE_EXPLOSION_INTERVAL: f32 = 0.28;
const PRE_EXPLOSION_OFFSETS: [Vec2; PRE_EXPLOSION_COUNT] = [
    Vec2::new(-58.0, 32.0),
    Vec2::new(52.0, 26.0),
    Vec2::new(-40.0, -8.0),
    Vec2::new(46.0, -18.0),
    Vec2::new(-14.0, 38.0),
    Vec2::new(18.0, 6.0),
];
const FINAL_EXPLOSION_OFFSET: Vec2 = Vec2::new(0.0, 28.0);

pub fn boss_explosion_system(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<GameAssets>,
    audio: Option<Res<BossAudio>>,
    emitters: Res<SfxEmitters>,
    mut q: Query<(Entity, &mut BossExplodingState, &BossParts, &Transform), With<Boss>>,
    mut tf_q: Query<(&GlobalTransform, &mut Transform), Without<BossExplodingState>>,
    mut control: Option<ResMut<PlayerControl>>,
) {
    let dt = time.delta_secs();
    for (e, mut ex, parts, root_transform) in q.iter_mut() {
        ex.timer += dt;

        if !ex.defeat_sound_played {
            if let Some(a) = &audio {
                play_sfx_once(&mut commands, emitters.boss_defeat, a.defeat.clone());
            }
            ex.defeat_sound_played = true;
            if let Some(c) = control.as_deref_mut() {
                c.enabled = false;
            }
        }

        let root_pos = root_transform.translation;

        let mut trigger_final_blast = false;
        if !ex.final_blast_triggered {
            ex.pre_explosion_timer -= dt;
            while ex.pre_explosion_timer <= 0.0
                && (ex.pre_explosions_spawned as usize) < PRE_EXPLOSION_COUNT
            {
                let index = ex.pre_explosions_spawned as usize;
                let offset = PRE_EXPLOSION_OFFSETS
                    .get(index)
                    .copied()
                    .unwrap_or(Vec2::ZERO);
                let pos = Vec3::new(
                    root_pos.x + offset.x,
                    root_pos.y + offset.y,
                    root_pos.z + 0.4,
                );
                spawn_explosion_c(&mut commands, &assets, pos);
                play_sfx_once(
                    &mut commands,
                    emitters.enemy_explosion,
                    assets.enemy_explosion_sfx.clone(),
                );

                ex.pre_explosions_spawned += 1;

                if (ex.pre_explosions_spawned as usize) < PRE_EXPLOSION_COUNT {
                    ex.pre_explosion_timer += PRE_EXPLOSION_INTERVAL;
                } else {
                    trigger_final_blast = true;
                    break;
                }
            }
        } else if !ex.converted {
            trigger_final_blast = true;
        }

        if trigger_final_blast && !ex.final_blast_triggered {
            let pos = Vec3::new(
                root_pos.x + FINAL_EXPLOSION_OFFSET.x,
                root_pos.y + FINAL_EXPLOSION_OFFSET.y,
                root_pos.z + 0.6,
            );
            spawn_explosion_d(&mut commands, &assets, pos);
            play_sfx_once(
                &mut commands,
                emitters.enemy_explosion,
                assets.enemy_explosion_sfx.clone(),
            );
            ex.final_blast_triggered = true;
        }

        if !ex.final_blast_triggered {
            continue;
        }

        if !ex.converted {
            let _ = BOSS_SETTINGS.explosion.part_launch.cannon;
            let ordered_parts = [
                BossPartKind::Head,
                BossPartKind::LeftArm,
                BossPartKind::RightArm,
                BossPartKind::Spine,
                BossPartKind::Torso,
                BossPartKind::Cannon,
            ];
            let mut released = 0;
            for kind in ordered_parts.iter() {
                let Some(pe) = parts.get(*kind) else {
                    continue;
                };
                released += 1;
                if let Ok((gtf, mut tr)) = tf_q.get_mut(pe) {
                    let world = gtf.compute_transform();
                    *tr = world;
                }
                commands.entity(pe).remove::<bevy::prelude::ChildOf>();
                let mut ec = commands.entity(pe);
                let launch = match kind {
                    BossPartKind::Head => Some(BOSS_SETTINGS.explosion.part_launch.head),
                    BossPartKind::LeftArm => Some(BOSS_SETTINGS.explosion.part_launch.left_arm),
                    BossPartKind::RightArm => Some(BOSS_SETTINGS.explosion.part_launch.right_arm),
                    BossPartKind::Spine => Some(BOSS_SETTINGS.explosion.part_launch.spine),
                    BossPartKind::Torso => Some(BOSS_SETTINGS.explosion.part_launch.torso),
                    BossPartKind::Cannon => None,
                };
                ec.remove::<DetachedCannon>();
                if let Some(launch) = launch {
                    ec.insert(ExplodingPart {
                        velocity: launch.velocity,
                        angular_velocity: launch.angular_velocity,
                    });
                }
            }
            ex.released = released;
            ex.converted = true;
        }

        if !ex.win_started && ex.timer >= BOSS_SETTINGS.explosion.win_start_delay {
            if let Some(a) = &audio {
                play_sfx_once(&mut commands, emitters.boss_win, a.win.clone());
            }
            ex.win_started = true;
            ex.win_timer = 0.0;
        }
        if ex.win_started {
            ex.win_timer += dt;
            if ex.win_timer >= BOSS_SETTINGS.explosion.win_exit_delay && !ex.result_sent {
                ex.result_sent = true;
                #[cfg(target_arch = "wasm32")]
                {
                    crate::systems::browser_events::send_game_result(true);
                }
                commands.entity(e).despawn();
                commands.trigger(bevy::app::AppExit::Success);
            }
        }
    }
}

pub fn detached_cannon_system(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &mut DetachedCannon)>,
) {
    for (mut tr, mut dc) in q.iter_mut() {
        dc.velocity.y += crate::constants::DEFAULT_GRAVITY
            * BOSS_SETTINGS.transition.cannon_gravity_scale
            * time.delta_secs();
        tr.translation.x += dc.velocity.x * time.delta_secs();
        tr.translation.y += dc.velocity.y * time.delta_secs();

        let current_angle = tr.rotation.to_euler(EulerRot::XYZ).2;
        if let Some(target) = dc.target_angle {
            let rotation_speed = BOSS_SETTINGS.transition.cannon_settle_rotation_speed;
            let max_rotation = rotation_speed * time.delta_secs();
            let new_angle = approach_angle(current_angle, target, max_rotation);
            tr.rotation = Quat::from_rotation_z(new_angle);
            if shortest_angle_diff(target, new_angle).abs() < 0.01 {
                dc.target_angle = None;
                tr.rotation = Quat::from_rotation_z(target);
            }
        } else {
            tr.rotation =
                Quat::from_rotation_z(current_angle + dc.angular_velocity * time.delta_secs());
        }

        let ground_y =
            -(crate::constants::SCREEN_HEIGHT * 0.5) + crate::constants::GROUND_RECT_HEIGHT;

        if dc.target_angle.is_none()
            && tr.translation.y
                <= ground_y + BOSS_SETTINGS.transition.cannon_upright_trigger_height
        {
            dc.target_angle = Some(0.0);
        }
        let snap_height = ground_y + BOSS_SETTINGS.transition.cannon_ground_snap_offset;
        if tr.translation.y <= snap_height {
            tr.translation.y = snap_height;
            dc.velocity = Vec2::ZERO;
            dc.angular_velocity = 0.0;
            dc.target_angle = Some(0.0);
        }
    }
}

pub fn exploding_part_system(time: Res<Time>, mut q: Query<(&mut Transform, &mut ExplodingPart)>) {
    for (mut tr, mut p) in q.iter_mut() {
        p.velocity.y += crate::constants::DEFAULT_GRAVITY
            * BOSS_SETTINGS.explosion.part_gravity_scale
            * time.delta_secs();
        tr.translation.x += p.velocity.x * time.delta_secs();
        tr.translation.y += p.velocity.y * time.delta_secs();
        let current_angle = tr.rotation.to_euler(EulerRot::XYZ).2;
        tr.rotation = Quat::from_rotation_z(current_angle + p.angular_velocity * time.delta_secs());
    }
}
