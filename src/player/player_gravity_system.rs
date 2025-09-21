use bevy::prelude::*;
use super::components::*;


pub fn player_gravity_system(
    time: Res<Time>,
    mut gravity_query: Query<(&mut Velocity, &Gravity, Option<&JumpState>)>,
) {
    for (mut velocity, gravity, jump_state) in gravity_query.iter_mut() {
        let should_apply_gravity = match jump_state {
            Some(js) => !js.is_jumping,
            None => true,
        };
        
        if should_apply_gravity {
            velocity.y += gravity.force * time.delta_secs();
        }
    }
}
