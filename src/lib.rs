use bevy::prelude::*;
use wasm_bindgen::prelude::*;

mod components;
mod systems;
mod player;
mod collision;
mod projectile;
mod assets;

use systems::{setup_camera, setup_layer_geometry};
use player::{setup_player, player_input_system, player_movement_system, player_shooting_system, camera_follow_system, player_gravity_system, player_collision_system};
use projectile::{projectile_movement_system, projectile_collision_system};
use assets::{load_game_assets, setup_level_background, position_level_background};
use components::LayerGeometryStorage;

#[wasm_bindgen]
pub fn run_app() {
    main();
}

pub fn main() {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Gunday - Contra-like Game".into(),
            resolution: (896.0, 672.0).into(),
            resizable: false,
            canvas: Some("#bevy-canvas".to_string()),
            ..default()
        }),
        ..default()
    }));
    
    app.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0))) // Black background (transparent to level background)
        .insert_resource(LayerGeometryStorage::default())
        .add_systems(Startup, (
            load_game_assets,
            setup_camera, 
            setup_layer_geometry, 
            setup_player,
            setup_level_background.after(load_game_assets),
        ))
        .add_systems(Update, (
            player_gravity_system,
            player_input_system,
            player_movement_system,
            player_collision_system.after(player_movement_system),
            camera_follow_system,
            player_shooting_system,
            projectile_movement_system,
            projectile_collision_system.after(projectile_movement_system),
            position_level_background,
        ))
        .run();
}
