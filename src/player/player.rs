use bevy::prelude::*;
use crate::components::{Ground, GROUND_HEIGHT, WORLD_WIDTH, SCREEN_WIDTH, Projectile, MainCamera, CameraState}; // Import non-player components from main
use super::components::*; // Import player components from local module

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
        Grounded { is_grounded: true }, // Start grounded
        JumpState::default(),
        PlayerDirection::default(), // Start facing right
        ShootingState::default(), // Add shooting capability
    ));
}

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

/// Apply movement based on velocity
pub fn player_movement_system(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Velocity), With<Player>>,
) {
    for (mut transform, velocity) in player_query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();

        // Keep player within world bounds (horizontal)
        let min_x = PLAYER_WIDTH / 2.0;
        let max_x = WORLD_WIDTH - PLAYER_WIDTH / 2.0;
        transform.translation.x = transform.translation.x.clamp(min_x, max_x);
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

/// Camera following system with direction-change animation
pub fn camera_follow_system(
    time: Res<Time>,
    player_query: Query<(&Transform, &PlayerDirection), (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<(&mut Transform, &mut CameraState), (With<MainCamera>, Without<Player>)>,
) {
    for (player_transform, player_direction) in player_query.iter() {
        for (mut camera_transform, mut camera_state) in camera_query.iter_mut() {
            let player_x = player_transform.translation.x;
            
            // Calculate ideal camera position for current direction
            let quarter_screen = SCREEN_WIDTH / 4.0;
            
            let ideal_camera_x = if player_direction.facing_right {
                // Player facing right: place player at 1/4 from left edge
                player_x + quarter_screen
            } else {
                // Player facing left: place player at 1/4 from right edge  
                player_x - quarter_screen
            };
            
            // Apply world bounds
            let half_screen_width = SCREEN_WIDTH / 2.0;
            let world_left_bound = half_screen_width;
            let world_right_bound = WORLD_WIDTH - half_screen_width;
            let clamped_ideal = ideal_camera_x.clamp(world_left_bound, world_right_bound);
            
            // Check if direction has changed
            if player_direction.facing_right != camera_state.last_facing_right {
                // Direction changed - start animation
                camera_state.last_facing_right = player_direction.facing_right;
                camera_state.start_x = camera_state.current_x;
                camera_state.target_x = clamped_ideal;
                camera_state.animation_timer = 0.0;
                camera_state.is_animating = true;
            }
            
            if camera_state.is_animating {
                // During animation, continuously update target to follow player movement
                camera_state.target_x = clamped_ideal;
                
                // Currently animating - interpolate towards updated target
                camera_state.animation_timer += time.delta_secs();
                
                if camera_state.animation_timer >= camera_state.animation_duration {
                    // Animation complete - snap to current target and switch to following mode
                    camera_state.current_x = camera_state.target_x;
                    camera_state.is_animating = false;
                } else {
                    // Continue animation with smooth interpolation to updated target
                    let t = camera_state.animation_timer / camera_state.animation_duration;
                    let eased_t = t * t * (3.0 - 2.0 * t); // Smoothstep
                    camera_state.current_x = camera_state.start_x + (camera_state.target_x - camera_state.start_x) * eased_t;
                }
            } else {
                // Not animating - snap to follow player immediately
                camera_state.current_x = clamped_ideal;
            }
            
            // Apply camera position
            camera_transform.translation.x = camera_state.current_x;
        }
    }
}

/// Apply gravity to entities with Gravity component
pub fn player_gravity_system(
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
