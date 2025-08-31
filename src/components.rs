use bevy::prelude::*;

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

/// Component for layer geometry objects
#[derive(Component)]
pub struct LayerGeometry {
    pub bottom_left: Vec2,
    pub width: f32,
    pub height: f32,
}

/// Component to mark objects as solid/collidable
#[derive(Component)]
pub struct Solid;

impl LayerGeometry {
    pub fn new_rectangle(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            bottom_left: Vec2::new(x, y),
            width,
            height,
        }
    }
    
    pub fn from_coords(coords: Vec<(f32, f32)>) -> Self {
        if coords.len() < 2 {
            panic!("Need at least 2 coordinates to create a rectangle");
        }
        
        let min_x = coords.iter().map(|(x, _)| *x).fold(f32::INFINITY, f32::min);
        let max_x = coords.iter().map(|(x, _)| *x).fold(f32::NEG_INFINITY, f32::max);
        let min_y = coords.iter().map(|(_, y)| *y).fold(f32::INFINITY, f32::min);
        let max_y = coords.iter().map(|(_, y)| *y).fold(f32::NEG_INFINITY, f32::max);
        
        Self {
            bottom_left: Vec2::new(min_x, min_y),
            width: max_x - min_x,
            height: max_y - min_y,
        }
    }
}

/// Resource for storing all layer geometry data
#[derive(Resource)]
pub struct LayerGeometryStorage {
    pub objects: Vec<LayerGeometry>,
}

impl Default for LayerGeometryStorage {
    fn default() -> Self {
        Self {
            objects: vec![
                // Rectangle with bottom-left at (0,0), width 1280, height 60
                LayerGeometry::new_rectangle(0.0, 0.0, 1280.0, 60.0),
            ]
        }
    }
}

/// Size constants
pub const GROUND_HEIGHT: f32 = 60.0;

/// World and camera constants
pub const SCREEN_WIDTH: f32 = 640.0;
pub const SCREEN_HEIGHT: f32 = 480.0;
pub const WORLD_WIDTH: f32 = 8320.0;
