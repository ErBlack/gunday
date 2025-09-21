use crate::collision::rectangles_collide;
use crate::components::*;
use crate::constants::{BROADPHASE_MARGIN_X, SCREEN_HEIGHT};
use crate::player::components::*;
use bevy::prelude::*;

pub fn player_collision_system(
    mut player_query: Query<
        (
            &mut Transform,
            &mut Velocity,
            &mut Grounded,
            &crate::player::setup_player::SpriteSize,
        ),
        With<Player>,
    >,
    geometry_query: Query<&LayerGeometry, With<Solid>>,
) {
    for (mut player_transform, mut velocity, mut grounded, sprite_size) in player_query.iter_mut() {
        let player_size = Vec2::new(sprite_size.width, sprite_size.height);
        let mut on_ground = false;
        let player_world_y = player_transform.translation.y + (SCREEN_HEIGHT / 2.0);
        let player_bottom = player_world_y - (sprite_size.height / 2.0);
        let player_top = player_world_y + (sprite_size.height / 2.0);

        if player_bottom <= 2.0 {
            let new_world_y = sprite_size.height / 2.0;
            player_transform.translation.y = new_world_y - (SCREEN_HEIGHT / 2.0);
            if velocity.y < 0.0 {
                velocity.y = 0.0;
            }
            on_ground = true;
        }

        if player_top >= SCREEN_HEIGHT - 2.0 {
            let new_world_y = SCREEN_HEIGHT - (sprite_size.height / 2.0);
            player_transform.translation.y = new_world_y - (SCREEN_HEIGHT / 2.0);
            if velocity.y > 0.0 {
                velocity.y = 0.0;
            }
        }

        for geometry in geometry_query.iter() {
            let geometry_max_x = geometry.bottom_left.x + geometry.width;
            let geometry_max_y = geometry.bottom_left.y + geometry.height;
            let player_world_x = player_transform.translation.x;

            let margin = BROADPHASE_MARGIN_X;
            if geometry_max_x < player_world_x - margin
                || geometry.bottom_left.x > player_world_x + margin
            {
                continue;
            }

            if geometry_max_y < player_world_y - margin
                || geometry.bottom_left.y > player_world_y + margin
            {
                continue;
            }

            let player_world_pos = Vec2::new(
                player_transform.translation.x - (sprite_size.width / 2.0),
                player_world_y - (sprite_size.height / 2.0),
            );

            if rectangles_collide(
                player_world_pos,
                player_size,
                geometry.bottom_left,
                Vec2::new(geometry.width, geometry.height),
            ) {
                let player_center_x = player_transform.translation.x;
                let player_center_world_y = player_world_y;
                let geometry_center_x = geometry.bottom_left.x + (geometry.width / 2.0);
                let geometry_center_y = geometry.bottom_left.y + (geometry.height / 2.0);

                let dx = player_center_x - geometry_center_x;
                let dy = player_center_world_y - geometry_center_y;

                let overlap_x = (sprite_size.width / 2.0) + (geometry.width / 2.0) - dx.abs();
                let overlap_y = (sprite_size.height / 2.0) + (geometry.height / 2.0) - dy.abs();

                if overlap_x < overlap_y {
                    if dx > 0.0 {
                        player_transform.translation.x =
                            geometry.bottom_left.x + geometry.width + (sprite_size.width / 2.0);
                    } else {
                        player_transform.translation.x =
                            geometry.bottom_left.x - (sprite_size.width / 2.0);
                    }
                    velocity.x = 0.0;
                } else {
                    if dy > 0.0 {
                        let new_world_y =
                            geometry.bottom_left.y + geometry.height + (sprite_size.height / 2.0);
                        player_transform.translation.y = new_world_y - (SCREEN_HEIGHT / 2.0);
                        if velocity.y < 0.0 {
                            velocity.y = 0.0;
                        }
                        on_ground = true;
                    } else {
                        let new_world_y = geometry.bottom_left.y - (sprite_size.height / 2.0);
                        player_transform.translation.y = new_world_y - (SCREEN_HEIGHT / 2.0);
                        if velocity.y > 0.0 {
                            velocity.y = 0.0;
                        }
                    }
                }
            }
        }

        if !on_ground && velocity.y <= 0.0 {
            let player_min_x = player_transform.translation.x - (sprite_size.width / 2.0);
            let player_max_x = player_transform.translation.x + (sprite_size.width / 2.0);
            for geometry in geometry_query.iter() {
                let geo_min_x = geometry.bottom_left.x;
                let geo_max_x = geometry.bottom_left.x + geometry.width;
                if player_max_x < geo_min_x || player_min_x > geo_max_x {
                    continue;
                }
                let geo_top = geometry.bottom_left.y + geometry.height;
                let dy = player_bottom - geo_top;
                if dy.abs() <= 1.0 {
                    let new_world_y = geo_top + (sprite_size.height / 2.0);
                    player_transform.translation.y = new_world_y - (SCREEN_HEIGHT / 2.0);
                    if velocity.y < 0.0 {
                        velocity.y = 0.0;
                    }
                    on_ground = true;
                    break;
                }
            }
        }

        grounded.is_grounded = on_ground;
    }
}
