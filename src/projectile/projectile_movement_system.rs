use bevy::prelude::*;
use crate::components::{SCREEN_HEIGHT, WORLD_WIDTH};
use super::components::*;

/// Move projectiles and handle cleanup
pub fn projectile_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_query: Query<(Entity, &mut Transform, &Projectile)>,
) {
    for (entity, mut transform, projectile) in projectile_query.iter_mut() {
        // Move projectile
        let movement = projectile.direction * projectile.speed * time.delta_secs();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
        
        // Remove projectiles that are off-screen or out of world bounds
        if transform.translation.x < -100.0 
            || transform.translation.x > WORLD_WIDTH + 100.0
            || transform.translation.y < -300.0
            || transform.translation.y > SCREEN_HEIGHT + 300.0{
            commands.entity(entity).despawn();
        }
    }
}
