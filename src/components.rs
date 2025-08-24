use bevy::prelude::*;

/// Component for the ground
#[derive(Component)]
pub struct Ground;

/// Component for the main camera
#[derive(Component)]
pub struct MainCamera;

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

/// Component for projectiles
#[derive(Component)]
pub struct Projectile {
    pub direction: Vec2,
    pub speed: f32,
}


/// Size constants
pub const GROUND_HEIGHT: f32 = 60.0;

/// World and camera constants
pub const SCREEN_WIDTH: f32 = 640.0;
pub const SCREEN_HEIGHT: f32 = 480.0;
pub const WORLD_WIDTH: f32 = SCREEN_WIDTH * 13.0;
