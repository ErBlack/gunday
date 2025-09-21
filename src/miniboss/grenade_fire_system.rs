use super::components::*;
use super::config::MINIBOSS_CONFIG;
use crate::assets::GameAssets;
use crate::audio::{SfxEmitters, play_sfx_once};
use crate::constants::{PROJECTILE_SIZE, Z_PROJECTILES};
use bevy::prelude::*;

pub fn miniboss_grenade_fire_system(
    mut commands: Commands,
    assets: Res<GameAssets>,
    emitters: Res<SfxEmitters>,
    mut events: EventReader<MinibossFireEvent>,
    miniboss_q: Query<(&Transform, &MinibossBehavior)>,
) {
    let time_of_flight = miniboss_time_of_flight();
    for event in events.read() {
        let Ok((transform, behavior)) = miniboss_q.get(event.entity) else {
            continue;
        };
        let points_per_volley = MINIBOSS_CONFIG.grenade_points.len() / 2;
        let start_index = match event.volley_kind {
            MinibossVolleyKind::First => 0,
            MinibossVolleyKind::Second => points_per_volley,
        };
        let point = MINIBOSS_CONFIG.grenade_points[start_index + event.shot_index as usize];
        let mut offset = miniboss_grenade_offset(point);
        if !behavior.facing_right {
            offset.x = -offset.x;
        }
        let spawn_pos = Vec3::new(
            transform.translation.x + offset.x,
            transform.translation.y + offset.y,
            Z_PROJECTILES,
        );

        let dir = if behavior.facing_right { 1.0 } else { -1.0 };
        let distance = fastrand::f32() * 400.0 + 200.0;
        let vx = (distance / time_of_flight) * dir;
        let vy = MINIBOSS_CONFIG.grenade_initial_velocity_y;

        let mut sprite = Sprite::from_image(assets.enemy_b_grenade.clone());
        sprite.custom_size = Some(Vec2::splat(PROJECTILE_SIZE));

        commands.spawn((
            sprite,
            Transform::from_translation(spawn_pos),
            MinibossGrenade {
                velocity: Vec2::new(vx, vy),
                rotation_timer: 0.0,
            },
            Name::new("Miniboss Grenade"),
        ));

        play_sfx_once(
            &mut commands,
            emitters.enemy_shoot,
            assets.enemy_shoot_sfx.clone(),
        );
    }
}
