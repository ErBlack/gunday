use bevy::prelude::*;
use crate::components::*;

/// Setup the camera
pub fn setup_camera(mut commands: Commands) {
    // Calculate initial camera position for right-facing player
    let spawn_x = -WORLD_WIDTH / 2.0 + SCREEN_WIDTH / 4.0;
    let initial_camera_x = spawn_x + SCREEN_WIDTH / 4.0;
    
    commands.spawn((
        Camera2d::default(),
        MainCamera,
        CameraState {
            target_x: initial_camera_x,
            current_x: initial_camera_x,
            animation_timer: 0.0,
            animation_duration: 1.0,
            start_x: initial_camera_x,
            is_animating: false,
            last_facing_right: true,
        },
        Transform::from_translation(Vec3::new(initial_camera_x, 0.0, 0.0)),
    ));
}

/// Setup the world background with gradient
pub fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create a gradient background that spans the entire world width
    let gradient_segments = 32; // Number of segments for smooth gradient
    let segment_width = WORLD_WIDTH / gradient_segments as f32;
    
    for i in 0..gradient_segments {
        let t = i as f32 / (gradient_segments - 1) as f32; // 0.0 to 1.0
        
        // Interpolate from green (0.0, 0.8, 0.0) to dark green (0.0, 0.3, 0.0)
        let color = Color::srgb(
            0.0,
            0.8 * (1.0 - t) + 0.3 * t, // Green component interpolation
            0.0
        );
        
        let x_pos = -WORLD_WIDTH / 2.0 + (i as f32 + 0.5) * segment_width;
        
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(segment_width, SCREEN_HEIGHT * 2.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
            Transform::from_translation(Vec3::new(x_pos, 0.0, -10.0)), // Behind everything
        ));
    }
}

/// Setup the ground
pub fn setup_ground(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.3, 0.1), // Brown ground color
            custom_size: Some(Vec2::new(WORLD_WIDTH, GROUND_HEIGHT)), // World width, 60px height
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, -210.0, 0.0)), // Bottom of screen
        Ground,
    ));
}

/// Setup the player character
pub fn setup_player(mut commands: Commands) {
    // Calculate spawn position: first 1/4 of screen width from start of world
    let spawn_x = -WORLD_WIDTH / 2.0 + SCREEN_WIDTH / 4.0;
    
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
        let max_x = WORLD_WIDTH / 2.0 - PLAYER_WIDTH / 2.0;
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
            let world_left_bound = -WORLD_WIDTH / 2.0 + half_screen_width;
            let world_right_bound = WORLD_WIDTH / 2.0 - half_screen_width;
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
