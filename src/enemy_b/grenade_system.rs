use super::components::*;
use super::config::ENEMY_B_CONFIG;
use crate::assets::GameAssets;
use crate::audio::{SfxEmitters, play_sfx_once};
use crate::collision::rectangles_collide;
use crate::components::{LayerGeometry, Solid};
use crate::constants::{DEFAULT_GRAVITY, PROJECTILE_SIZE, SCREEN_HEIGHT, Z_PROJECTILES};
use crate::effects::explosion_anim::{Explosion, ExplosionKind};
use crate::player::components::{Player, PlayerInvincibility, PlayerProne, PlayerRespawning};
use crate::player::player_damage_system::PlayerDamagedEvent;
use crate::player::setup_player::SpriteSize;
use bevy::prelude::*;

pub fn enemy_b_grenade_movement_system(
    mut commands: Commands,
    assets: Res<GameAssets>,
    player_q: Query<&Transform, With<crate::player::components::Player>>,
    mut enemies: Query<(&Transform, &EnemyBState, &mut EnemyBThrowAnim), With<EnemyB>>,
) {
    let Ok(player_tf) = player_q.single() else {
        return;
    };
    for (tf, st, mut anim) in enemies.iter_mut() {
        if st.state == EnemyBStateKind::Throwing && anim.frame == 2 && !anim.thrown {
            let offset = ENEMY_B_CONFIG.grenade_spawn_offset;
            let start = tf.translation
                + Vec3::new(
                    if st.facing_right { offset.x } else { -offset.x },
                    offset.y,
                    0.0,
                );
            let target = player_tf.translation;
            let dx = target.x - start.x;
            let t = ENEMY_B_CONFIG.grenade_time_of_flight;
            let vx = dx / t;
            let dy = target.y - start.y;
            let vy = (dy - 0.5 * DEFAULT_GRAVITY * t * t) / t;
            let mut e = commands.spawn((
                Sprite {
                    image: assets.enemy_b_grenade.clone(),
                    custom_size: Some(Vec2::splat(PROJECTILE_SIZE)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(start.x, start.y, Z_PROJECTILES))
                    .with_rotation(Quat::from_rotation_z(0.0)),
                Grenade {
                    velocity: Vec2::new(vx, vy),
                    rotation_timer: 0.0,
                },
            ));
            e.insert(Name::new("EnemyB Grenade"));
            anim.thrown = true;
        }
    }
}

pub fn enemy_b_grenade_physics_system(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &mut Grenade)>,
) {
    let rot_interval = 1.0 / ENEMY_B_CONFIG.grenade_rotation_fps;
    for (mut tr, mut g) in q.iter_mut() {
        g.velocity.y += DEFAULT_GRAVITY * time.delta_secs();
        tr.translation.x += g.velocity.x * time.delta_secs();
        tr.translation.y += g.velocity.y * time.delta_secs();
        g.rotation_timer += time.delta_secs();
        while g.rotation_timer >= rot_interval {
            g.rotation_timer -= rot_interval;
            let current = tr.rotation.to_euler(EulerRot::XYZ).2;
            tr.rotation = Quat::from_rotation_z(current + ENEMY_B_CONFIG.grenade_rotation_step);
        }
    }
}

pub fn enemy_b_grenade_collision_system(
    mut commands: Commands,
    grenades: Query<(Entity, &Transform), With<Grenade>>,
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
    for (e, tr) in grenades.iter() {
        let w = PROJECTILE_SIZE;
        let h = PROJECTILE_SIZE;
        let pos_world = Vec2::new(
            tr.translation.x - w / 2.0,
            tr.translation.y + SCREEN_HEIGHT / 2.0 - h / 2.0,
        );
        let size = Vec2::new(w, h);
        if rectangles_collide(pos_world, size, player_min, player_size) {
            spawn_explosion(&mut commands, &assets, emitters.as_ref(), tr.translation);
            if can_damage {
                let knockback_dir = if tr.translation.x <= player_tf.translation.x {
                    1.0
                } else {
                    -1.0
                };
                damage_writer.write(PlayerDamagedEvent { knockback_dir });
            }
            commands.entity(e).despawn();
            continue;
        }
        for geom in solids.iter() {
            if rectangles_collide(
                pos_world,
                size,
                geom.bottom_left,
                Vec2::new(geom.width, geom.height),
            ) {
                spawn_explosion(&mut commands, &assets, emitters.as_ref(), tr.translation);
                commands.entity(e).despawn();
                break;
            }
        }
    }
}

fn spawn_explosion(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    emitters: &SfxEmitters,
    pos: Vec3,
) {
    let p = Vec3::new(pos.x, pos.y, pos.z + 0.05);
    commands.spawn((
        Sprite::from_image(assets.explosion_a_a.clone()),
        Transform::from_translation(p),
        Explosion {
            kind: ExplosionKind::A,
            timer: 0.0,
            frame: 0,
        },
    ));
    play_sfx_once(
        commands,
        emitters.enemy_explosion,
        assets.enemy_explosion_sfx.clone(),
    );
}
