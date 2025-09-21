use super::components::*;
use crate::assets::GameAssets;
use crate::constants::{GROUND_RECT_HEIGHT, SCREEN_HEIGHT, SCREEN_WIDTH, Z_PLAYER_BASE};
use crate::player::PLAYER_CONFIG;
use bevy::prelude::*;

pub fn setup_player(mut commands: Commands, game_assets: Res<GameAssets>) {
    let spawn_x = SCREEN_WIDTH * PLAYER_CONFIG.spawn_screen_fraction;
    let spawn_world_y = GROUND_RECT_HEIGHT + PLAYER_CONFIG.ground_collider.y / 2.0;

    let translation = Vec3::new(
        spawn_x,
        spawn_world_y - (SCREEN_HEIGHT / 2.0),
        Z_PLAYER_BASE,
    );

    let sprite_size = SpriteSize {
        width: PLAYER_CONFIG.ground_collider.x,
        height: PLAYER_CONFIG.ground_collider.y,
    };

    let spawn_translation = translation;

    let parent_id = commands
        .spawn(PlayerBundle::new(
            Transform::from_translation(spawn_translation),
            sprite_size,
        ))
        .id();
    let mut sprite_entity_opt: Option<Entity> = None;
    commands.entity(parent_id).with_children(|parent| {
        let child_id = parent
            .spawn((
                Sprite::from_image(game_assets.player_static.clone()),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
                PlayerSprite,
                PlayerSpriteOffset::default(),
                PlayerSpriteKind::Static,
            ))
            .id();
        sprite_entity_opt = Some(child_id);
    });
    if let Some(sprite_entity) = sprite_entity_opt {
        commands
            .entity(parent_id)
            .insert(PlayerSpriteEntity(sprite_entity));
    }

    commands.entity(parent_id).insert((
        PlayerLives::new(PLAYER_CONFIG.starting_lives, PLAYER_CONFIG.max_lives),
        PlayerSpawnPoint(spawn_translation),
    ));
}

#[derive(Component)]
pub struct SpriteSize {
    pub width: f32,
    pub height: f32,
}
