use bevy::prelude::*;
use crate::components::Projectile;
use super::components::*;

/// Projectile constants
const PROJECTILE_SIZE: f32 = 11.2;
const PROJECTILE_SPEED: f32 = 1680.0;

/// Handle player shooting input
pub fn player_shooting_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&Transform, &PlayerDirection, &Grounded, &mut ShootingState), With<Player>>,
) {
    for (player_transform, player_direction, grounded, mut shooting_state) in player_query.iter_mut() {
        // Update shot timer
        shooting_state.last_shot_timer += time.delta_secs();
        
        // Check if player wants to shoot and cooldown has passed
        if keyboard_input.pressed(KeyCode::ControlLeft) || keyboard_input.pressed(KeyCode::ControlRight) 
            || keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight) {
            if shooting_state.last_shot_timer >= shooting_state.shot_cooldown {
                // Calculate shooting direction based on input
                let mut direction = Vec2::ZERO;
                
                // Horizontal direction (default forward)
                if player_direction.facing_right {
                    direction.x = 1.0;
                } else {
                    direction.x = -1.0;
                }
                
                // Vertical direction based on input
                if keyboard_input.pressed(KeyCode::ArrowUp) {
                    direction.y = 1.0;
                    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::ArrowRight) {
                        // 45 degree diagonal up
                        direction = direction.normalize();
                    } else {
                        // Straight up
                        direction = Vec2::Y;
                    }
                } else if keyboard_input.pressed(KeyCode::ArrowDown) {
                    if !grounded.is_grounded {
                        // In air and down pressed - shoot straight down
                        direction = Vec2::NEG_Y;
                    } else if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::ArrowRight) {
                        // 45 degree diagonal down
                        direction.y = -1.0;
                        direction = direction.normalize();
                    }
                }
                
                // If no vertical input, shoot horizontally
                if direction.y == 0.0 && direction.x != 0.0 {
                    direction.y = 0.0; // Ensure horizontal
                }
                
                // Normalize direction if not already done
                if direction.length() > 0.0 {
                    direction = direction.normalize();
                    
                    // Calculate projectile spawn position (at player center)
                    let spawn_pos = Vec3::new(
                        player_transform.translation.x,
                        player_transform.translation.y,
                        player_transform.translation.z + 0.1, // Slightly in front
                    );
                    
                    // Spawn projectile
                    commands.spawn((
                        Sprite {
                            color: Color::srgb(1.0, 1.0, 0.0), // Yellow projectile
                            custom_size: Some(Vec2::new(PROJECTILE_SIZE, PROJECTILE_SIZE)),
                            ..default()
                        },
                        Transform::from_translation(spawn_pos),
                        Projectile {
                            direction,
                            speed: PROJECTILE_SPEED,
                        },
                    ));
                    
                    // Reset shot timer
                    shooting_state.last_shot_timer = 0.0;
                }
            }
        }
    }
}
