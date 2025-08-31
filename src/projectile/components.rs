use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub direction: Vec2,
    pub speed: f32,
}

pub const PROJECTILE_SIZE: f32 = 10.0;
pub const PROJECTILE_SPEED: f32 = 1680.0;
