use bevy::prelude::*;
use super::components::*; // Import player components from local module

/// Apply gravity to entities with Gravity component
pub fn player_gravity_system(
    time: Res<Time>,
    mut gravity_query: Query<(&mut Velocity, &Gravity, Option<&JumpState>)>,
) {
    for (mut velocity, gravity, jump_state) in gravity_query.iter_mut() {
        // Only apply gravity if not actively jumping
        let should_apply_gravity = match jump_state {
            Some(js) => !js.is_jumping,
            None => true, // Apply gravity to entities without JumpState
        };
        
        if should_apply_gravity {
            velocity.y += gravity.force * time.delta_secs();
        }
    }
}
