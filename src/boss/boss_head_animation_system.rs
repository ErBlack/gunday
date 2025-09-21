use super::components::*;
use super::config::BOSS_SETTINGS;
use super::util::approach_angle;
use crate::player::components::Player;
use bevy::prelude::*;

pub fn boss_head_animation_system(
    time: Res<Time>,
    boss_q: Query<
        (
            &BossStage,
            Option<&BossStage2TransitionState>,
            &GlobalTransform,
        ),
        With<Boss>,
    >,
    mut head_q: Query<(&GlobalTransform, &mut Transform), With<BossHead>>,
    player_q: Query<&GlobalTransform, With<Player>>,
) {
    let Ok(player_gtf) = player_q.single() else {
        return;
    };
    let Ok((stage, trans_opt, _boss_gtf)) = boss_q.single() else {
        return;
    };
    let head_cfg = BOSS_SETTINGS.head;
    let neutral_angle = head_cfg.neutral_angle_deg.to_radians();
    let rotation_range = head_cfg.rotation_range_deg.to_radians();
    let downed_angle = head_cfg.downed_angle_deg.to_radians();

    if stage.0 == BossStageKind::Exploding {
        return;
    }

    if stage.0 == BossStageKind::TransitionToStage2 {
        if let Some(trans) = trans_opt {
            match trans.phase {
                TransitionPhase::TurnToPlayer => {
                    for (head_gtf, mut head_transform) in head_q.iter_mut() {
                        let head_pos = head_gtf.translation().truncate();
                        let to_player =
                            (player_gtf.translation().truncate() - head_pos).normalize_or_zero();
                        let target_angle = to_player.to_angle();
                        head_transform.rotation = Quat::from_rotation_z(target_angle);
                    }
                    return;
                }
                _ => {
                    for (_head_gtf, mut head_transform) in head_q.iter_mut() {
                        head_transform.rotation = Quat::from_rotation_z(downed_angle);
                    }
                    return;
                }
            }
        } else {
            for (_head_gtf, mut head_transform) in head_q.iter_mut() {
                head_transform.rotation = Quat::from_rotation_z(downed_angle);
            }
            return;
        }
    }

    for (head_gtf, mut head_transform) in head_q.iter_mut() {
        let head_pos = head_gtf.translation().truncate();
        let to_player = (player_gtf.translation().truncate() - head_pos).normalize_or_zero();
        let world_angle = to_player.to_angle();
        let target_angle = -world_angle;
        let target_angle_with_offset = target_angle + neutral_angle;
        let clamped_angle =
            target_angle_with_offset.clamp(neutral_angle - rotation_range, neutral_angle + rotation_range);

        let current_angle = head_transform.rotation.to_euler(EulerRot::XYZ).2;
        let max_rotation_this_frame = head_cfg.rotation_speed * time.delta_secs();
        let new_angle = approach_angle(current_angle, clamped_angle, max_rotation_this_frame);
        head_transform.rotation = Quat::from_rotation_z(new_angle);
    }
}
