use super::components::*;
use super::config::MINIBOSS_CONFIG;
use crate::constants::DEFAULT_GRAVITY;
use bevy::prelude::*;

pub fn miniboss_grenade_physics_system(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &mut MinibossGrenade)>,
) {
    let dt = time.delta_secs();
    for (mut transform, mut grenade) in q.iter_mut() {
        grenade.velocity.y += DEFAULT_GRAVITY * dt;
        transform.translation.x += grenade.velocity.x * dt;
        transform.translation.y += grenade.velocity.y * dt;

        grenade.rotation_timer += dt;
        while grenade.rotation_timer >= 1.0 / MINIBOSS_CONFIG.grenade_rotation_fps {
            grenade.rotation_timer -= 1.0 / MINIBOSS_CONFIG.grenade_rotation_fps;
            let current = transform.rotation.to_euler(EulerRot::XYZ).2;
            transform.rotation =
                Quat::from_rotation_z(current + MINIBOSS_CONFIG.grenade_rotation_step);
        }
    }
}
