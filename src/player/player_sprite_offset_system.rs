use super::components::{Player, PlayerSpriteEntity, PlayerSpriteKind, PlayerSpriteOffset};
use bevy::prelude::*;

fn offset_for_kind(kind: &PlayerSpriteKind) -> Vec2 {
    match kind {
        PlayerSpriteKind::Static => Vec2::new(0.0, 0.0),
        PlayerSpriteKind::Up => Vec2::new(-5.0, 16.0),
        PlayerSpriteKind::Shooting => Vec2::new(0.0, 0.0),
        PlayerSpriteKind::Run(_f) => Vec2::new(0.0, 0.0),
        PlayerSpriteKind::RunUp(_f) => Vec2::new(0.0, 6.0),
        PlayerSpriteKind::RunDown(_f) => Vec2::new(0.0, 0.0),
        PlayerSpriteKind::Jump => Vec2::new(0.0, 0.0),
        PlayerSpriteKind::Hit => Vec2::new(0.0, 0.0),
        PlayerSpriteKind::Fallen => Vec2::new(0.0, -25.0),
        PlayerSpriteKind::Win => Vec2::new(0.0, 0.0),
    }
}

pub fn player_sprite_offset_system(
    parent_query: Query<&PlayerSpriteEntity, With<Player>>,
    mut sprite_query: Query<(&PlayerSpriteKind, &mut Transform, &mut PlayerSpriteOffset)>,
) {
    for sprite_ent in parent_query.iter() {
        if let Ok((kind, mut transform, mut offset)) = sprite_query.get_mut(**sprite_ent) {
            let desired = offset_for_kind(kind);
            if offset.0 != desired {
                offset.0 = desired;
                transform.translation.x = desired.x;
                transform.translation.y = desired.y;
            }
        }
    }
}
