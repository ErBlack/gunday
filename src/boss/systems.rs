use super::components::*;
use super::events::*;
use bevy::prelude::*;

pub use super::boss_absorb_player_projectiles_system::boss_absorb_player_projectiles_system;
pub use super::boss_arm_animation_system::{
    boss_arm_animation_system, boss_arm_transition_pose_system,
};
pub use super::boss_explosion_system::boss_explosion_system;
pub use super::boss_explosion_system::detached_cannon_system;
pub use super::boss_explosion_system::exploding_part_system;
pub use super::boss_head_animation_system::boss_head_animation_system;
pub use super::boss_head_hit_system::boss_head_hit_system;
pub use super::boss_spine_hit_animation_system::boss_spine_hit_animation_system;
pub use super::boss_spine_hit_system::boss_spine_hit_system;
pub use super::boss_sprite_flip_system::boss_sprite_flip_system;
pub use super::boss_stage1_movement_system::boss_stage1_movement_system;
pub use super::boss_stage1_shooting_system::boss_stage1_shooting_system;
pub use super::boss_stage2_animation_system::boss_stage2_animation_system;
pub use super::boss_stage2_movement_system::boss_stage2_movement_system;
pub use super::boss_torso_animation_system::{
    boss_torso_animation_system, boss_torso_downed_pose_system,
};
pub use super::boss_transition_to_stage2_system::{
    boss_cannon_detacher_system, boss_transition_explosion_queue_system,
    boss_transition_to_stage2_start_system, boss_transition_to_stage2_update_system,
};
use crate::components::{LayerGeometry, Solid};
use crate::constants::SCREEN_HEIGHT;
use crate::systems::WinMusic;

pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BossStageTransitionEvent>()
            .add_event::<BossDefeatedEvent>()
            .add_systems(Startup, super::setup_boss::load_boss_audio)
            .add_systems(Update, super::spawn_system::boss_spawn_system)
            .add_systems(
                Update,
                (
                    boss_stage1_movement_system
                        .run_if(boss_in_stage(BossStageKind::Stage1)),
                    boss_collision_with_solids_system.after(boss_stage1_movement_system),
                    boss_arm_animation_system
                        .after(boss_stage1_movement_system)
                        .run_if(boss_in_stage(BossStageKind::Stage1)),
                    boss_torso_animation_system
                        .after(boss_arm_animation_system)
                        .run_if(boss_in_stage(BossStageKind::Stage1)),
                    boss_stage1_shooting_system
                        .after(boss_torso_animation_system)
                        .run_if(boss_in_stage(BossStageKind::Stage1)),
                    boss_sprite_flip_system.after(boss_stage1_shooting_system),
                ),
            )
            .add_systems(
                Update,
                (
                    boss_arm_transition_pose_system
                        .run_if(boss_in_stage(BossStageKind::TransitionToStage2)),
                    boss_torso_downed_pose_system.run_if(boss_in_stages(&[
                        BossStageKind::TransitionToStage2,
                        BossStageKind::Stage2,
                    ])),
                    boss_head_animation_system
                        .after(boss_torso_animation_system)
                        .after(boss_torso_downed_pose_system),
                    boss_spine_hit_system.run_if(boss_in_stage(BossStageKind::Stage1)),
                    boss_spine_hit_animation_system
                        .after(boss_spine_hit_system)
                        .run_if(boss_in_stages(&[
                            BossStageKind::Stage1,
                            BossStageKind::TransitionToStage2,
                        ])),
                    boss_transition_to_stage2_start_system.after(boss_spine_hit_system),
                    boss_cannon_detacher_system.after(boss_transition_to_stage2_start_system),
                    boss_transition_to_stage2_update_system
                        .after(boss_transition_to_stage2_start_system),
                    boss_transition_explosion_queue_system
                        .after(boss_transition_to_stage2_start_system)
                        .run_if(boss_in_stage(BossStageKind::TransitionToStage2)),
                ),
            )
            .add_systems(
                Update,
                (
                    boss_stage2_movement_system.run_if(boss_in_stage(BossStageKind::Stage2)),
                    boss_stage2_animation_system
                        .after(boss_stage2_movement_system)
                        .run_if(boss_in_stage(BossStageKind::Stage2)),
                    boss_head_hit_system.run_if(boss_in_stage(BossStageKind::Stage2)),
                    boss_absorb_player_projectiles_system
                        .after(boss_spine_hit_system)
                        .after(boss_head_hit_system),
                    boss_explosion_system.run_if(boss_in_stage(BossStageKind::Exploding)),
                    boss_win_music_signal_system.after(boss_explosion_system),
                    exploding_part_system.after(boss_explosion_system),
                    detached_cannon_system.after(boss_transition_to_stage2_update_system),
                ),
            );
    }
}

pub fn boss_collision_with_solids_system(
    mut q: Query<
        (
            &mut Transform,
            &super::components::BossCollider,
            &super::components::BossStage,
        ),
        With<super::components::Boss>,
    >,
    solids: Query<&LayerGeometry, With<Solid>>,
) {
    for (mut tf, col, stage) in q.iter_mut() {
        if stage.0 == super::components::BossStageKind::TransitionToStage2 {
            continue;
        }

        let pos = tf.translation;
        let mut clamped = Vec3::new(pos.x, pos.y, pos.z);
        let boss_world_min = Vec2::new(
            pos.x - col.half_size.x,
            pos.y + SCREEN_HEIGHT * 0.5 - col.half_size.y,
        );
        let boss_world_max = Vec2::new(
            pos.x + col.half_size.x,
            pos.y + SCREEN_HEIGHT * 0.5 + col.half_size.y,
        );
        for g in solids.iter() {
            let s_min = g.bottom_left;
            let s_max = g.bottom_left + Vec2::new(g.width, g.height);
            let overlap_x = (boss_world_max.x - s_min.x).min(s_max.x - boss_world_min.x);
            let overlap_y = (boss_world_max.y - s_min.y).min(s_max.y - boss_world_min.y);
            let intersecting = boss_world_min.x < s_max.x
                && boss_world_max.x > s_min.x
                && boss_world_min.y < s_max.y
                && boss_world_max.y > s_min.y;
            if intersecting {
                if overlap_x < overlap_y {
                    if (pos.x) < (s_min.x + s_max.x) * 0.5 {
                        clamped.x -= overlap_x;
                    } else {
                        clamped.x += overlap_x;
                    }
                } else {
                    if (pos.y + SCREEN_HEIGHT * 0.5) < (s_min.y + s_max.y) * 0.5 {
                        clamped.y -= overlap_y;
                    } else {
                        clamped.y += overlap_y;
                    }
                }
            }
        }
        tf.translation = clamped;
    }
}

pub fn boss_win_music_signal_system(
    q: Query<&super::components::BossExplodingState, With<super::components::Boss>>,
    mut win: ResMut<WinMusic>,
) {
    if win.0 {
        return;
    }
    for ex in q.iter() {
        if ex.win_started {
            win.0 = true;
            break;
        }
    }
}

fn boss_in_stage(
    stage: BossStageKind,
) -> impl FnMut(Query<&BossStage, With<Boss>>) -> bool + Clone {
    move |q: Query<&BossStage, With<Boss>>| q.iter().next().map(|s| s.0 == stage).unwrap_or(false)
}

fn boss_in_stages(
    stages: &'static [BossStageKind],
) -> impl FnMut(Query<&BossStage, With<Boss>>) -> bool + Clone {
    move |q: Query<&BossStage, With<Boss>>| {
        q.iter()
            .next()
            .map(|s| stages.contains(&s.0))
            .unwrap_or(false)
    }
}
