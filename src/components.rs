use bevy::prelude::*;


#[derive(Component)]
pub struct MainCamera;


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
            animation_duration: 1.0,
            start_x: 0.0,
            is_animating: false,
            last_facing_right: true,
        }
    }
}

#[derive(Component)]
pub struct LayerGeometry {
    pub bottom_left: Vec2,
    pub width: f32,
    pub height: f32,
}

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
}

#[derive(Resource)]
pub struct LayerGeometryStorage {
    pub objects: Vec<LayerGeometry>,
}

impl Default for LayerGeometryStorage {
    fn default() -> Self {
        Self {
            objects: vec![
                LayerGeometry::new_rectangle(0.0, 0.0, 11648.0, 124.0),
            
            ]
        }
    }
}

pub const SCREEN_WIDTH: f32 = 896.0;
pub const SCREEN_HEIGHT: f32 = 672.0;
pub const WORLD_WIDTH: f32 = 11648.0;
