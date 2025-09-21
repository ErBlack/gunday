use super::components::{Player, PlayerHeartIcon, PlayerHeartsRoot, PlayerLives};
use crate::assets::GameAssets;
use crate::components::MainCamera;
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH, Z_FOREGROUND};
use crate::player::PLAYER_CONFIG;
use bevy::prelude::*;

const HEART_SIZE: Vec2 = Vec2::new(21.0, 32.0);

pub fn setup_player_hearts_ui(
    mut commands: Commands,
    camera_query: Query<Entity, With<MainCamera>>,
    existing_ui: Query<Entity, With<PlayerHeartsRoot>>,
    game_assets: Res<GameAssets>,
) {
    if existing_ui.iter().next().is_some() {
        return;
    }
    let Some(camera_entity) = camera_query.iter().next() else {
        return;
    };

    let heart_size = HEART_SIZE;
    let spacing = heart_size.x + PLAYER_CONFIG.heart_spacing;
    let base_x = -SCREEN_WIDTH / 2.0 + PLAYER_CONFIG.hearts_offset.x + heart_size.x * 0.5;
    let base_y = SCREEN_HEIGHT / 2.0 - PLAYER_CONFIG.hearts_offset.y - heart_size.y * 0.5;

    commands.entity(camera_entity).with_children(|parent| {
        parent
            .spawn((
                PlayerHeartsRoot,
                Transform::from_xyz(0.0, 0.0, Z_FOREGROUND + 10.0),
                GlobalTransform::default(),
                Visibility::Visible,
                InheritedVisibility::default(),
            ))
            .with_children(|root| {
                for i in 0..PLAYER_CONFIG.max_lives {
                    let offset_x = base_x + (i as f32) * spacing;
                    let translation = Vec3::new(offset_x, base_y, 0.0);
                    root.spawn((
                        Sprite {
                            image: game_assets.ui_heart.clone(),
                            custom_size: Some(heart_size),
                            ..default()
                        },
                        Transform::from_translation(translation),
                        GlobalTransform::default(),
                        Visibility::Visible,
                        InheritedVisibility::default(),
                        PlayerHeartIcon { index: i },
                    ));
                }
            });
    });
}

pub fn player_hearts_update_system(
    player_query: Query<&PlayerLives, With<Player>>,
    mut heart_query: Query<(&PlayerHeartIcon, &mut Visibility)>,
) {
    let Some(lives) = player_query.iter().next() else {
        return;
    };

    for (icon, mut visibility) in heart_query.iter_mut() {
        *visibility = if icon.index < lives.current {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
