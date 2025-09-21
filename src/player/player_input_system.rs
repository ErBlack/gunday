use super::components::*;
use crate::player::PLAYER_CONFIG;
use crate::systems::PlayerControl;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

pub fn gather_player_actions(
    time: Res<Time>,
    control: Option<Res<PlayerControl>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut actions: ResMut<PlayerActions>,
) {
    if control.is_some_and(|ctrl| !ctrl.enabled) {
        actions.reset();
        return;
    }

    let dt = time.delta_secs();

    let mut movement: f32 = 0.0;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        movement -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        movement += 1.0;
    }
    actions.move_axis = movement.clamp(-1.0, 1.0);

    let mut aim_axis = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        aim_axis.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        aim_axis.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        aim_axis.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        aim_axis.y -= 1.0;
    }
    if aim_axis.length_squared() > 1.0 {
        aim_axis = aim_axis.normalize();
    }
    actions.aim_axis = aim_axis;

    let shoot_pressed = keyboard_input.pressed(KeyCode::ControlLeft)
        || keyboard_input.pressed(KeyCode::ControlRight)
        || keyboard_input.pressed(KeyCode::ShiftLeft)
        || keyboard_input.pressed(KeyCode::ShiftRight);

    actions
        .jump
        .update(keyboard_input.pressed(KeyCode::Space), dt);
    actions.shoot.update(shoot_pressed, dt);
    actions.dash.update(false, dt);
    actions
        .aim_up
        .update(keyboard_input.pressed(KeyCode::ArrowUp), dt);
    actions
        .aim_down
        .update(keyboard_input.pressed(KeyCode::ArrowDown), dt);
}

pub fn player_input_system(
    actions: Res<PlayerActions>,
    time: Res<Time>,
    mut player_query: Query<
        (
            &mut Velocity,
            &Grounded,
            &mut JumpState,
            &mut PlayerDirection,
            Option<&PlayerProne>,
            Option<&PlayerRespawning>,
        ),
        With<Player>,
    >,
) {
    let dt = time.delta_secs();
    for (mut velocity, grounded, mut jump_state, mut direction, prone, respawning) in
        player_query.iter_mut()
    {
        if prone.is_some() || respawning.is_some() {
            continue;
        }

        if grounded.is_grounded {
            let ground_input = actions.move_axis;

            if ground_input != 0.0 {
                direction.last_movement_direction = ground_input;
                direction.facing_right = ground_input > 0.0;
            }

            let mut effective_ground_input = ground_input;
            if ground_input == 0.0 && velocity.x.abs() > 0.1 {
                effective_ground_input = if velocity.x > 0.0 { -1.0 } else { 1.0 };
            }

            velocity.x += effective_ground_input * PLAYER_CONFIG.ground_acceleration * dt;

            if grounded.is_grounded && actions.move_axis == 0.0 {
                if (velocity.x > 0.0 && effective_ground_input < 0.0)
                    || (velocity.x < 0.0 && effective_ground_input > 0.0)
                {
                    if velocity.x.abs() < PLAYER_CONFIG.ground_acceleration * dt {
                        velocity.x = 0.0;
                    }
                }
            }

            velocity.x = velocity.x.clamp(
                -PLAYER_CONFIG.max_ground_speed,
                PLAYER_CONFIG.max_ground_speed,
            );
        } else {
            velocity.x *= PLAYER_CONFIG.air_resistance;

            let air_input = actions.move_axis;

            if air_input != 0.0 {
                direction.last_movement_direction = air_input;
                direction.facing_right = air_input > 0.0;
            }

            velocity.x += air_input * PLAYER_CONFIG.air_acceleration * dt;

            velocity.x = velocity.x.clamp(
                -PLAYER_CONFIG.max_ground_speed * PLAYER_CONFIG.max_air_speed_multiplier,
                PLAYER_CONFIG.max_ground_speed * PLAYER_CONFIG.max_air_speed_multiplier,
            );
        }

        if actions.jump.just_pressed {
            jump_state.jump_buffer_timer = jump_state.jump_buffer_time;
        }

        if jump_state.jump_buffer_timer > 0.0 {
            jump_state.jump_buffer_timer -= dt;
        }

        if jump_state.jump_buffer_timer > 0.0 && grounded.is_grounded && !jump_state.is_jumping {
            velocity.y = PLAYER_CONFIG.jump_force;
            jump_state.is_jumping = true;
            jump_state.jump_timer = 0.0;
            jump_state.jump_buffer_timer = 0.0;
        }

        if jump_state.is_jumping && actions.jump.pressed {
            jump_state.jump_timer += dt;
            if jump_state.jump_timer < jump_state.max_jump_duration {
                velocity.y = PLAYER_CONFIG.jump_force;
            } else {
                jump_state.is_jumping = false;
            }
        } else if jump_state.is_jumping {
            jump_state.is_jumping = false;
        }

        if grounded.is_grounded && velocity.y <= 0.0 {
            jump_state.is_jumping = false;
            jump_state.jump_timer = 0.0;
        }
    }
}
