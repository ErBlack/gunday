use bevy::prelude::*;
use crate::components::*;

/// Setup the camera
pub fn setup_camera(mut commands: Commands) {
    // Calculate initial camera position for right-facing player
    let spawn_x = SCREEN_WIDTH / 4.0;
    let initial_camera_x = spawn_x + SCREEN_WIDTH / 4.0;
    
    commands.spawn((
        Camera2d::default(),
        MainCamera,
        CameraState {
            target_x: initial_camera_x,
            current_x: initial_camera_x,
            animation_timer: 0.0,
            animation_duration: 1.0,
            start_x: initial_camera_x,
            is_animating: false,
            last_facing_right: true,
        },
        Transform::from_translation(Vec3::new(initial_camera_x, 0.0, 0.0)),
    ));
}

/// Setup the world background with gradient
pub fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create a gradient background that spans the entire world width
    let gradient_segments = 32; // Number of segments for smooth gradient
    let segment_width = WORLD_WIDTH / gradient_segments as f32;
    
    for i in 0..gradient_segments {
        let t = i as f32 / (gradient_segments - 1) as f32; // 0.0 to 1.0
        
        // Interpolate from green (0.0, 0.8, 0.0) to dark green (0.0, 0.3, 0.0)
        let color = Color::srgb(
            0.0,
            0.8 * (1.0 - t) + 0.3 * t, // Green component interpolation
            0.0
        );
        
        let x_pos = (i as f32 + 0.5) * segment_width;
        
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(segment_width, SCREEN_HEIGHT * 2.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
            Transform::from_translation(Vec3::new(x_pos, 0.0, -10.0)), // Behind everything
        ));
    }
}

/// Setup the ground
pub fn setup_ground(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.3, 0.1), // Brown ground color
            custom_size: Some(Vec2::new(WORLD_WIDTH, GROUND_HEIGHT)), // World width, 60px height
            ..default()
        },
        Transform::from_translation(Vec3::new(WORLD_WIDTH / 2.0, -210.0, 0.0)), // Bottom of screen, centered in world
        Ground,
    ));
}

/// Move projectiles and handle cleanup
pub fn projectile_system(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_query: Query<(Entity, &mut Transform, &Projectile)>,
) {
    for (entity, mut transform, projectile) in projectile_query.iter_mut() {
        // Move projectile
        let movement = projectile.direction * projectile.speed * time.delta_secs();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
        
        // Remove projectiles that are off-screen or out of world bounds
        if transform.translation.x < -100.0 
            || transform.translation.x > WORLD_WIDTH + 100.0
            || transform.translation.y < -300.0 
            || transform.translation.y > 300.0 {
            commands.entity(entity).despawn();
        }
    }
}
