use super::components::{
    Grounded, Player, PlayerJumpAnim, PlayerProne, PlayerRespawning, PlayerSprite,
    PlayerSpriteEntity, PlayerSpriteKind,
};
use crate::assets::GameAssets;
use bevy::prelude::*;

pub fn player_jump_anim_system(
    mut parent_query: Query<
        (
            &mut PlayerJumpAnim,
            &Grounded,
            &PlayerSpriteEntity,
            Option<&PlayerProne>,
            Option<&PlayerRespawning>,
        ),
        With<Player>,
    >,
    mut sprite_query: Query<
        (&mut Sprite, &mut Transform, &mut PlayerSpriteKind),
        With<PlayerSprite>,
    >,
    time: Res<Time>,
    game_assets: Res<GameAssets>,
) {
    for (mut jump_anim, grounded, sprite_ent, prone, respawning) in parent_query.iter_mut() {
        if prone.is_some() || respawning.is_some() {
            continue;
        }
        let Ok((mut sprite, mut sprite_transform, mut kind)) = sprite_query.get_mut(**sprite_ent)
        else {
            continue;
        };
        if *kind == PlayerSpriteKind::Win {
            continue;
        }
        if !grounded.is_grounded {
            jump_anim.timer -= time.delta_secs();
            if jump_anim.timer <= 0.0 {
                jump_anim.frame = if jump_anim.frame >= 7 {
                    0
                } else {
                    jump_anim.frame + 1
                };
                jump_anim.timer = 0.1;
            }
            let rotation_deg = (jump_anim.frame as f32) * 45.0;
            jump_anim.rotation = rotation_deg;
            sprite.image = game_assets.player_jump.clone();
            *kind = PlayerSpriteKind::Jump;
            sprite_transform.rotation = Quat::from_rotation_z(rotation_deg.to_radians());
        } else {
            jump_anim.frame = 0;
            jump_anim.timer = 0.0;
            jump_anim.rotation = 0.0;
            sprite_transform.rotation = Quat::IDENTITY;
        }
    }
}
