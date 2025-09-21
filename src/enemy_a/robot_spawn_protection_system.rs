use super::robot_components::EnemySpawnProtection;
use crate::components::MainCamera;
use crate::constants::SCREEN_WIDTH;
use bevy::prelude::*;

pub fn enemy_spawn_protection_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut EnemySpawnProtection, &Transform)>,
    camera_q: Query<&Transform, With<MainCamera>>,
    mut commands: Commands,
) {
    let delta = time.delta_secs();
    if delta <= 0.0 {
        return;
    }

    let cam_x = camera_q.iter().next().map(|tf| tf.translation.x);
    let half_w = SCREEN_WIDTH * 0.5;

    for (entity, mut prot, transform) in query.iter_mut() {
        prot.timer -= delta;
        let center_on_screen = cam_x
            .map(|cx| (transform.translation.x - cx).abs() <= half_w)
            .unwrap_or(false);
        if prot.timer <= 0.0 || center_on_screen {
            commands.entity(entity).remove::<EnemySpawnProtection>();
        }
    }
}
