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
                LayerGeometry::new_rectangle(0.0, 0.0, 4063.0, 182.0),
                LayerGeometry::new_rectangle(4252.0, 0.0, 1056.0, 182.0),
                LayerGeometry::new_rectangle(5308.0, 0.0, 1584.0, 87.0),
                LayerGeometry::new_rectangle(7084.0, 0.0, 2953.0, 87.0),
                LayerGeometry::new_rectangle(10037.0, 0.0, 1611.0, 182.0),
                
                LayerGeometry::new_rectangle(896.0, 288.0, 24.0, 384.0),

                LayerGeometry::new_rectangle(1666.0, 182.0, 759.0, 97.0),
                LayerGeometry::new_rectangle(1762.0, 279.0, 663.0, 97.0),
                LayerGeometry::new_rectangle(1858.0, 376.0, 567.0, 97.0),
                
                LayerGeometry::new_rectangle(5074.0, 182.0, 234.0, 97.0),

                LayerGeometry::new_rectangle(2578.0, 424.0, 663.0, 48.0),
                LayerGeometry::new_rectangle(2914.0, 279.0, 380.0, 48.0),
                LayerGeometry::new_rectangle(3445.0, 424.0, 570.0, 48.0),
                LayerGeometry::new_rectangle(3394.0, 279.0, 380.0, 48.0),
                LayerGeometry::new_rectangle(4162.0, 424.0, 570.0, 48.0),
                LayerGeometry::new_rectangle(4450.0, 279.0, 285.0, 48.0),
                
                LayerGeometry::new_rectangle(6420.0, 231.0, 184.0, 48.0),
                LayerGeometry::new_rectangle(6706.0, 327.0, 184.0, 48.0),
                LayerGeometry::new_rectangle(7084.0, 327.0, 618.0, 48.0),

                LayerGeometry::new_rectangle(7762.0, 87.0, 618.0, 97.0),
                LayerGeometry::new_rectangle(7954.0, 184.0, 426.0, 97.0),
            ]
        }
    }
}


/// World and camera constants
pub const SCREEN_WIDTH: f32 = 896.0;
pub const SCREEN_HEIGHT: f32 = 672.0;
pub const WORLD_WIDTH: f32 = 11648.0;
