use super::components::{
    Grounded, JumpState, Player, PlayerActions, PlayerBlink, PlayerDirection, PlayerGameOver,
    PlayerInvincibility, PlayerLives, PlayerProne, PlayerRespawning, PlayerSpawnPoint,
    PlayerSprite, PlayerSpriteEntity, PlayerSpriteKind, Velocity,
};
use crate::assets::GameAssets;
use crate::audio::{SfxEmitters, play_sfx_once};
use crate::boss::components::{Boss, BossCollider, BossStage, BossStageKind};
use crate::components::MainCamera;
use crate::systems::PlayerControl;
use crate::constants::{PROJECTILE_SIZE, SCREEN_WIDTH};
use crate::enemy_a::robot_components::{
    ENEMY_ROBOT_HEIGHT, ENEMY_ROBOT_WIDTH, EnemyRobot, EnemyRobotState, EnemyRobotStateKind,
    EnemySpawnProtection,
};
use crate::enemy_b::components::{
    ENEMY_B_HEIGHT, ENEMY_B_WIDTH, EnemyB, EnemyBSpawnProtection, EnemyBState, EnemyBStateKind,
};
use crate::enemy_c::components::{
    ENEMY_C_HEIGHT, ENEMY_C_WIDTH, EnemyC, EnemyCState, EnemyCStateKind,
};
use crate::miniboss::MINIBOSS_CONFIG;
use crate::miniboss::components::{Miniboss, MinibossBehavior, MinibossPhase};
use crate::player::PLAYER_CONFIG;
use crate::player::setup_player::SpriteSize;
use crate::projectile::components::{
    EnemyProjectile, Projectile, ProjectileHitEvent, swept_projectile_hit_center,
};
use bevy::prelude::*;

#[derive(Event, Debug, Clone, Copy)]
pub struct PlayerDamagedEvent {
    pub knockback_dir: f32,
}

pub fn player_enemy_projectile_hit_system(
    mut commands: Commands,
    player_query: Query<
        (
            &Transform,
            &SpriteSize,
            Option<&PlayerInvincibility>,
            Option<&PlayerRespawning>,
            Option<&PlayerProne>,
        ),
        With<Player>,
    >,
    projectile_query: Query<
        (Entity, &Transform, &Projectile, Option<&Sprite>),
        (With<Projectile>, With<EnemyProjectile>),
    >,
    mut damage_writer: EventWriter<PlayerDamagedEvent>,
    mut hit_writer: EventWriter<ProjectileHitEvent>,
) {
    if PLAYER_CONFIG.permanent_invincibility {
        return;
    }

    let Some((player_transform, sprite_size, invincible, respawning, prone)) =
        player_query.iter().next()
    else {
        return;
    };

    if invincible.is_some() || respawning.is_some() || prone.is_some() {
        return;
    }

    let player_min = Vec2::new(
        player_transform.translation.x - sprite_size.width / 2.0,
        player_transform.translation.y - sprite_size.height / 2.0,
    );
    let player_size = Vec2::new(sprite_size.width, sprite_size.height);

    for (projectile_entity, projectile_transform, projectile, sprite_opt) in projectile_query.iter()
    {
        let projectile_size = sprite_opt
            .and_then(|sprite| sprite.custom_size)
            .map(|size| Vec2::new(size.x, size.y))
            .unwrap_or(Vec2::splat(PROJECTILE_SIZE));

        if let Some(hit_center) = swept_projectile_hit_center(
            projectile.previous_translation,
            Vec2::new(
                projectile_transform.translation.x,
                projectile_transform.translation.y,
            ),
            projectile_size,
            player_min,
            player_size,
        ) {
            let knockback_dir = if hit_center.x <= player_transform.translation.x {
                1.0
            } else {
                -1.0
            };
            damage_writer.write(PlayerDamagedEvent { knockback_dir });
            hit_writer.write(ProjectileHitEvent {
                position: Vec3::new(
                    hit_center.x,
                    hit_center.y,
                    projectile_transform.translation.z,
                ),
            });
            commands.entity(projectile_entity).despawn();
            break;
        }
    }
}

pub fn player_enemy_contact_damage_system(
    mut damage_writer: EventWriter<PlayerDamagedEvent>,
    player_query: Query<
        (
            &Transform,
            &SpriteSize,
            Option<&PlayerInvincibility>,
            Option<&PlayerRespawning>,
            Option<&PlayerProne>,
        ),
        With<Player>,
    >,
    enemy_robot_query: Query<
        (&Transform, &EnemyRobotState, Option<&EnemySpawnProtection>),
        With<EnemyRobot>,
    >,
    enemy_b_query: Query<(&Transform, &EnemyBState, Option<&EnemyBSpawnProtection>), With<EnemyB>>,
    enemy_c_query: Query<(&Transform, &EnemyCState), With<EnemyC>>,
    miniboss_query: Query<(&Transform, &MinibossBehavior), With<Miniboss>>,
    boss_query: Query<(&Transform, &BossCollider, Option<&BossStage>), With<Boss>>,
) {
    if PLAYER_CONFIG.permanent_invincibility {
        return;
    }

    let Ok((player_transform, sprite_size, invincible, respawning, prone)) = player_query.single()
    else {
        return;
    };

    if invincible.is_some() || respawning.is_some() || prone.is_some() {
        return;
    }

    let player_center = player_transform.translation.truncate();
    let player_half = Vec2::new(sprite_size.width * 0.5, sprite_size.height * 0.5);
    let mut damaged = false;

    if !damaged {
        for (transform, state, protection) in enemy_robot_query.iter() {
            if matches!(state.state, EnemyRobotStateKind::Hit) {
                continue;
            }
            if protection.is_some() {
                continue;
            }
            let enemy_center = transform.translation.truncate();
            let enemy_half = Vec2::new(ENEMY_ROBOT_WIDTH * 0.5, ENEMY_ROBOT_HEIGHT * 0.5);
            if aabb_overlap(player_center, player_half, enemy_center, enemy_half) {
                let knockback_dir = if enemy_center.x <= player_center.x {
                    1.0
                } else {
                    -1.0
                };
                damage_writer.write(PlayerDamagedEvent { knockback_dir });
                damaged = true;
                break;
            }
        }
    }

    if !damaged {
        for (transform, state, protection) in enemy_b_query.iter() {
            if matches!(state.state, EnemyBStateKind::Hit) {
                continue;
            }
            if protection.is_some() {
                continue;
            }
            let enemy_center = transform.translation.truncate();
            let enemy_half = Vec2::new(ENEMY_B_WIDTH * 0.5, ENEMY_B_HEIGHT * 0.5);
            if aabb_overlap(player_center, player_half, enemy_center, enemy_half) {
                let knockback_dir = if enemy_center.x <= player_center.x {
                    1.0
                } else {
                    -1.0
                };
                damage_writer.write(PlayerDamagedEvent { knockback_dir });
                damaged = true;
                break;
            }
        }
    }

    if !damaged {
        for (transform, state) in enemy_c_query.iter() {
            if matches!(state.state, EnemyCStateKind::Dying) {
                continue;
            }
            let enemy_center = Vec2::new(
                transform.translation.x,
                transform.translation.y + ENEMY_C_HEIGHT * 0.5,
            );
            let enemy_half = Vec2::new(ENEMY_C_WIDTH * 0.5, ENEMY_C_HEIGHT * 0.5);
            if aabb_overlap(player_center, player_half, enemy_center, enemy_half) {
                let knockback_dir = if enemy_center.x <= player_center.x {
                    1.0
                } else {
                    -1.0
                };
                damage_writer.write(PlayerDamagedEvent { knockback_dir });
                damaged = true;
                break;
            }
        }
    }

    if !damaged {
        for (transform, behavior) in miniboss_query.iter() {
            if matches!(behavior.phase, MinibossPhase::Dying | MinibossPhase::Dead) {
                continue;
            }
            let enemy_center = Vec2::new(
                transform.translation.x,
                transform.translation.y + MINIBOSS_CONFIG.height * 0.5,
            );
            let enemy_half = Vec2::new(MINIBOSS_CONFIG.width * 0.5, MINIBOSS_CONFIG.height * 0.5);
            if aabb_overlap(player_center, player_half, enemy_center, enemy_half) {
                let knockback_dir = if enemy_center.x <= player_center.x {
                    1.0
                } else {
                    -1.0
                };
                damage_writer.write(PlayerDamagedEvent { knockback_dir });
                damaged = true;
                break;
            }
        }
    }

    if !damaged {
        for (transform, collider, stage_opt) in boss_query.iter() {
            if let Some(BossStage(stage_kind)) = stage_opt {
                if matches!(stage_kind, BossStageKind::Exploding | BossStageKind::TransitionToStage2) {
                    continue;
                }
            }
            let enemy_center = transform.translation.truncate();
            let enemy_half = collider.half_size;
            if aabb_overlap(player_center, player_half, enemy_center, enemy_half) {
                let knockback_dir = if enemy_center.x <= player_center.x {
                    1.0
                } else {
                    -1.0
                };
                damage_writer.write(PlayerDamagedEvent { knockback_dir });
                break;
            }
        }
    }
}

pub fn player_damage_system(
    mut commands: Commands,
    mut events: EventReader<PlayerDamagedEvent>,
    mut queries: ParamSet<(
        Query<
            (
                Entity,
                &mut PlayerLives,
                &mut Velocity,
                &mut Grounded,
                &mut JumpState,
                &mut PlayerDirection,
                &PlayerSpriteEntity,
                Option<&PlayerProne>,
            ),
            (With<Player>, Without<PlayerSprite>),
        >,
        Query<
            (&mut Sprite, &mut PlayerSpriteKind, &mut Transform),
            (With<PlayerSprite>, Without<Player>),
        >,
    )>,
    game_assets: Res<GameAssets>,
    emitters: Res<SfxEmitters>,
) {
    if events.is_empty() {
        return;
    }

    let mut sprite_updates: Vec<(Entity, SpriteUpdate)> = Vec::new();

    {
        let mut player_query = queries.p0();

        let Some((
            player_entity,
            mut lives,
            mut velocity,
            mut grounded,
            mut jump_state,
            mut direction,
            sprite_entity,
            prone,
        )) = player_query.iter_mut().next()
        else {
            events.clear();
            return;
        };

        let sprite_entity = **sprite_entity;

        for event in events.read() {
            if PLAYER_CONFIG.permanent_invincibility {
                continue;
            }
            if prone.is_some() {
                continue;
            }
            if lives.current == 0 {
                continue;
            }

            let was_last_life = lives.current == 1;

            lives.lose_life();

            if was_last_life {
                commands
                    .entity(player_entity)
                    .insert(PlayerGameOver::new(PLAYER_CONFIG.game_over_prone_duration));
            }

            commands
                .entity(player_entity)
                .insert(PlayerProne::new(PLAYER_CONFIG.knockdown_duration));
            commands
                .entity(player_entity)
                .remove::<PlayerInvincibility>();

            velocity.x = event.knockback_dir * PLAYER_CONFIG.knockback_horizontal_speed;
            velocity.y = PLAYER_CONFIG.knockback_vertical_speed;
            grounded.is_grounded = false;
            jump_state.is_jumping = false;
            jump_state.jump_timer = 0.0;
            jump_state.jump_buffer_timer = 0.0;

            if event.knockback_dir > 0.0 {
                direction.facing_right = false;
                direction.last_movement_direction = -1.0;
            } else if event.knockback_dir < 0.0 {
                direction.facing_right = true;
                direction.last_movement_direction = 1.0;
            }

            sprite_updates.push((sprite_entity, SpriteUpdate::Hit));
            commands.entity(player_entity).remove::<PlayerBlink>();

            play_sfx_once(
                &mut commands,
                emitters.player_hit,
                game_assets.player_hit_sfx.clone(),
            );

            break;
        }
    }

    if sprite_updates.is_empty() {
        return;
    }

    let mut sprite_query = queries.p1();
    for (entity, update) in sprite_updates {
        apply_sprite_update(&mut sprite_query, entity, update, &game_assets);
    }
}

pub fn player_prone_system(
    mut commands: Commands,
    time: Res<Time>,
    game_assets: Res<GameAssets>,
    mut actions: ResMut<PlayerActions>,
    camera_query: Query<&Transform, (With<MainCamera>, Without<Player>, Without<PlayerSprite>)>,
    mut queries: ParamSet<(
        Query<
            (
                Entity,
                &mut PlayerProne,
                &mut Velocity,
                &mut Grounded,
                &PlayerSpawnPoint,
                &PlayerSpriteEntity,
                &mut Transform,
                &mut Visibility,
                &mut PlayerDirection,
                Option<&PlayerGameOver>,
            ),
            (With<Player>, Without<PlayerSprite>),
        >,
        Query<
            (&mut Sprite, &mut PlayerSpriteKind, &mut Transform),
            (With<PlayerSprite>, Without<Player>),
        >,
    )>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    let mut sprite_updates: Vec<(Entity, SpriteUpdate)> = Vec::new();

    {
        let mut player_query = queries.p0();

        for (
            entity,
            mut prone,
            mut velocity,
            mut grounded,
            spawn_point,
            sprite_entity,
            mut transform,
            mut visibility,
            mut direction,
            game_over_opt,
        ) in player_query.iter_mut()
        {
            let sprite_entity = **sprite_entity;
            let is_game_over = game_over_opt.is_some();

            if !prone.landed {
                if grounded.is_grounded && velocity.y <= 0.0 {
                    prone.landed = true;
                    velocity.x = 0.0;
                    velocity.y = 0.0;
                    sprite_updates.push((sprite_entity, SpriteUpdate::Fallen));
                    if is_game_over {
                        actions.reset();
                    } else {
                        prone.timer = PLAYER_CONFIG.knockdown_duration;
                    }
                }
                continue;
            }

            if is_game_over {
                velocity.x = 0.0;
                velocity.y = 0.0;
                grounded.is_grounded = true;
                continue;
            }

            prone.timer -= dt;
            if prone.timer > 0.0 {
                continue;
            }

            let base_spawn = **spawn_point;
            let camera_x = camera_query
                .iter()
                .next()
                .map(|transform| transform.translation.x)
                .unwrap_or(base_spawn.x);
            let left_edge = camera_x - SCREEN_WIDTH / 2.0;
            let respawn_x = left_edge + PLAYER_CONFIG.spawn_screen_fraction * SCREEN_WIDTH;
            let new_translation = Vec3::new(respawn_x, base_spawn.y, base_spawn.z);

            transform.translation = new_translation;
            commands
                .entity(entity)
                .insert(PlayerSpawnPoint(new_translation));
            velocity.x = 0.0;
            velocity.y = 0.0;
            grounded.is_grounded = true;
            *visibility = Visibility::Visible;
            direction.facing_right = true;
            direction.last_movement_direction = 1.0;

            commands.entity(entity).remove::<PlayerProne>();
            commands.entity(entity).insert(PlayerInvincibility {
                timer: PLAYER_CONFIG.respawn_invincibility,
            });
            commands
                .entity(entity)
                .insert(PlayerBlink::new(PLAYER_CONFIG.invincibility_flash_interval));

            sprite_updates.push((sprite_entity, SpriteUpdate::Static));

            actions.reset();
        }
    }

    if sprite_updates.is_empty() {
        return;
    }

    let mut sprite_query = queries.p1();
    for (entity, update) in sprite_updates {
        apply_sprite_update(&mut sprite_query, entity, update, &game_assets);
    }
}

pub fn player_invincibility_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut PlayerInvincibility,
        Option<&PlayerSpriteEntity>,
    )>,
    mut sprite_query: Query<&mut Sprite>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    for (entity, mut invincibility, sprite_entity) in query.iter_mut() {
        invincibility.timer -= dt;
        if invincibility.timer <= 0.0 {
            commands.entity(entity).remove::<PlayerInvincibility>();
            commands.entity(entity).remove::<PlayerBlink>();
            if let Some(sprite_entity) = sprite_entity {
                if let Ok(mut sprite) = sprite_query.get_mut(**sprite_entity) {
                    sprite.color = sprite.color.with_alpha(1.0);
                }
            }
        }
    }
}

pub fn player_respawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut queries: ParamSet<(
        Query<
            (
                Entity,
                &mut PlayerRespawning,
                &PlayerSpawnPoint,
                &mut PlayerLives,
                &PlayerSpriteEntity,
                &mut Transform,
                &mut Visibility,
                &mut Velocity,
                &mut Grounded,
            ),
            (With<Player>, Without<PlayerSprite>),
        >,
        Query<
            (&mut Sprite, &mut PlayerSpriteKind, &mut Transform),
            (With<PlayerSprite>, Without<Player>),
        >,
    )>,
    game_assets: Res<GameAssets>,
    mut actions: ResMut<PlayerActions>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    let mut sprite_updates: Vec<(Entity, SpriteUpdate)> = Vec::new();

    {
        let mut player_query = queries.p0();

        for (
            entity,
            mut respawning,
            spawn_point,
            mut lives,
            sprite_entity,
            mut transform,
            mut visibility,
            mut velocity,
            mut grounded,
        ) in player_query.iter_mut()
        {
            let sprite_entity = **sprite_entity;

            respawning.timer -= dt;
            if respawning.timer <= 0.0 {
                lives.restore_full(PLAYER_CONFIG.starting_lives);
                transform.translation = **spawn_point;
                velocity.x = 0.0;
                velocity.y = 0.0;
                grounded.is_grounded = true;
                *visibility = Visibility::Visible;
                commands.entity(entity).remove::<PlayerRespawning>();
                commands.entity(entity).remove::<PlayerProne>();
                commands.entity(entity).insert(PlayerInvincibility {
                    timer: PLAYER_CONFIG.respawn_invincibility,
                });
                commands
                    .entity(entity)
                    .insert(PlayerBlink::new(PLAYER_CONFIG.invincibility_flash_interval));
                sprite_updates.push((sprite_entity, SpriteUpdate::Static));
                actions.reset();
            } else if matches!(*visibility, Visibility::Visible) {
                *visibility = Visibility::Hidden;
            }
        }
    }

    if sprite_updates.is_empty() {
        return;
    }

    let mut sprite_query = queries.p1();
    for (entity, update) in sprite_updates {
        apply_sprite_update(&mut sprite_query, entity, update, &game_assets);
    }
}

pub fn player_invincibility_blink_system(
    time: Res<Time>,
    mut player_query: Query<
        (
            &mut PlayerBlink,
            Option<&PlayerInvincibility>,
            &PlayerSpriteEntity,
        ),
        With<Player>,
    >,
    mut sprite_query: Query<&mut Sprite>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    for (mut blink, invincibility, sprite_entity) in player_query.iter_mut() {
        if invincibility.is_none() {
            if let Ok(mut sprite) = sprite_query.get_mut(**sprite_entity) {
                if !blink.visible {
                    sprite.color = sprite.color.with_alpha(1.0);
                }
            }
            blink.reset();
            continue;
        }

        blink.timer -= dt;
        if blink.timer <= 0.0 {
            blink.timer += blink.interval;
            blink.visible = !blink.visible;
            if let Ok(mut sprite) = sprite_query.get_mut(**sprite_entity) {
                let alpha = if blink.visible { 1.0 } else { 0.0 };
                sprite.color = sprite.color.with_alpha(alpha);
            }
        }
    }
}

pub fn player_game_over_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut PlayerGameOver, Option<&PlayerProne>)>,
    game_assets: Res<GameAssets>,
    emitters: Option<Res<SfxEmitters>>,
    mut control: Option<ResMut<PlayerControl>>,
) {
    let dt = time.delta_secs();
    if dt <= 0.0 {
        return;
    }

    for (_entity, mut game_over, prone) in query.iter_mut() {
        if let Some(prone) = prone {
            if !prone.landed {
                continue;
            }
        }

        if !game_over.started {
            game_over.started = true;
            if let Some(control) = control.as_deref_mut() {
                control.enabled = false;
            }
        }

        if !game_over.sfx_played {
            game_over.sfx_delay -= dt;
            if game_over.sfx_delay <= 0.0 {
                if let Some(emitters) = emitters.as_deref() {
                    play_sfx_once(
                        &mut commands,
                        emitters.player_game_over,
                        game_assets.player_game_over_sfx.clone(),
                    );
                }
                game_over.sfx_played = true;
            }
        }

        game_over.timer -= dt;
        if game_over.timer > 0.0 {
            continue;
        }

        if game_over.result_sent {
            continue;
        }

        game_over.result_sent = true;

        #[cfg(target_arch = "wasm32")]
        {
            crate::systems::browser_events::send_game_result(false);
        }

        commands.trigger(bevy::app::AppExit::Success);
        break;
    }
}

enum SpriteUpdate {
    Hit,
    Fallen,
    Static,
}

fn apply_sprite_update(
    sprite_query: &mut Query<
        (&mut Sprite, &mut PlayerSpriteKind, &mut Transform),
        (With<PlayerSprite>, Without<Player>),
    >,
    entity: Entity,
    update: SpriteUpdate,
    game_assets: &GameAssets,
) {
    if let Ok((mut sprite, mut kind, mut sprite_transform)) = sprite_query.get_mut(entity) {
        match update {
            SpriteUpdate::Hit => {
                sprite.image = game_assets.player_hit.clone();
                sprite.color = sprite.color.with_alpha(1.0);
                *kind = PlayerSpriteKind::Hit;
                sprite_transform.rotation = Quat::IDENTITY;
            }
            SpriteUpdate::Fallen => {
                sprite.image = game_assets.player_fall.clone();
                *kind = PlayerSpriteKind::Fallen;
                sprite_transform.rotation = Quat::IDENTITY;
            }
            SpriteUpdate::Static => {
                sprite.image = game_assets.player_static.clone();
                sprite.color = sprite.color.with_alpha(1.0);
                *kind = PlayerSpriteKind::Static;
                sprite_transform.rotation = Quat::IDENTITY;
            }
        }
    }
}

fn aabb_overlap(center_a: Vec2, half_a: Vec2, center_b: Vec2, half_b: Vec2) -> bool {
    (center_a.x - center_b.x).abs() < (half_a.x + half_b.x)
        && (center_a.y - center_b.y).abs() < (half_a.y + half_b.y)
}
