use bevy::prelude::*;
use crate::components::*;
use crate::constants::SCREEN_HEIGHT;
use crate::collision::rectangles_collide;
use super::components::*;
use crate::constants::PROJECTILE_SIZE;

pub fn projectile_collision_system(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform, Option<&Sprite>), With<Projectile>>,
    geometry_query: Query<&LayerGeometry, With<Solid>>,
    mut hit_writer: EventWriter<ProjectileHitEvent>,
) {
    for (projectile_entity, projectile_transform, sprite_opt) in projectile_query.iter() {
        let projectile_world_y = projectile_transform.translation.y + (SCREEN_HEIGHT / 2.0);
        let (w, h) = sprite_opt
            .and_then(|s| s.custom_size)
            .map(|v| (v.x, v.y))
            .unwrap_or((PROJECTILE_SIZE, PROJECTILE_SIZE));
        let projectile_world_pos = Vec2::new(
            projectile_transform.translation.x - (w / 2.0),
            projectile_world_y - (h / 2.0)
        );
        let projectile_size = Vec2::new(w, h);

        let px_min = projectile_world_pos.x;
        let px_max = projectile_world_pos.x + projectile_size.x;
        let py_min = projectile_world_pos.y;
        let py_max = projectile_world_pos.y + projectile_size.y;
        
        for geometry in geometry_query.iter() {
            let gx_min = geometry.bottom_left.x;
            let gx_max = geometry.bottom_left.x + geometry.width;
            let gy_min = geometry.bottom_left.y;
            let gy_max = geometry.bottom_left.y + geometry.height;
            if px_max < gx_min || px_min > gx_max || py_max < gy_min || py_min > gy_max { continue; }

            if rectangles_collide(
                projectile_world_pos, projectile_size,
                geometry.bottom_left, Vec2::new(geometry.width, geometry.height)
            ) {
                hit_writer.write(ProjectileHitEvent { position: projectile_transform.translation });
                commands.entity(projectile_entity).despawn();
                break;
            }
        }
    }
}
