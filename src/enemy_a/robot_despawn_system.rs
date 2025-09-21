use super::robot_components::{EnemyDespawnTimer, EnemyRobot};
use bevy::prelude::*;

pub fn enemy_robot_despawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut EnemyDespawnTimer), With<EnemyRobot>>,
) {
    for (entity, mut timer) in query.iter_mut() {
        timer.timer -= time.delta_secs();
        if timer.timer <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
