use bevy::prelude::*;
use crate::components::WORLD_WIDTH;
use super::components::*;

/// Apply movement based on velocity
pub fn player_movement_system(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Velocity), With<Player>>,
) {
    for (mut transform, velocity) in player_query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();

        // Keep player within world bounds (horizontal)
        let min_x = PLAYER_WIDTH / 2.0;
        let max_x = WORLD_WIDTH - PLAYER_WIDTH / 2.0;
        transform.translation.x = transform.translation.x.clamp(min_x, max_x);
    }
}
