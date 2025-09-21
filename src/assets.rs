use bevy::prelude::*;

/// Resource that holds references to loaded game assets
#[derive(Resource)]
pub struct GameAssets {
    pub level1_background: Handle<Image>,
    // Add more assets here as needed
    // pub player_sprite: Handle<Image>,
    // pub enemy_sprites: Vec<Handle<Image>>,
}

/// System to preload all game assets
pub fn load_game_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let game_assets = GameAssets {
        level1_background: asset_server.load("levels/level.png"),
        // Load more assets here
    };
    
    commands.insert_resource(game_assets);
}

/// Component to mark entities that should render the level background
#[derive(Component)]
pub struct LevelBackground;

/// System to spawn the level background sprite (initial spawn)
pub fn setup_level_background(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    commands.spawn((
        Sprite::from_image(game_assets.level1_background.clone()),
        Transform::from_xyz(0.0, 0.0, -10.0), // Temporary position
        LevelBackground,
    ));
}

/// System to position the level background correctly once the image is loaded
pub fn position_level_background(
    mut background_query: Query<&mut Transform, With<LevelBackground>>,
    game_assets: Res<GameAssets>,
    images: Res<Assets<Image>>,
) {
    if let Some(image) = images.get(&game_assets.level1_background) {
        for mut transform in background_query.iter_mut() {
            let image_width = image.width() as f32;
            
            // Position so the left edge of the image aligns with x = 0 (level start)
            // Since sprites are positioned by their center, offset by half the image width
            let background_x = image_width / 2.0;
            
            transform.translation.x = background_x;
            transform.translation.y = 0.0;
            transform.translation.z = -10.0;
        }
    }
}