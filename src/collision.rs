use bevy::prelude::*;
use crate::components::*;

/// Check if two rectangles are colliding (AABB collision detection)
pub fn rectangles_collide(
    pos1: Vec2, size1: Vec2,
    pos2: Vec2, size2: Vec2,
) -> bool {
    pos1.x < pos2.x + size2.x &&
    pos1.x + size1.x > pos2.x &&
    pos1.y < pos2.y + size2.y &&
    pos1.y + size1.y > pos2.y
}

/// Collision system for projectiles with layer geometry
pub fn projectile_collision_system(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    geometry_query: Query<&LayerGeometry, With<Solid>>,
) {
    for (projectile_entity, projectile_transform) in projectile_query.iter() {
        // Convert projectile position to world coordinates
        let projectile_world_y = projectile_transform.translation.y + (SCREEN_HEIGHT / 2.0);
        let projectile_world_pos = Vec2::new(
            projectile_transform.translation.x - 4.0, // Assuming 8x8 projectile
            projectile_world_y - 4.0
        );
        let projectile_size = Vec2::new(8.0, 8.0);
        
        // Check collision with layer geometry
        for geometry in geometry_query.iter() {
            if rectangles_collide(
                projectile_world_pos, projectile_size,
                geometry.bottom_left, Vec2::new(geometry.width, geometry.height)
            ) {
                // Projectile hit geometry, remove it
                commands.entity(projectile_entity).despawn();
                break;
            }
        }
    }
}
