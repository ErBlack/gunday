use super::components::{Player, PlayerSpriteEntity, PlayerSpriteKind};
use crate::assets::GameAssets;
use crate::systems::WinMusic;
use bevy::prelude::*;

pub fn player_win_pose_system(
    win: Res<WinMusic>,
    parent_q: Query<&PlayerSpriteEntity, With<Player>>,
    mut child_q: Query<(&mut Sprite, &mut Transform, &mut PlayerSpriteKind)>,
    assets: Res<GameAssets>,
) {
    if !win.0 {
        return;
    }
    for sprite_ent in parent_q.iter() {
        if let Ok((mut sprite, mut tr, mut kind)) = child_q.get_mut(**sprite_ent) {
            if *kind != PlayerSpriteKind::Win {
                sprite.image = assets.player_win.clone();
                *kind = PlayerSpriteKind::Win;
                tr.rotation = Quat::IDENTITY;
            }
        }
    }
}
