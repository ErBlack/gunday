use super::components::*;
use super::config::BOSS_SETTINGS;
use bevy::prelude::*;

pub fn boss_torso_animation_system(
    boss_q: Query<
        (
            Option<&BossStage1ShootingState>,
            Option<&BossMovementTimer>,
        ),
        With<Boss>,
    >,
    mut torso_q: Query<&mut Transform, With<BossTorso>>,
) {
    let Ok((shooting_opt, mov_opt)) = boss_q.single() else {
        return;
    };
    let (shooting, mov) = match (shooting_opt, mov_opt) {
        (Some(shooting), Some(mov)) => (shooting, mov),
        _ => return,
    };

    let torso_cfg = BOSS_SETTINGS.torso;

    if shooting.aiming || (!shooting.aiming && !shooting.shooting) {
        let oscillation = torso_cfg.oscillation_amplitude
            * (2.0 * std::f32::consts::PI * torso_cfg.oscillation_frequency * mov.timer).sin();

        for mut transform in torso_q.iter_mut() {
            transform.translation.x = oscillation;
            transform.rotation = Quat::IDENTITY;
        }
    } else {
        for mut transform in torso_q.iter_mut() {
            transform.translation.x = 0.0;
            transform.rotation = Quat::IDENTITY;
        }
    }
}

pub fn boss_torso_downed_pose_system(mut torso_q: Query<&mut Transform, With<BossTorso>>) {
    let torso_cfg = BOSS_SETTINGS.torso;
    for mut transform in torso_q.iter_mut() {
        transform.translation.x = torso_cfg.stage2_base_offset.x;
        transform.translation.y = torso_cfg.stage2_base_offset.y;
        transform.rotation = Quat::from_rotation_z(torso_cfg.stage2_base_rotation_deg.to_radians());
    }
}
