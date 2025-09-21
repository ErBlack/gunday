use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub direction: Vec2,
    pub speed: f32,
    pub previous_translation: Vec2,
}

#[derive(Component)]
pub struct PlayerProjectile;
#[derive(Component)]
pub struct EnemyProjectile;

#[derive(Component)]
pub struct MuzzleFlash;

#[derive(Component)]
pub struct OneShotLifetime {
    pub timer: f32,
}

#[derive(Event, Clone, Copy)]
pub struct ProjectileHitEvent {
    pub position: Vec3,
}

pub fn swept_projectile_hit_center(
    start_center: Vec2,
    end_center: Vec2,
    projectile_size: Vec2,
    target_min: Vec2,
    target_size: Vec2,
) -> Option<Vec2> {
    let half = projectile_size * 0.5;
    let movement = end_center - start_center;
    let total_distance = movement.length();
    let step_distance = half.x.min(half.y).max(1.0);
    let steps = (total_distance / step_distance).ceil().max(1.0) as u32;

    for step in 0..=steps {
        let t = if steps == 0 {
            0.0
        } else {
            step as f32 / steps as f32
        };
        let center = start_center + movement * t;
        let min = center - half;
        let overlap = min.x < target_min.x + target_size.x
            && min.x + projectile_size.x > target_min.x
            && min.y < target_min.y + target_size.y
            && min.y + projectile_size.y > target_min.y;
        if overlap {
            return Some(center);
        }
    }

    None
}
