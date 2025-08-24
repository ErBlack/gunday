use bevy::prelude::*;
use super::components::*;

/// Handle player input
pub fn player_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Velocity, &Grounded, &mut JumpState, &mut PlayerDirection), With<Player>>,
) {
    for (mut velocity, grounded, mut jump_state, mut direction) in player_query.iter_mut() {
        if grounded.is_grounded {
            // Ground movement - with active deceleration
            let mut ground_input = 0.0;
            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                ground_input = -1.0;
            }
            if keyboard_input.pressed(KeyCode::ArrowRight) {
                ground_input = 1.0;
            }
            
            // Update player direction based on input
            if ground_input != 0.0 {
                direction.last_movement_direction = ground_input;
                direction.facing_right = ground_input > 0.0;
            }
            
            // If no input is pressed, apply deceleration force opposite to current movement
            if ground_input == 0.0 && velocity.x.abs() > 0.1 {
                // Apply deceleration force in opposite direction of movement
                ground_input = if velocity.x > 0.0 { -1.0 } else { 1.0 };
            }
            
            // Apply ground acceleration/deceleration
            velocity.x += ground_input * GROUND_ACCELERATION * time.delta_secs();
            
            // Stop completely if we've changed direction (prevents oscillation)
            if grounded.is_grounded && keyboard_input.pressed(KeyCode::ArrowLeft) == false && keyboard_input.pressed(KeyCode::ArrowRight) == false {
                // We're decelerating - check if we've crossed zero
                if (velocity.x > 0.0 && ground_input < 0.0) || (velocity.x < 0.0 && ground_input > 0.0) {
                    // We've crossed zero or very close to it, stop completely
                    if velocity.x.abs() < GROUND_ACCELERATION * time.delta_secs() {
                        velocity.x = 0.0;
                    }
                }
            }
            
            // Clamp ground speed to max speed
            velocity.x = velocity.x.clamp(-PLAYER_SPEED, PLAYER_SPEED);
        } else {
            // Air movement - with inertia
            // Apply air resistance (gradual slowdown)
            velocity.x *= AIR_RESISTANCE;
            
            // Apply air acceleration when keys are pressed
            let mut air_input = 0.0;
            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                air_input = -1.0;
            }
            if keyboard_input.pressed(KeyCode::ArrowRight) {
                air_input = 1.0;
            }
            
            // Update player direction based on input (even in air)
            if air_input != 0.0 {
                direction.last_movement_direction = air_input;
                direction.facing_right = air_input > 0.0;
            }
            
            // Accelerate in air (but slower than ground movement)
            velocity.x += air_input * AIR_ACCELERATION * time.delta_secs();
            
            // Clamp air speed to reasonable limits
            velocity.x = velocity.x.clamp(-PLAYER_SPEED * 1.2, PLAYER_SPEED * 1.2);
        }

        // Variable height jumping system
        if keyboard_input.just_pressed(KeyCode::Space) && grounded.is_grounded {
            // Start jump
            velocity.y = JUMP_FORCE;
            jump_state.is_jumping = true;
            jump_state.jump_timer = 0.0;
        }
        
        // Continue jump while button is held
        if jump_state.is_jumping && keyboard_input.pressed(KeyCode::Space) {
            jump_state.jump_timer += time.delta_secs();
            if jump_state.jump_timer < jump_state.max_jump_duration {
                // Keep applying upward force - this overrides gravity
                velocity.y = JUMP_FORCE;
            } else {
                // Max time reached, let gravity take over
                jump_state.is_jumping = false;
            }
        } else if jump_state.is_jumping {
            // Button was released, let gravity take over
            jump_state.is_jumping = false;
        }
        
        // Reset jump state when landed
        if grounded.is_grounded && velocity.y <= 0.0 {
            jump_state.is_jumping = false;
            jump_state.jump_timer = 0.0;
        }
    }
}
