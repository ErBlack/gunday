use super::components::*;
use super::config::BOSS_SETTINGS;
use super::util::approach_angle;
use crate::assets::GameAssets;
use crate::audio::{SfxEmitters, play_sfx_once};
use crate::player::components::Player;
use crate::projectile::projectile_spawning_system::spawn_boss_projectile;
use bevy::prelude::*;

pub fn boss_stage1_shooting_system(
    mut commands: Commands,
    time: Res<Time>,
    mut q: Query<
        (
            &GlobalTransform,
            &mut BossStage1ShootingState,
            &mut BossStage1MovementState,
            &BossParts,
            Option<&mut BossFacing>,
        ),
        With<Boss>,
    >,
    player_q: Query<&GlobalTransform, With<Player>>,
    boss_audio: Res<BossAudio>,
    assets: Res<GameAssets>,
    emitters: Res<SfxEmitters>,
    mut cannon_ps: ParamSet<(
        Query<&GlobalTransform, With<BossCannon>>,
        Query<(&mut Transform, &mut BossGunRotation), (With<BossCannon>, Without<Boss>)>,
    )>,
) {
    let Ok(player_gtf) = player_q.single() else {
        return;
    };
    for (boss_gtf, mut shooting, mut movement, parts, facing_opt) in q.iter_mut() {
        shooting.shoot_timer -= time.delta_secs();
        if shooting.aim_cooldown > 0.0 {
            shooting.aim_cooldown = (shooting.aim_cooldown - time.delta_secs()).max(0.0);
        }

        let mut facing_right_now = facing_opt.as_ref().map(|f| f.right).unwrap_or(true);
        if !shooting.shooting {
            let dx = player_gtf.translation().x - boss_gtf.translation().x;
            if dx.abs() > 0.001 {
                facing_right_now = dx >= 0.0;
                if let Some(mut facing) = facing_opt {
                    facing.right = facing_right_now;
                }
            }
        }

        if shooting.shoot_timer <= 0.0 && !shooting.aiming && !shooting.shooting {
            shooting.aiming = true;
            shooting.aim_timer = BOSS_SETTINGS.stage1.shooting.aim_duration;
            shooting.target = player_gtf.translation().truncate();
            shooting.shot_count = 0;
            shooting.locked_target = None;
        }

        let mut cannon_direction = None;
        if let Some(cannon_e) = parts.get(BossPartKind::Cannon) {
            if let Ok(gtf) = cannon_ps.p0().get(cannon_e) {
                let cannon_pos = gtf.translation().truncate();

                let target_pos = if shooting.shooting {
                    shooting
                        .locked_target
                        .unwrap_or(player_gtf.translation().truncate())
                } else if shooting.aim_cooldown > 0.0 {
                    shooting.target
                } else {
                    player_gtf.translation().truncate()
                };

                let to_target = (target_pos - cannon_pos).normalize_or_zero();
                let a = to_target.to_angle();
                let target_angle = if facing_right_now {
                    -a
                } else {
                    a + std::f32::consts::PI
                };

                if let Ok((mut tr, mut gun_rotation)) = cannon_ps.p1().get_mut(cannon_e) {
                    if shooting.aim_cooldown <= 0.0 {
                        let tracking_speed = if shooting.aiming || shooting.shooting {
                            gun_rotation.max_angular_speed * 0.01
                        } else {
                            gun_rotation.max_angular_speed
                        };

                        let max_rotation_this_frame = tracking_speed * time.delta_secs();
                        gun_rotation.current_angle = approach_angle(
                            gun_rotation.current_angle,
                            target_angle,
                            max_rotation_this_frame,
                        );
                    }
                    tr.rotation = Quat::from_rotation_z(gun_rotation.current_angle);

                    let world_angle = if facing_right_now {
                        -gun_rotation.current_angle
                    } else {
                        gun_rotation.current_angle - std::f32::consts::PI
                    };
                    cannon_direction = Some(Vec2::from_angle(world_angle));
                }
            }
        }

        if shooting.aiming {
            shooting.aim_timer -= time.delta_secs();
            if shooting.aim_timer <= 0.0 {
                shooting.aiming = false;
                shooting.shooting = true;
                shooting.locked_target = Some(player_gtf.translation().truncate());
                shooting.aim_timer = 0.0;
                shooting.shot_count = 0;
                shooting.aim_cooldown = 0.0;
                play_sfx_once(&mut commands, emitters.boss_shot, boss_audio.shot.clone());
            }
        }

        if shooting.shooting {
            shooting.aim_timer -= time.delta_secs();
            if let Some(cannon_e) = parts.get(BossPartKind::Cannon) {
                let cannon_pos = if let Ok(gtf) = cannon_ps.p0().get(cannon_e) {
                    gtf.translation().truncate()
                } else {
                    boss_gtf.translation().truncate()
                };

                let target = shooting
                    .locked_target
                    .unwrap_or(player_gtf.translation().truncate());
                let to_target = (target - cannon_pos).normalize_or_zero();
                if shooting.aim_timer <= 0.0 {
                    let tip_distance = BOSS_SETTINGS.stage1.shooting.tip_distance;
                    let mut dir = cannon_direction.unwrap_or(to_target);
                    if dir.length_squared() <= f32::EPSILON {
                        dir = to_target;
                    }
                    let dir = dir.normalize_or_zero();

                    let origin = Vec3::new(
                        cannon_pos.x + dir.x * tip_distance,
                        cannon_pos.y
                            + dir.y * tip_distance
                            + BOSS_SETTINGS.stage1.shooting.muzzle_vertical_offset,
                        boss_gtf.translation().z + 0.2,
                    );

                    spawn_boss_projectile(&mut commands, &assets, origin, dir);
                    shooting.shot_count += 1;
                    if shooting.shot_count >= shooting.shots_per_burst {
                        shooting.shooting = false;
                        shooting.shoot_timer = BOSS_SETTINGS.stage1.shooting.shoot_cooldown;
                        shooting.aim_cooldown = 0.0;

                        if fastrand::f32() < 0.5 {
                            movement.pending_move_request = true;
                        }
                    } else {
                        shooting.aim_timer = BOSS_SETTINGS.stage1.shooting.inter_shot_delay;
                    }
                }
            }
        }
    }
}
