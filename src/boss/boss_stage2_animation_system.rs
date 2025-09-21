use super::components::*;
use super::config::BOSS_SETTINGS;
use bevy::prelude::*;
pub fn boss_stage2_animation_system(
    mut bosses: Query<(&BossParts, &BossStage2State, &mut BossStage2Pose), With<Boss>>,
    mut tf_query: Query<&mut Transform, Without<Boss>>,
) {
    for (parts, s2, mut pose) in bosses.iter_mut() {
        if !pose.initialized {
            pose.initialized = true;
        }
        let stage2_cfg = BOSS_SETTINGS.stage2;
        let arm_amp = stage2_cfg.arm_swing_amplitude;
        let arm_freq = stage2_cfg.arm_swing_frequency;
        let spine_amp = stage2_cfg.spine_twitch_amplitude;
        let spine_freq = stage2_cfg.spine_twitch_frequency;
        let arm_base_shift = stage2_cfg.arm_base_shift_deg;
        if let Some(left) = parts.get(BossPartKind::LeftArm) {
            if let Ok(mut tr) = tf_query.get_mut(left) {
                let base = (stage2_cfg.left_arm_base_deg + arm_base_shift).to_radians();
                tr.rotation = Quat::from_rotation_z(
                    base + arm_amp
                        * (2.0 * std::f32::consts::PI * arm_freq * (s2.crawl_timer)).sin(),
                );
            }
        }
        if let Some(right) = parts.get(BossPartKind::RightArm) {
            if let Ok(mut tr) = tf_query.get_mut(right) {
                let base = (stage2_cfg.right_arm_base_deg + arm_base_shift).to_radians();
                let phase = stage2_cfg.right_arm_phase_offset_deg.to_radians();
                tr.rotation = Quat::from_rotation_z(
                    base + arm_amp
                        * (2.0 * std::f32::consts::PI * arm_freq * (s2.crawl_timer)
                            + phase)
                            .sin(),
                );
            }
        }
        if let Some(spine) = parts.get(BossPartKind::Spine) {
            if let Ok(mut tr) = tf_query.get_mut(spine) {
                let base = stage2_cfg.spine_base_deg.to_radians();
                tr.rotation = Quat::from_rotation_z(
                    base + spine_amp
                        * (2.0 * std::f32::consts::PI
                            * spine_freq
                            * (s2.crawl_timer + stage2_cfg.spine_phase_offset))
                            .sin(),
                );
            }
        }
    }
}
