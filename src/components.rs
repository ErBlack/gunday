use bevy::prelude::*;

/// Component for the player character
#[derive(Component)]
pub struct Player;

/// Component for velocity (physics)
#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

/// Component for entities that are affected by gravity
#[derive(Component)]
pub struct Gravity {
    pub force: f32,
}

impl Default for Gravity {
    fn default() -> Self {
        Self { force: -1000.0 } // Negative because Y axis goes up in Bevy
    }
}

/// Component for the ground
#[derive(Component)]
pub struct Ground;

/// Component for the main camera
#[derive(Component)]
pub struct MainCamera;

/// Component for entities that can be grounded (standing on ground)
#[derive(Component)]
pub struct Grounded {
    pub is_grounded: bool,
}

impl Default for Grounded {
    fn default() -> Self {
        Self { is_grounded: false }
    }
}

/// Component for tracking jump state and timing
#[derive(Component)]
pub struct JumpState {
    pub is_jumping: bool,
    pub jump_timer: f32,
    pub max_jump_duration: f32,
}

impl Default for JumpState {
    fn default() -> Self {
        Self { 
            is_jumping: false, 
            jump_timer: 0.0,
            max_jump_duration: 0.5, // 500ms
        }
    }
}

/// Component for tracking player's facing direction
#[derive(Component)]
pub struct PlayerDirection {
    pub facing_right: bool,
    pub last_movement_direction: f32, // -1.0 for left, 1.0 for right, 0.0 for no movement
}

impl Default for PlayerDirection {
    fn default() -> Self {
        Self { 
            facing_right: true, // Player spawns facing right
            last_movement_direction: 1.0, // Initially facing right
        }
    }
}

/// Component for camera animation state
#[derive(Component)]
pub struct CameraState {
    pub target_x: f32,
    pub current_x: f32,
    pub animation_timer: f32,
    pub animation_duration: f32,
    pub start_x: f32,
    pub is_animating: bool,
    pub last_facing_right: bool,
}

impl Default for CameraState {
    fn default() -> Self {
        Self { 
            target_x: 0.0,
            current_x: 0.0,
            animation_timer: 0.0,
            animation_duration: 1.0, // 1 second default
            start_x: 0.0,
            is_animating: false,
            last_facing_right: true, // Start facing right
        }
    }
}


/// Movement speed constants
pub const PLAYER_SPEED: f32 = 200.0;
pub const JUMP_FORCE: f32 = 200.0;

/// Physics constants
pub const AIR_RESISTANCE: f32 = 0.98; // Inertia factor when flying (0.98 = 2% speed loss per frame)
pub const AIR_ACCELERATION: f32 = 400.0; // How fast player accelerates in air
pub const GROUND_ACCELERATION: f32 = 600.0; // How fast player accelerates on ground

/// Size constants
pub const PLAYER_WIDTH: f32 = 28.0;
pub const PLAYER_HEIGHT: f32 = 68.0;
pub const GROUND_HEIGHT: f32 = 60.0;

/// World and camera constants
pub const SCREEN_WIDTH: f32 = 640.0;
pub const SCREEN_HEIGHT: f32 = 480.0;
pub const WORLD_WIDTH: f32 = SCREEN_WIDTH * 13.0;
