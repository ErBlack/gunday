use bevy::prelude::*;
use crate::components::*;

/// Setup the camera
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

/// Setup the ground
pub fn setup_ground(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.8, 0.0), // Green color
            custom_size: Some(Vec2::new(GROUND_WIDTH, GROUND_HEIGHT)), // Full width, 60px height
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, -210.0, 0.0)), // Bottom of screen
        Ground,
    ));
}

/// Setup the player character
pub fn setup_player(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, -210.0 + (GROUND_HEIGHT / 2.0) + (PLAYER_HEIGHT / 2.0), 1.0)), // On ground surface
        Player,
        Velocity::default(),
        Gravity::default(),
        Grounded { is_grounded: true }, // Start grounded
        JumpState::default(),
    ));
}

/// Handle player input
pub fn player_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Velocity, &Grounded, &mut JumpState), With<Player>>,
) {
    for (mut velocity, grounded, mut jump_state) in player_query.iter_mut() {
        if grounded.is_grounded {
            // Ground movement - with active deceleration
            let mut ground_input = 0.0;
            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                ground_input = -1.0;
            }
            if keyboard_input.pressed(KeyCode::ArrowRight) {
                ground_input = 1.0;
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

/// Apply movement based on velocity
pub fn player_movement_system(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Velocity), With<Player>>,
) {
    for (mut transform, velocity) in player_query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();

        // Keep player within screen bounds (horizontal)
        let screen_width = 640.0;
        let max_x = screen_width / 2.0 - PLAYER_WIDTH / 2.0;
        transform.translation.x = transform.translation.x.clamp(-max_x, max_x);
    }
}

/// Apply gravity to entities with Gravity component
pub fn apply_gravity_system(
    time: Res<Time>,
    mut gravity_query: Query<(&mut Velocity, &Gravity, Option<&JumpState>)>,
) {
    for (mut velocity, gravity, jump_state) in gravity_query.iter_mut() {
        // Only apply gravity if not actively jumping
        let should_apply_gravity = match jump_state {
            Some(js) => !js.is_jumping,
            None => true, // Apply gravity to entities without JumpState
        };
        
        if should_apply_gravity {
            velocity.y += gravity.force * time.delta_secs();
        }
    }
}

/// Handle collision with ground
pub fn ground_collision_system(
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut Grounded), With<Player>>,
    ground_query: Query<&Transform, (With<Ground>, Without<Player>)>,
) {
    for (mut player_transform, mut velocity, mut grounded) in player_query.iter_mut() {
        for ground_transform in ground_query.iter() {
            let player_bottom = player_transform.translation.y - (PLAYER_HEIGHT / 2.0); // Bottom of player
            let ground_top = ground_transform.translation.y + (GROUND_HEIGHT / 2.0); // Top of ground

            // Check if player is touching or below ground
            if player_bottom <= ground_top {
                // Place player on top of ground (bottom of player touching top of ground)
                player_transform.translation.y = ground_top + (PLAYER_HEIGHT / 2.0);
                
                // Stop downward velocity
                if velocity.y < 0.0 {
                    velocity.y = 0.0;
                }
                
                grounded.is_grounded = true;
            } else {
                grounded.is_grounded = false;
            }
        }
    }
}
