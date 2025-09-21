use super::components::*;
use super::config::BOSS_SETTINGS;
use bevy::prelude::*;

pub fn boss_spine_hit_animation_system(
    time: Res<Time>,
    mut commands: Commands,
    mut params: ParamSet<(
        Query<
            (
                Entity,
                &mut Transform,
                &mut Sprite,
                &mut BossSpineHitAnimation,
            ),
            (With<BossSpine>, Without<super::components::ExplodingPart>),
        >,
        Query<(&mut Transform, &mut BossGunRotation), With<BossCannon>>,
    )>,
) {
    let mut restore_requests: Vec<(Entity, f32)> = Vec::new();

    for (entity, mut transform, mut sprite, mut animation) in params.p0().iter_mut() {
        animation.timer -= time.delta_secs();

        if animation.timer <= 0.0 {
            transform.rotation = Quat::from_rotation_z(animation.original_rotation);
            sprite.color = Color::WHITE;
            if let (Some(angle), Some(cannon)) =
                (animation.stored_gun_angle, animation.cannon_entity)
            {
                restore_requests.push((cannon, angle));
            }
            commands.entity(entity).remove::<BossSpineHitAnimation>();
            continue;
        }

        let duration = BOSS_SETTINGS.spine.hit_animation_duration;
        let remaining = animation.timer.clamp(0.0, duration);
        let elapsed = duration - remaining;
        let progress = (elapsed / duration).clamp(0.0, 1.0);

        let rotation_cycles = BOSS_SETTINGS.spine.hit_rotation_cycles;
        let rotation_offset = (progress * std::f32::consts::TAU * rotation_cycles).sin()
            * BOSS_SETTINGS.spine.hit_rotation_amplitude_deg.to_radians();

        transform.rotation = Quat::from_rotation_z(animation.original_rotation + rotation_offset);

        let flash_phase = ((elapsed * BOSS_SETTINGS.spine.hit_flash_toggle_hz) as i32) % 2;
        sprite.color = if flash_phase == 0 {
            Color::WHITE
        } else {
            Color::srgba(1.0, 1.0, 1.0, 0.3)
        };
    }

    for (cannon, angle) in restore_requests {
        if let Ok((mut cannon_tr, mut gun_rot)) = params.p1().get_mut(cannon) {
            gun_rot.current_angle = angle;
            cannon_tr.rotation = Quat::from_rotation_z(angle);
        }
    }
}
