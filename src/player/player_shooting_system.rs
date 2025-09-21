use super::components::{
    Grounded, Player, PlayerActions, PlayerDirection, PlayerProne, PlayerRespawning, PlayerRunAnim,
    PlayerShootingAnim, PlayerSprite, PlayerSpriteEntity, PlayerSpriteKind, ShootingState,
};
use crate::assets::GameAssets;
use crate::projectile::spawn_projectile;
use bevy::prelude::*;

pub fn player_shooting_system(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<
        (
            &Transform,
            &PlayerDirection,
            &Grounded,
            &mut ShootingState,
            &super::setup_player::SpriteSize,
            &mut PlayerShootingAnim,
            &mut PlayerRunAnim,
            &PlayerSpriteEntity,
            Option<&PlayerProne>,
            Option<&PlayerRespawning>,
        ),
        With<Player>,
    >,
    mut sprite_query: Query<(&mut Sprite, &mut PlayerSpriteKind), With<PlayerSprite>>,
    game_assets: Res<GameAssets>,
    actions: Res<PlayerActions>,
) {
    for (
        player_transform,
        player_direction,
        grounded,
        mut shooting_state,
        sprite_size,
        mut shooting_anim,
        run_anim,
        sprite_entity,
        prone,
        respawning,
    ) in player_query.iter_mut()
    {
        if prone.is_some() || respawning.is_some() {
            continue;
        }

        let Ok((mut sprite, mut kind)) = sprite_query.get_mut(**sprite_entity) else {
            continue;
        };
        if *kind == PlayerSpriteKind::Win {
            continue;
        }
        shooting_state.last_shot_timer += time.delta_secs();
        let is_static = run_anim.frame == 0;
        let shoot_pressed = actions.shoot.pressed;

        let aiming_horizontal = actions.aim_axis.x.abs() > f32::EPSILON;
        let shooting_up = actions.aim_up.pressed && !aiming_horizontal;

        if grounded.is_grounded && is_static && shooting_anim.timer > 0.0 {
            if shooting_up {
                sprite.image = game_assets.player_up.clone();
                *kind = PlayerSpriteKind::Up;
            } else {
                sprite.image = game_assets.player_shooting.clone();
                *kind = PlayerSpriteKind::Shooting;
            }
            shooting_anim.timer -= time.delta_secs();
            if shooting_anim.timer <= 0.0 {
                shooting_anim.frame = 0;
            }
        }

        if shoot_pressed {
            if shooting_state.last_shot_timer >= shooting_state.shot_cooldown {
                let facing_horizontal = if player_direction.facing_right {
                    1.0
                } else {
                    -1.0
                };
                let aim_horizontal = if aiming_horizontal {
                    actions.aim_axis.x.signum()
                } else {
                    0.0
                };
                let mut direction = Vec2::new(
                    if aim_horizontal != 0.0 {
                        aim_horizontal
                    } else {
                        facing_horizontal
                    },
                    0.0,
                );
                let mut up_shoot = false;
                if actions.aim_up.pressed {
                    if aiming_horizontal {
                        direction.y = 1.0;
                        direction = direction.normalize();
                    } else {
                        direction = Vec2::Y;
                        up_shoot = true;
                    }
                } else if actions.aim_down.pressed {
                    if !grounded.is_grounded {
                        direction = Vec2::NEG_Y;
                    } else if aiming_horizontal {
                        direction.y = -1.0;
                        direction = direction.normalize();
                    }
                }

                if direction.length() > 0.0 {
                    direction = direction.normalize();
                    let mut spawn_pos = player_transform.translation;
                    if !grounded.is_grounded {
                        let offset = direction * 28.0;
                        spawn_pos.x += offset.x;
                        spawn_pos.y += offset.y;
                    } else if up_shoot {
                        let player_width = sprite_size.width;
                        let player_height = sprite_size.height;
                        let top_left_x = player_transform.translation.x - (player_width / 2.0);
                        let top_left_y = player_transform.translation.y + (player_height / 2.0);
                        let gun_x = if player_direction.facing_right {
                            top_left_x + 20.0
                        } else {
                            top_left_x + player_width - 32.0
                        };
                        let gun_y = top_left_y + 40.0;
                        spawn_pos = Vec3::new(gun_x, gun_y, player_transform.translation.z + 0.1);
                    } else if actions.aim_up.pressed && aiming_horizontal {
                        let player_width = sprite_size.width;
                        let player_height = sprite_size.height;
                        let top_left_x = player_transform.translation.x - (player_width / 2.0);
                        let top_left_y = player_transform.translation.y + (player_height / 2.0);

                        let gun_x = if player_direction.facing_right {
                            top_left_x + 64.0
                        } else {
                            top_left_x + player_width - 64.0
                        };
                        let gun_y = top_left_y + 20.0;
                        spawn_pos = Vec3::new(gun_x, gun_y, player_transform.translation.z + 0.1);
                    } else if actions.aim_down.pressed && aiming_horizontal {
                        let player_width = sprite_size.width;
                        let player_height = sprite_size.height;
                        let top_left_x = player_transform.translation.x - (player_width / 2.0);
                        let top_left_y = player_transform.translation.y + (player_height / 2.0);
                        let gun_x = if player_direction.facing_right {
                            top_left_x + 64.0
                        } else {
                            top_left_x + player_width - 64.0
                        };
                        let gun_y = top_left_y - 64.0;
                        spawn_pos = Vec3::new(gun_x, gun_y, player_transform.translation.z + 0.1);
                    } else {
                        let player_width = sprite_size.width;
                        let player_height = sprite_size.height;
                        let top_left_x = player_transform.translation.x - (player_width / 2.0);
                        let top_left_y = player_transform.translation.y + (player_height / 2.0);
                        let gun_x = if player_direction.facing_right {
                            top_left_x + 62.0
                        } else {
                            top_left_x + player_width - 62.0
                        };
                        let gun_y = top_left_y - 24.0;
                        spawn_pos = Vec3::new(gun_x, gun_y, player_transform.translation.z + 0.1);
                    }
                    spawn_projectile(&mut commands, spawn_pos, direction);
                    shooting_state.last_shot_timer = 0.0;
                    shooting_anim.frame = 1;
                    shooting_anim.timer = shooting_state.shot_cooldown;
                }
            }
        }
    }
}
