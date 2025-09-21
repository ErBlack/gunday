use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct BossSettings {
    pub width: f32,
    pub collider_half_size: Vec2,
    pub spawn_move_duration: f32,
    pub initial_waving_amplitude: f32,
    pub stage1: Stage1Settings,
    pub stage2: Stage2Settings,
    pub transition: TransitionSettings,
    pub arms: ArmSettings,
    pub torso: TorsoSettings,
    pub head: HeadSettings,
    pub spine: SpineSettings,
    pub cannon: CannonSettings,
    pub explosion: ExplosionSettings,
}

#[derive(Debug, Clone, Copy)]
pub struct Stage1Settings {
    pub spine_hp: u8,
    pub hover_ground_offset: f32,
    pub wave_frequency: f32,
    pub amplitude_lerp_speed: f32,
    pub amplitude_snap_threshold: f32,
    pub idle_target_amplitude: f32,
    pub movement: Stage1MovementSettings,
    pub shooting: Stage1ShootingSettings,
}

#[derive(Debug, Clone, Copy)]
pub struct Stage1MovementSettings {
    pub move_duration: f32,
    pub anchor_side_margin: f32,
    pub anchor_top_margin: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Stage1ShootingSettings {
    pub initial_shoot_timer: f32,
    pub initial_aim_timer: f32,
    pub aim_duration: f32,
    pub shoot_cooldown: f32,
    pub inter_shot_delay: f32,
    pub shots_per_burst: u8,
    pub tip_distance: f32,
    pub muzzle_vertical_offset: f32,
    pub aim_cooldown_hit_penalty: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Stage2Settings {
    pub crawl_speed: f32,
    pub head_hp: u8,
    pub arm_swing_amplitude: f32,
    pub arm_swing_frequency: f32,
    pub arm_base_shift_deg: f32,
    pub left_arm_base_deg: f32,
    pub right_arm_base_deg: f32,
    pub right_arm_phase_offset_deg: f32,
    pub spine_base_deg: f32,
    pub spine_twitch_amplitude: f32,
    pub spine_twitch_frequency: f32,
    pub spine_phase_offset: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct TransitionSettings {
    pub edge_padding: f32,
    pub initial_velocity: Vec2,
    pub gravity_scale: f32,
    pub ground_offset: f32,
    pub downed_wait: f32,
    pub explosion_burst_count: u8,
    pub explosion_interval: f32,
    pub cannon_launch_velocity: Vec2,
    pub cannon_angular_velocity: f32,
    pub cannon_gravity_scale: f32,
    pub cannon_ground_snap_offset: f32,
    pub cannon_upright_trigger_height: f32,
    pub cannon_settle_rotation_speed: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct ArmSettings {
    pub relaxed_left_deg: f32,
    pub relaxed_right_deg: f32,
    pub aiming_left_deg: f32,
    pub aiming_right_deg: f32,
    pub shooting_base_left_deg: f32,
    pub shooting_base_right_deg: f32,
    pub shooting_oscillation_amplitude_deg: f32,
    pub shooting_oscillation_frequency: f32,
    pub idle_oscillation_amplitude_deg: f32,
    pub idle_oscillation_frequency: f32,
    pub rotation_speed: f32,
    pub transition_stretched_angle_deg: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct TorsoSettings {
    pub oscillation_amplitude: f32,
    pub oscillation_frequency: f32,
    pub stage2_base_rotation_deg: f32,
    pub stage2_base_offset: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct HeadSettings {
    pub rotation_speed: f32,
    pub rotation_range_deg: f32,
    pub neutral_angle_deg: f32,
    pub downed_angle_deg: f32,
    pub size: Vec2,
    pub explosion_offset_y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct SpineSettings {
    pub size: Vec2,
    pub hit_animation_duration: f32,
    pub hit_rotation_cycles: f32,
    pub hit_rotation_amplitude_deg: f32,
    pub hit_flash_toggle_hz: f32,
    pub explosion_offset_y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct CannonSettings {
    pub max_rotation_speed: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct ExplosionSettings {
    pub win_start_delay: f32,
    pub win_exit_delay: f32,
    pub part_gravity_scale: f32,
    pub part_launch: ExplosionLaunchSettings,
}

#[derive(Debug, Clone, Copy)]
pub struct ExplosionLaunchSettings {
    pub head: PartLaunchSettings,
    pub left_arm: PartLaunchSettings,
    pub right_arm: PartLaunchSettings,
    pub spine: PartLaunchSettings,
    pub torso: PartLaunchSettings,
    pub cannon: PartLaunchSettings,
}

#[derive(Debug, Clone, Copy)]
pub struct PartLaunchSettings {
    pub velocity: Vec2,
    pub angular_velocity: f32,
}

pub const BOSS_SETTINGS: BossSettings = BossSettings {
    width: 220.0,
    collider_half_size: Vec2::new(44.0, 40.0),
    spawn_move_duration: 3.0,
    initial_waving_amplitude: 2.0,
    stage1: Stage1Settings {
        spine_hp: 80,
        hover_ground_offset: 86.0,
        wave_frequency: 0.6,
        amplitude_lerp_speed: 3.0,
        amplitude_snap_threshold: 0.01,
        idle_target_amplitude: 3.0,
        movement: Stage1MovementSettings {
            move_duration: 2.0,
            anchor_side_margin: 100.0,
            anchor_top_margin: 150.0,
        },
        shooting: Stage1ShootingSettings {
            initial_shoot_timer: 4.5,
            initial_aim_timer: 2.0,
            aim_duration: 1.0,
            shoot_cooldown: 2.0,
            inter_shot_delay: 0.067,
            shots_per_burst: 25,
            tip_distance: 150.0,
            muzzle_vertical_offset: -4.0,
            aim_cooldown_hit_penalty: 0.35,
        },
    },
    stage2: Stage2Settings {
        crawl_speed: 120.0,
        head_hp: 50,
        arm_swing_amplitude: 0.5,
        arm_swing_frequency: 1.0,
        arm_base_shift_deg: 15.0,
        left_arm_base_deg: -131.8,
        right_arm_base_deg: -45.8,
        right_arm_phase_offset_deg: 180.0,
        spine_base_deg: -68.8,
        spine_twitch_amplitude: 0.1,
        spine_twitch_frequency: 4.0,
        spine_phase_offset: 0.13,
    },
    transition: TransitionSettings {
        edge_padding: 80.0,
        initial_velocity: Vec2::new(520.0, 220.0),
        gravity_scale: 0.45,
        ground_offset: 22.0,
        downed_wait: 3.0,
        explosion_burst_count: 3,
        explosion_interval: 0.25,
        cannon_launch_velocity: Vec2::new(360.0, 300.0),
        cannon_angular_velocity: 6.0,
        cannon_gravity_scale: 1.1,
        cannon_ground_snap_offset: -8.0,
        cannon_upright_trigger_height: 50.0,
        cannon_settle_rotation_speed: 6.0,
    },
    arms: ArmSettings {
        relaxed_left_deg: -12.0,
        relaxed_right_deg: 22.0,
        aiming_left_deg: 8.0,
        aiming_right_deg: 2.0,
        shooting_base_left_deg: 28.0,
        shooting_base_right_deg: -18.0,
        shooting_oscillation_amplitude_deg: 1.0,
        shooting_oscillation_frequency: 10.0,
        idle_oscillation_amplitude_deg: 2.0,
        idle_oscillation_frequency: 0.6,
        rotation_speed: 2.0,
        transition_stretched_angle_deg: 0.0,
    },
    torso: TorsoSettings {
        oscillation_amplitude: 1.5,
        oscillation_frequency: 0.6,
        stage2_base_rotation_deg: 78.0,
        stage2_base_offset: Vec2::new(6.0, -2.0),
    },
    head: HeadSettings {
        rotation_speed: 1.5,
        rotation_range_deg: 15.0,
        neutral_angle_deg: -8.0,
        downed_angle_deg: -80.0,
        size: Vec2::new(48.0, 42.0),
        explosion_offset_y: 24.0,
    },
    spine: SpineSettings {
        size: Vec2::new(30.0, 48.0),
        hit_animation_duration: 0.3,
        hit_rotation_cycles: 2.0,
        hit_rotation_amplitude_deg: 1.0,
        hit_flash_toggle_hz: 20.0,
        explosion_offset_y: 24.0,
    },
    cannon: CannonSettings {
        max_rotation_speed: 2.0,
    },
    explosion: ExplosionSettings {
        win_start_delay: 3.6,
        win_exit_delay: 4.22,
        part_gravity_scale: 0.9,
        part_launch: ExplosionLaunchSettings {
            head: PartLaunchSettings {
                velocity: Vec2::new(-520.0, 720.0),
                angular_velocity: 9.0,
            },
            left_arm: PartLaunchSettings {
                velocity: Vec2::new(-680.0, 840.0),
                angular_velocity: -8.0,
            },
            right_arm: PartLaunchSettings {
                velocity: Vec2::new(680.0, 840.0),
                angular_velocity: 8.0,
            },
            spine: PartLaunchSettings {
                velocity: Vec2::new(-560.0, 780.0),
                angular_velocity: -7.0,
            },
            torso: PartLaunchSettings {
                velocity: Vec2::new(480.0, 660.0),
                angular_velocity: 6.0,
            },
            cannon: PartLaunchSettings {
                velocity: Vec2::new(820.0, 620.0),
                angular_velocity: -10.0,
            },
        },
    },
};
