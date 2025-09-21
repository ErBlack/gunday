use super::robot_components::{EnemyRobot, EnemyRobotState, EnemyRobotStateKind};
use crate::assets::GameAssets;
use crate::enemy_c::components::{EnemyCBundle, EnemyCSpawnPause};
use crate::spawn::SpawnedFromEdge;
use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Resource, Debug, Clone)]
pub struct InfectedEnemyConfig {
    pub transform_frame_times: [f32; 4],
    pub spawn_pause_duration: f32,
}

impl Default for InfectedEnemyConfig {
    fn default() -> Self {
        Self {
            transform_frame_times: [0.6, 0.4, 0.1, 0.1],
            spawn_pause_duration: 0.3,
        }
    }
}

#[derive(Component)]
pub struct InfectedEnemyRobot;

#[derive(Component)]
pub struct InfectedTransformAnim {
    pub timer: f32,
    pub frame: usize,
    pub frame_time: f32,
    pub frame_durations: [f32; 4],
    pub baseline_y: f32,
    pub facing_right: bool,
}

impl InfectedTransformAnim {
    pub fn new(baseline_y: f32, facing_right: bool, frame_durations: [f32; 4]) -> Self {
        let initial_time = frame_durations.first().copied().unwrap_or(0.0);
        Self {
            timer: 0.0,
            frame: 0,
            frame_time: initial_time,
            frame_durations,
            baseline_y,
            facing_right,
        }
    }
}

pub fn infected_transform_system(
    time: Res<Time>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    infected_config: Res<InfectedEnemyConfig>,
    mut query: Query<
        (
            Entity,
            &mut Sprite,
            &mut Transform,
            &mut InfectedTransformAnim,
            &EnemyRobotState,
            Option<&SpawnedFromEdge>,
        ),
        With<EnemyRobot>,
    >,
) {
    if query.is_empty() {
        return;
    }

    let infected_config = infected_config.as_ref();

    for (entity, mut sprite, mut transform, mut anim, state, edge_marker) in query.iter_mut() {
        if state.state != EnemyRobotStateKind::Hit {
            continue;
        }

        sprite.flip_x = anim.facing_right;
        sprite.anchor = Anchor::BottomCenter;
        transform.translation.y = anim.baseline_y;

        let frames = [
            assets.enemy_c_transform_a.clone(),
            assets.enemy_c_transform_b.clone(),
            assets.enemy_c_transform_c.clone(),
            assets.enemy_c_transform_d.clone(),
        ];

        if anim.frame >= frames.len() {
            let spawn_translation = Vec3::new(
                transform.translation.x,
                anim.baseline_y,
                transform.translation.z,
            );

            commands.entity(entity).despawn();
            let mut spawned = commands.spawn((
                EnemyCBundle::new(
                    assets.enemy_c_run_a.clone(),
                    spawn_translation,
                    anim.facing_right,
                ),
                Name::new("EnemyC"),
            ));
            if let Some(edge) = edge_marker {
                spawned.insert(SpawnedFromEdge { edge: edge.edge });
            }
            spawned.insert(EnemyCSpawnPause::new(infected_config.spawn_pause_duration));
            continue;
        }

        if sprite.image != frames[anim.frame] {
            sprite.image = frames[anim.frame].clone();
        }

        anim.timer += time.delta_secs();
        if anim.timer >= anim.frame_time {
            anim.timer -= anim.frame_time;
            anim.frame += 1;
            let next_index = anim.frame.min(anim.frame_durations.len().saturating_sub(1));
            anim.frame_time = anim.frame_durations[next_index];
        }
    }
}
