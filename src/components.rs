use crate::constants::{GROUND_RECT_HEIGHT, WORLD_WIDTH};
use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraState {
    pub current_x: f32,
    pub max_reached_x: f32,
    pub lock_position: Option<f32>,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            current_x: 0.0,
            max_reached_x: 0.0,
            lock_position: None,
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
                LayerGeometry::new_rectangle(0.0, 0.0, WORLD_WIDTH, GROUND_RECT_HEIGHT),
                LayerGeometry::new_rectangle(892.0, 232.0, 26.0, 438.0),
                LayerGeometry::new_rectangle(3408.0, 338.0, 158.0, 18.0),
                LayerGeometry::new_rectangle(3924.0, 338.0, 158.0, 18.0),
                LayerGeometry::new_rectangle(4389.0, 338.0, 158.0, 18.0),
                LayerGeometry::new_rectangle(6873.0, 374.0, 18.0, 298.0),
                LayerGeometry::new_rectangle(7895.0, 374.0, 18.0, 298.0),
                LayerGeometry::new_rectangle(8008.0, 374.0, 18.0, 298.0),
                LayerGeometry::new_rectangle(10242.0, 338.0, 117.0, 18.0),
            ],
        }
    }
}
