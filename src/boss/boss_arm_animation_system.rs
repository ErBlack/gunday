use super::components::*;
use super::config::BOSS_SETTINGS;
use super::util::approach_angle;
use bevy::prelude::*;

pub fn boss_arm_animation_system(
    time: Res<Time>,
    boss_q: Query<
        (
            Option<&BossStage1ShootingState>,
            Option<&BossMovementTimer>,
        ),
        With<Boss>,
    >,
    mut arm_q: Query<(&mut Transform, &BossArm), Without<Boss>>,
) {
    let Ok((shooting_opt, mov_opt)) = boss_q.single() else {
        return;
    };
    let (shooting, mov) = match (shooting_opt, mov_opt) {
        (Some(shooting), Some(mov)) => (shooting, mov),
        _ => return,
    };

    let arms_cfg = BOSS_SETTINGS.arms;

    let (mut target_left, mut target_right) = if shooting.shooting {
        let base_left = arms_cfg.shooting_base_left_deg.to_radians();
        let base_right = arms_cfg.shooting_base_right_deg.to_radians();

        let oscillation = arms_cfg.shooting_oscillation_amplitude_deg.to_radians()
            * (2.0 * std::f32::consts::PI * arms_cfg.shooting_oscillation_frequency * mov.timer)
                .sin();
        (base_left + oscillation, base_right + oscillation)
    } else if shooting.aiming {
        (
            arms_cfg.aiming_left_deg.to_radians(),
            arms_cfg.aiming_right_deg.to_radians(),
        )
    } else {
        (
            arms_cfg.relaxed_left_deg.to_radians(),
            arms_cfg.relaxed_right_deg.to_radians(),
        )
    };

    if !shooting.aiming && !shooting.shooting {
        let freq = arms_cfg.idle_oscillation_frequency;
        let oscillation = arms_cfg.idle_oscillation_amplitude_deg.to_radians()
            * (2.0 * std::f32::consts::PI * freq * mov.timer).sin();
        target_left += oscillation;
        target_right += oscillation;
    }

    for (mut transform, arm) in arm_q.iter_mut() {
        let target_angle = if arm.is_left {
            target_left
        } else {
            target_right
        };
        let max_rotation_this_frame = arms_cfg.rotation_speed * time.delta_secs();
        let current_angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        let new_angle = approach_angle(current_angle, target_angle, max_rotation_this_frame);
        transform.rotation = Quat::from_rotation_z(new_angle);
    }
}

pub fn boss_arm_transition_pose_system(
    mut arm_q: Query<(&mut Transform, &BossArm), Without<Boss>>,
) {
    let stretched_angle = BOSS_SETTINGS.arms.transition_stretched_angle_deg.to_radians();
    for (mut transform, _) in arm_q.iter_mut() {
        transform.rotation = Quat::from_rotation_z(stretched_angle);
    }
}
