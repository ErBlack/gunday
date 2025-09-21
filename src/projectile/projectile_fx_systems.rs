use super::components::*;
use crate::assets::GameAssets;
use crate::audio::{SfxEmitters, play_sfx_once};
use bevy::prelude::*;

const MUZZLE_FLASH_SIZE: Vec2 = Vec2::new(16.0, 16.0);
const HIT_ANIM_Z_OFFSET: f32 = 0.02;
const HIT_ANIM_FRAME_TIME: f32 = 0.01;

pub fn projectile_shoot_fx_projectile_system(
    mut commands: Commands,
    assets: Res<GameAssets>,
    emitters: Res<SfxEmitters>,
    mut q: Query<&mut Sprite, (With<PlayerProjectile>, Added<Projectile>)>,
) {
    for mut sprite in q.iter_mut() {
        sprite.image = assets.player_projectile.clone();
        sprite.custom_size = Some(Vec2::splat(crate::constants::PROJECTILE_SIZE));
        play_sfx_once(
            &mut commands,
            emitters.player_shoot,
            assets.shoot_sfx.clone(),
        );
    }
}

pub fn projectile_shoot_fx_flash_system(
    assets: Res<GameAssets>,
    mut q: Query<&mut Sprite, (With<MuzzleFlash>, Added<MuzzleFlash>)>,
) {
    for mut sprite in q.iter_mut() {
        if sprite.image != assets.player_shoot_flash {
            sprite.image = assets.player_shoot_flash.clone();
        }
        sprite.custom_size = Some(MUZZLE_FLASH_SIZE);
    }
}

pub fn projectile_hit_fx_system(
    mut commands: Commands,
    mut reader: EventReader<ProjectileHitEvent>,
    assets: Res<GameAssets>,
) {
    for ev in reader.read() {
        let pos = Vec3::new(
            ev.position.x,
            ev.position.y,
            ev.position.z + HIT_ANIM_Z_OFFSET,
        );
        commands.spawn((
            Sprite::from_image(assets.player_projectile_hit_a.clone()),
            Transform::from_translation(pos),
            ProjectileHitAnim {
                timer: 0.0,
                frame: 0,
            },
        ));
    }
}

#[derive(Component)]
pub struct ProjectileHitAnim {
    pub timer: f32,
    pub frame: u8,
}

pub fn projectile_hit_anim_update_system(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<GameAssets>,
    mut q: Query<(Entity, &mut Sprite, &mut ProjectileHitAnim)>,
) {
    for (e, mut sprite, mut anim) in q.iter_mut() {
        anim.timer += time.delta_secs();
        if anim.timer >= HIT_ANIM_FRAME_TIME {
            anim.timer = 0.0;
            anim.frame += 1;
            match anim.frame {
                1 => {
                    sprite.image = assets.player_projectile_hit_b.clone();
                }
                2 => {
                    sprite.image = assets.player_projectile_hit_c.clone();
                }
                _ => {
                    commands.entity(e).despawn();
                }
            }
        }
    }
}
