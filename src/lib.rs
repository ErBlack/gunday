use bevy::prelude::*;
use wasm_bindgen::prelude::*;

mod components;
mod systems;

use systems::*;

#[wasm_bindgen]
pub fn run_app() {
    main();
}

pub fn main() {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Gunday - Contra-like Game".into(),
            resolution: (640.0, 480.0).into(),
            resizable: false,
            canvas: Some("#bevy-canvas".to_string()),
            ..default()
        }),
        ..default()
    }));
    
    app.insert_resource(ClearColor(Color::srgb(0.5, 0.8, 0.5))) // Light green background
        .add_systems(Startup, (setup_camera, setup_background, setup_ground, setup_player))
        .add_systems(Update, (
            apply_gravity_system,
            player_input_system,
            player_movement_system,
            ground_collision_system,
            camera_follow_system,
        ))
        .run();
}
