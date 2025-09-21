use super::components::*;
use crate::assets::GameAssets;
use crate::constants::{ENEMY_PROJECTILE_SPEED, PROJECTILE_SIZE, PROJECTILE_SPEED, Z_PROJECTILES};
use bevy::prelude::*;

pub fn spawn_projectile(commands: &mut Commands, position: Vec3, direction: Vec2) {
    let mut pos = position;
    pos.z = pos.z.max(Z_PROJECTILES);
    let initial_translation = Vec2::new(pos.x, pos.y);
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(PROJECTILE_SIZE, PROJECTILE_SIZE)),
            ..default()
        },
        Transform::from_translation(pos),
        Projectile {
            direction: direction.normalize(),
            speed: PROJECTILE_SPEED,
            previous_translation: initial_translation,
        },
        PlayerProjectile,
    ));
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(16.0, 16.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(position.x, position.y, pos.z + 0.01)),
        MuzzleFlash,
        OneShotLifetime { timer: 0.01 },
    ));
}

pub fn spawn_enemy_projectile(
    commands: &mut Commands,
    assets: &GameAssets,
    mut position: Vec3,
    direction: Vec2,
) {
    position.z = position.z.max(Z_PROJECTILES);
    let initial_translation = Vec2::new(position.x, position.y);
    commands.spawn((
        Sprite {
            image: assets.enemy_a_projectile.clone(),
            color: Color::WHITE,
            custom_size: Some(Vec2::splat(PROJECTILE_SIZE)),
            ..default()
        },
        Transform::from_translation(position),
        Projectile {
            direction: direction.normalize(),
            speed: ENEMY_PROJECTILE_SPEED,
            previous_translation: initial_translation,
        },
        EnemyProjectile,
    ));
}

pub fn spawn_boss_projectile(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    mut position: Vec3,
    direction: Vec2,
) {
    position.z = position.z.max(Z_PROJECTILES);
    let dir = direction.normalize_or_zero();
    let angle = dir.to_angle();
    let transform =
        Transform::from_translation(position).with_rotation(Quat::from_rotation_z(angle));
    commands.spawn((
        Sprite {
            image: assets.boss_projectile.clone(),
            custom_size: Some(Vec2::new(16.0, 8.0)),
            ..default()
        },
        transform,
        Projectile {
            direction: dir,
            speed: PROJECTILE_SPEED * 0.85,
            previous_translation: Vec2::new(position.x, position.y),
        },
        EnemyProjectile,
    ));
}

pub fn one_shot_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut q: Query<(Entity, &mut OneShotLifetime)>,
) {
    for (e, mut life) in q.iter_mut() {
        life.timer -= time.delta_secs();
        if life.timer <= 0.0 {
            commands.entity(e).despawn();
        }
    }
}
