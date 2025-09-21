use super::components::*;
use crate::assets::GameAssets;
use crate::audio::{SfxEmitters, play_sfx_once};
use crate::collision::rectangles_collide;
use crate::components::{LayerGeometry, Solid};
use crate::constants::{PROJECTILE_SIZE, SCREEN_HEIGHT};
use crate::effects::explosion_anim::spawn_explosion_a;
use crate::player::components::{Player, PlayerInvincibility, PlayerProne, PlayerRespawning};
use crate::player::player_damage_system::PlayerDamagedEvent;
use crate::player::setup_player::SpriteSize;
use bevy::prelude::*;

pub fn miniboss_grenade_collision_system(
    mut commands: Commands,
    grenades: Query<(Entity, &Transform), With<MinibossGrenade>>,
    solids: Query<&LayerGeometry, With<Solid>>,
    players: Query<
        (
            &Transform,
            &SpriteSize,
            Option<&PlayerInvincibility>,
            Option<&PlayerRespawning>,
            Option<&PlayerProne>,
        ),
        With<Player>,
    >,
    mut damage_writer: EventWriter<PlayerDamagedEvent>,
    assets: Res<GameAssets>,
    emitters: Res<SfxEmitters>,
) {
    let Ok((player_tf, sprite_size, invincible, respawning, prone)) = players.single() else {
        return;
    };
    let player_world_y = player_tf.translation.y + SCREEN_HEIGHT / 2.0;
    let player_min = Vec2::new(
        player_tf.translation.x - sprite_size.width / 2.0,
        player_world_y - sprite_size.height / 2.0,
    );
    let player_size = Vec2::new(sprite_size.width, sprite_size.height);
    let can_damage = invincible.is_none() && respawning.is_none() && prone.is_none();
    for (entity, transform) in grenades.iter() {
        let w = PROJECTILE_SIZE;
        let h = PROJECTILE_SIZE;
        let pos_world = Vec2::new(
            transform.translation.x - w / 2.0,
            transform.translation.y + SCREEN_HEIGHT / 2.0 - h / 2.0,
        );
        let size = Vec2::splat(PROJECTILE_SIZE);
        if rectangles_collide(pos_world, size, player_min, player_size) {
            spawn_explosion_fx(&mut commands, &assets, &emitters, transform.translation);
            if can_damage {
                let knockback_dir = if transform.translation.x <= player_tf.translation.x {
                    1.0
                } else {
                    -1.0
                };
                damage_writer.write(PlayerDamagedEvent { knockback_dir });
            }
            commands.entity(entity).despawn();
            continue;
        }
        for geom in solids.iter() {
            if rectangles_collide(
                pos_world,
                size,
                geom.bottom_left,
                Vec2::new(geom.width, geom.height),
            ) {
                spawn_explosion_fx(&mut commands, &assets, &emitters, transform.translation);
                commands.entity(entity).despawn();
                break;
            }
        }
    }
}

fn spawn_explosion_fx(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    emitters: &SfxEmitters,
    position: Vec3,
) {
    spawn_explosion_a(
        commands,
        assets,
        Vec3::new(position.x, position.y, position.z + 0.1),
    );
    play_sfx_once(
        commands,
        emitters.enemy_explosion,
        assets.enemy_explosion_sfx.clone(),
    );
}
