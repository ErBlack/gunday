use bevy::prelude::*;

mod components;
mod systems;

use systems::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run_app() {
    main();
}

pub fn main() {
    let mut app = App::new();
    
    #[cfg(target_arch = "wasm32")]
    {
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
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Gunday - Contra-like Game".into(),
                resolution: (640.0, 480.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }));
    }
    
    app.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.5))) // Navy background
        .add_systems(Startup, (setup_camera, setup_ground, setup_player))
        .add_systems(Update, (
            apply_gravity_system,
            player_input_system,
            player_movement_system,
            ground_collision_system,
        ))
        .run();
}
