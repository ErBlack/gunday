use super::components::*;
use crate::collision::rectangles_collide;
use crate::components::{LayerGeometry, Solid};
use crate::constants::{
    DESPAWN_MARGIN_X, DESPAWN_MARGIN_Y, PROJECTILE_SIZE, SCREEN_HEIGHT, WORLD_WIDTH,
};
use bevy::prelude::*;

pub fn projectile_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_query: Query<(Entity, &mut Transform, &mut Projectile, Option<&Sprite>)>,
    geometry_query: Query<&LayerGeometry, With<Solid>>,
    mut hit_writer: EventWriter<ProjectileHitEvent>,
) {
    let delta = time.delta_secs();
    if delta <= 0.0 {
        return;
    }

    for (entity, mut transform, mut projectile, sprite_opt) in projectile_query.iter_mut() {
        let start_center = Vec2::new(transform.translation.x, transform.translation.y);
        let movement = projectile.direction * projectile.speed * delta;

        projectile.previous_translation = start_center;

        let (width, height) = sprite_opt
            .and_then(|sprite| sprite.custom_size)
            .map(|size| (size.x, size.y))
            .unwrap_or((PROJECTILE_SIZE, PROJECTILE_SIZE));

        let step_distance = (width.min(height) * 0.5).max(1.0);
        let total_distance = movement.length();
        let steps = (total_distance / step_distance).ceil().max(1.0) as u32;

        let mut hit_position: Option<Vec2> = None;

        'sweep: for step in 1..=steps {
            let t = step as f32 / steps as f32;
            let sample_center = start_center + movement * t;
            let sample_world_center =
                Vec2::new(sample_center.x, sample_center.y + SCREEN_HEIGHT / 2.0);
            let sample_min = Vec2::new(
                sample_world_center.x - width * 0.5,
                sample_world_center.y - height * 0.5,
            );
            let sample_max_x = sample_min.x + width;
            let sample_max_y = sample_min.y + height;

            for geometry in geometry_query.iter() {
                let geo_min_x = geometry.bottom_left.x;
                let geo_max_x = geo_min_x + geometry.width;
                let geo_min_y = geometry.bottom_left.y;
                let geo_max_y = geo_min_y + geometry.height;
                if sample_max_x < geo_min_x
                    || sample_min.x > geo_max_x
                    || sample_max_y < geo_min_y
                    || sample_min.y > geo_max_y
                {
                    continue;
                }

                if rectangles_collide(
                    sample_min,
                    Vec2::new(width, height),
                    geometry.bottom_left,
                    Vec2::new(geometry.width, geometry.height),
                ) {
                    hit_position = Some(sample_center);
                    break 'sweep;
                }
            }
        }

        if let Some(hit_center) = hit_position {
            transform.translation.x = hit_center.x;
            transform.translation.y = hit_center.y;
            hit_writer.write(ProjectileHitEvent {
                position: Vec3::new(hit_center.x, hit_center.y, transform.translation.z),
            });
            commands.entity(entity).despawn();
            continue;
        }

        let end_center = start_center + movement;
        transform.translation.x = end_center.x;
        transform.translation.y = end_center.y;

        if transform.translation.x < -DESPAWN_MARGIN_X
            || transform.translation.x > WORLD_WIDTH + DESPAWN_MARGIN_X
            || transform.translation.y < -DESPAWN_MARGIN_Y
            || transform.translation.y > SCREEN_HEIGHT + DESPAWN_MARGIN_Y
        {
            commands.entity(entity).despawn();
        }
    }
}
