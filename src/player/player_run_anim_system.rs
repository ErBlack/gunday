use super::components::{
    Grounded, Player, PlayerActions, PlayerDirection, PlayerProne, PlayerRespawning, PlayerRunAnim,
    PlayerShootingAnim, PlayerSprite, PlayerSpriteEntity, PlayerSpriteKind, Velocity,
};
use crate::assets::GameAssets;
use bevy::prelude::*;

pub fn player_run_anim_system(
    mut parent_query: Query<
        (
            &mut PlayerRunAnim,
            &PlayerDirection,
            &Velocity,
            &PlayerShootingAnim,
            &PlayerSpriteEntity,
            &Grounded,
            Option<&PlayerProne>,
            Option<&PlayerRespawning>,
        ),
        With<Player>,
    >,
    mut child_sprites: Query<(&mut Sprite, &mut PlayerSpriteKind), With<PlayerSprite>>,
    time: Res<Time>,
    game_assets: Res<GameAssets>,
    actions: Res<PlayerActions>,
) {
    for (
        mut run_anim,
        _direction,
        velocity,
        shooting_anim,
        sprite_entity,
        grounded,
        prone,
        respawning,
    ) in parent_query.iter_mut()
    {
        if prone.is_some() || respawning.is_some() {
            continue;
        }

        if !grounded.is_grounded {
            continue;
        }
        let Ok((mut sprite, mut kind)) = child_sprites.get_mut(**sprite_entity) else {
            continue;
        };
        if *kind == PlayerSpriteKind::Win {
            continue;
        }
        if velocity.x.abs() > 1.0 {
            let min_fps = 4.0;
            let max_fps = 10.0;
            let speed = velocity.x.abs();
            let fps = (min_fps + (speed / 10.0) * (max_fps - min_fps)).clamp(min_fps, max_fps);
            let frame_time = 1.0 / fps;
            run_anim.timer -= time.delta_secs();
            if run_anim.timer <= 0.0 {
                run_anim.frame = if run_anim.frame == 4 {
                    1
                } else {
                    run_anim.frame + 1
                };
                run_anim.timer = frame_time;
            }
            let sprite_handle = if actions.aim_up.pressed {
                match run_anim.frame {
                    1 => &game_assets.player_run_up_a,
                    2 => &game_assets.player_run_up_b,
                    3 => &game_assets.player_run_up_c,
                    4 => &game_assets.player_run_up_d,
                    _ => &game_assets.player_run_up_a,
                }
            } else if actions.aim_down.pressed {
                match run_anim.frame {
                    1 => &game_assets.player_run_down_a,
                    2 => &game_assets.player_run_down_b,
                    3 => &game_assets.player_run_down_c,
                    4 => &game_assets.player_run_down_d,
                    _ => &game_assets.player_run_down_a,
                }
            } else {
                match run_anim.frame {
                    1 => &game_assets.player_run_a,
                    2 => &game_assets.player_run_b,
                    3 => &game_assets.player_run_c,
                    4 => &game_assets.player_run_d,
                    _ => &game_assets.player_run_a,
                }
            };
            if sprite.image != *sprite_handle {
                sprite.image = sprite_handle.clone();
            }
            *kind = if actions.aim_up.pressed {
                PlayerSpriteKind::RunUp(run_anim.frame)
            } else if actions.aim_down.pressed {
                PlayerSpriteKind::RunDown(run_anim.frame)
            } else {
                PlayerSpriteKind::Run(run_anim.frame)
            };
        } else {
            run_anim.frame = 0;
            run_anim.timer = 0.0;
            if shooting_anim.timer <= 0.0 {
                if actions.aim_up.pressed {
                    if sprite.image != game_assets.player_up {
                        sprite.image = game_assets.player_up.clone();
                    }
                    *kind = PlayerSpriteKind::Up;
                } else {
                    if sprite.image != game_assets.player_static {
                        sprite.image = game_assets.player_static.clone();
                    }
                    *kind = PlayerSpriteKind::Static;
                }
            }
        }
    }
}
