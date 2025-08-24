use bevy::prelude::*;
use crate::components::{GROUND_HEIGHT, SCREEN_WIDTH};
use super::components::*;

/// Setup the player character
pub fn setup_player(mut commands: Commands) {
    // Calculate spawn position: first 1/4 of screen width from start of world
    let spawn_x = SCREEN_WIDTH / 4.0;
    
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
            ..default()
        },
        Transform::from_translation(Vec3::new(spawn_x, -210.0 + (GROUND_HEIGHT / 2.0) + (PLAYER_HEIGHT / 2.0), 1.0)), // On ground surface
        Player,
        Velocity::default(),
        Gravity::default(),
        Grounded::default(),
        JumpState::default(),
        PlayerDirection::default(), // Start facing right
        ShootingState::default(), // Add shooting capability
    ));
}
