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

/// Movement speed constants
pub const PLAYER_SPEED: f32 = 200.0;
pub const JUMP_FORCE: f32 = 200.0;

/// Physics constants
pub const AIR_RESISTANCE: f32 = 0.98; // Inertia factor when flying (0.98 = 2% speed loss per frame)
pub const AIR_ACCELERATION: f32 = 400.0; // How fast player accelerates in air
pub const GROUND_ACCELERATION: f32 = 600.0; // How fast player accelerates on ground

/// Size constants
pub const PLAYER_WIDTH: f32 = 16.0;
pub const PLAYER_HEIGHT: f32 = 42.0;
pub const GROUND_WIDTH: f32 = 640.0;
pub const GROUND_HEIGHT: f32 = 60.0;
