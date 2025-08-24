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

/// Component for layer geometry objects
#[derive(Component)]
pub struct LayerGeometry {
    pub vertices: Vec<Vec2>,
}

impl LayerGeometry {
    pub fn new_rectangle(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            vertices: vec![
                Vec2::new(x, y),
                Vec2::new(x + width, y),
                Vec2::new(x + width, y + height),
                Vec2::new(x, y + height),
            ]
        }
    }
    
    pub fn from_coords(coords: Vec<(f32, f32)>) -> Self {
        Self {
            vertices: coords.into_iter().map(|(x, y)| Vec2::new(x, y)).collect()
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
                // Add the requested rectangle: 0,0; 1280,0; 1280,60; 0,60
                LayerGeometry::from_coords(vec![
                    (0.0, 0.0),
                    (1280.0, 0.0),
                    (1280.0, 60.0),
                    (0.0, 60.0),
                ])
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
