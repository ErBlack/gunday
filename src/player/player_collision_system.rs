use bevy::prelude::*;
use crate::components::*;
use crate::player::components::*;
use crate::collision::rectangles_collide;

/// Collision system for player with layer geometry and screen boundaries
pub fn player_collision_system(
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut Grounded), With<Player>>,
    geometry_query: Query<&LayerGeometry, With<Solid>>,
) {
    for (mut player_transform, mut velocity, mut grounded) in player_query.iter_mut() {
        let player_size = Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT);
        let mut on_ground = false;
        
        // Convert player position to world coordinates
        let player_world_y = player_transform.translation.y + (SCREEN_HEIGHT / 2.0);
        let player_bottom = player_world_y - (PLAYER_HEIGHT / 2.0);
        
        // Check collision with screen bottom (prevent falling through)
        if player_bottom <= 0.0 {
            // Hit screen bottom
            let new_world_y = PLAYER_HEIGHT / 2.0;
            player_transform.translation.y = new_world_y - (SCREEN_HEIGHT / 2.0);
            
            if velocity.y < 0.0 {
                velocity.y = 0.0;
            }
            on_ground = true;
        }
        
        // Check collision with layer geometry
        for geometry in geometry_query.iter() {
            // Convert player position to world coordinates for collision detection
            let player_world_pos = Vec2::new(
                player_transform.translation.x - (PLAYER_WIDTH / 2.0),
                player_world_y - (PLAYER_HEIGHT / 2.0)
            );
            
            // Check if colliding
            if rectangles_collide(
                player_world_pos, player_size,
                geometry.bottom_left, Vec2::new(geometry.width, geometry.height)
            ) {
                // Determine collision side and resolve
                let player_center_x = player_transform.translation.x;
                let player_center_world_y = player_world_y;
                let geometry_center_x = geometry.bottom_left.x + (geometry.width / 2.0);
                let geometry_center_y = geometry.bottom_left.y + (geometry.height / 2.0);
                
                let dx = player_center_x - geometry_center_x;
                let dy = player_center_world_y - geometry_center_y;
                
                let overlap_x = (PLAYER_WIDTH / 2.0) + (geometry.width / 2.0) - dx.abs();
                let overlap_y = (PLAYER_HEIGHT / 2.0) + (geometry.height / 2.0) - dy.abs();
                
                // Resolve collision on the axis with least overlap
                if overlap_x < overlap_y {
                    // Horizontal collision
                    if dx > 0.0 {
                        // Player is to the right, push right
                        player_transform.translation.x = geometry.bottom_left.x + geometry.width + (PLAYER_WIDTH / 2.0);
                    } else {
                        // Player is to the left, push left
                        player_transform.translation.x = geometry.bottom_left.x - (PLAYER_WIDTH / 2.0);
                    }
                    velocity.x = 0.0;
                } else {
                    // Vertical collision
                    if dy > 0.0 {
                        // Player is above, place on top
                        let new_world_y = geometry.bottom_left.y + geometry.height + (PLAYER_HEIGHT / 2.0);
                        player_transform.translation.y = new_world_y - (SCREEN_HEIGHT / 2.0);
                        if velocity.y < 0.0 {
                            velocity.y = 0.0;
                        }
                        on_ground = true;
                    } else {
                        // Player is below, push down
                        let new_world_y = geometry.bottom_left.y - (PLAYER_HEIGHT / 2.0);
                        player_transform.translation.y = new_world_y - (SCREEN_HEIGHT / 2.0);
                        if velocity.y > 0.0 {
                            velocity.y = 0.0;
                        }
                    }
                }
            }
        }
        
        grounded.is_grounded = on_ground;
    }
}
