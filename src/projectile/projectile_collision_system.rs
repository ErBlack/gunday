use bevy::prelude::*;
use crate::components::*;
use crate::collision::rectangles_collide;
use super::components::*;

pub fn projectile_collision_system(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    geometry_query: Query<&LayerGeometry, With<Solid>>,
) {
    for (projectile_entity, projectile_transform) in projectile_query.iter() {
        let projectile_world_y = projectile_transform.translation.y + (SCREEN_HEIGHT / 2.0);
        let projectile_world_pos = Vec2::new(
            projectile_transform.translation.x - (PROJECTILE_SIZE / 2.0),
            projectile_world_y - (PROJECTILE_SIZE / 2.0)
        );
        let projectile_size = Vec2::new(PROJECTILE_SIZE, PROJECTILE_SIZE);
        
        for geometry in geometry_query.iter() {
            if rectangles_collide(
                projectile_world_pos, projectile_size,
                geometry.bottom_left, Vec2::new(geometry.width, geometry.height)
            ) {
                commands.entity(projectile_entity).despawn();
                break;
            }
        }
    }
}
