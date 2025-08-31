use bevy::prelude::*;
use super::components::*;

/// Spawn a projectile at given position with given direction
pub fn spawn_projectile(
    commands: &mut Commands,
    position: Vec3,
    direction: Vec2,
) {
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 1.0, 0.0), // Yellow projectile
            custom_size: Some(Vec2::new(PROJECTILE_SIZE, PROJECTILE_SIZE)),
            ..default()
        },
        Transform::from_translation(position),
        Projectile {
            direction: direction.normalize(),
            speed: PROJECTILE_SPEED,
        },
    ));
}
