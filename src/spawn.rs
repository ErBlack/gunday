use crate::assets::GameAssets;
use crate::components::MainCamera;
use crate::constants::{GROUND_RECT_HEIGHT, SCREEN_HEIGHT, SCREEN_WIDTH, Z_ENEMY_BASE};
use crate::enemy_a::{
    ENEMY_A_CONFIG, InfectedEnemyRobot,
    robot_components::{
        ENEMY_ROBOT_HEIGHT, ENEMY_ROBOT_WIDTH, EnemyRobotBundle, EnemySpawnProtection,
    },
};
use crate::enemy_b::components::{
    ENEMY_B_HEIGHT, ENEMY_B_WIDTH, EnemyB, EnemyBBundle, EnemyBSpawnProtection,
};
use crate::enemy_c::components::{ENEMY_C_WIDTH, EnemyC};
use crate::game_state::{GamePhase, GameState};
use crate::player::components::Player;
use bevy::prelude::*;

const HANGAR_ENEMY_SPAWN_INTERVAL: f32 = 3.0;

#[derive(Clone, Copy)]
pub struct StaticSpawnCoordinates {
    pub grenade_thrower_1: f32,
    pub grenade_thrower_2: f32,
    pub grenade_thrower_3: f32,
    pub infected_enemy: f32,
}

pub const STATIC_SPAWN_COORDINATES: StaticSpawnCoordinates = StaticSpawnCoordinates {
    grenade_thrower_1: 3487.0,
    grenade_thrower_2: 4003.0,
    grenade_thrower_3: 4468.0,
    infected_enemy: 6680.0,
};

#[derive(Resource, Default)]
pub struct EdgeSpawnManager {
    pub definitions: Vec<EdgeSpawnDefinition>,
}

#[derive(Clone)]
pub struct EdgeSpawnDefinition {
    pub spawn_x: f32,
    pub spawn_position: Vec3,
    pub width: f32,
    pub edge: ScreenEdge,
    pub action: EdgeSpawnAction,
    pub spawned: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenEdge {
    Left,
    Right,
}

#[derive(Component, Clone, Copy)]
pub struct SpawnedFromEdge {
    pub edge: ScreenEdge,
}

#[derive(Clone)]
pub enum EdgeSpawnAction {
    EnemyB,
    InfectedEnemyA,
}

impl EdgeSpawnDefinition {
    pub fn enemy_b(position: Vec2) -> Self {
        Self {
            spawn_x: position.x,
            spawn_position: Vec3::new(position.x, position.y, Z_ENEMY_BASE),
            width: ENEMY_B_WIDTH,
            edge: ScreenEdge::Right,
            action: EdgeSpawnAction::EnemyB,
            spawned: false,
        }
    }

    pub fn infected_enemy_robot(spawn_x: f32, edge: ScreenEdge) -> Self {
        let y = GROUND_RECT_HEIGHT - SCREEN_HEIGHT * 0.5
            + ENEMY_ROBOT_HEIGHT * 0.5
            + ENEMY_A_CONFIG.spawn_ground_offset;
        Self {
            spawn_x,
            spawn_position: Vec3::new(spawn_x, y, Z_ENEMY_BASE),
            width: ENEMY_ROBOT_WIDTH,
            edge,
            action: EdgeSpawnAction::InfectedEnemyA,
            spawned: false,
        }
    }

}

impl EdgeSpawnAction {
    fn spawn(
        &self,
        commands: &mut Commands,
        game_assets: &GameAssets,
        spawn_position: Vec3,
        spawn_x: f32,
        player_x: Option<f32>,
        edge: ScreenEdge,
    ) {
        match self {
            EdgeSpawnAction::InfectedEnemyA => {
                let facing_right = player_x.map(|px| px > spawn_x).unwrap_or(false);
                commands
                    .spawn((
                        EnemyRobotBundle::new(
                            game_assets.enemy_a_run_a.clone(),
                            spawn_position,
                            facing_right,
                        ),
                        EnemySpawnProtection::default(),
                        Name::new("InfectedEnemyA"),
                        SpawnedFromEdge { edge },
                    ))
                    .insert(InfectedEnemyRobot);
            }
            EdgeSpawnAction::EnemyB => {
                let facing_right = player_x.map(|px| px >= spawn_position.x).unwrap_or(true);
                commands.spawn((
                    EnemyBBundle::new(
                        game_assets.enemy_b_sit.clone(),
                        spawn_position,
                        facing_right,
                    ),
                    EnemyBSpawnProtection::new(),
                    Name::new("EnemyB"),
                    SpawnedFromEdge { edge },
                ));
            }
        }
    }
}

pub fn configure_default_spawns(mut manager: ResMut<EdgeSpawnManager>) {
    if !manager.definitions.is_empty() {
        return;
    }

    let coords = STATIC_SPAWN_COORDINATES;
    let mut definitions = Vec::with_capacity(4);

    definitions.push(EdgeSpawnDefinition::infected_enemy_robot(
        coords.infected_enemy,
        ScreenEdge::Right,
    ));

    let platform_y = 342.0 - SCREEN_HEIGHT * 0.5 + ENEMY_B_HEIGHT * 0.5;
    for x in [
        coords.grenade_thrower_1,
        coords.grenade_thrower_2,
        coords.grenade_thrower_3,
    ] {
        definitions.push(EdgeSpawnDefinition::enemy_b(Vec2::new(x, platform_y)));
    }
    manager.definitions = definitions;
}

pub fn hangar_enemy_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    game_state: Res<GameState>,
    camera_q: Query<&Transform, With<MainCamera>>,
    player_q: Query<&Transform, With<Player>>,
    game_assets: Res<GameAssets>,
    mut timer: Local<Option<Timer>>,
) {
    if game_state.phase() != GamePhase::HangarFight {
        if let Some(existing) = timer.as_mut() {
            existing.pause();
            existing.reset();
        }
        return;
    }

    let Some(camera_tf) = camera_q.iter().next() else {
        return;
    };

    let timer = timer.get_or_insert_with(|| {
        let mut t = Timer::from_seconds(HANGAR_ENEMY_SPAWN_INTERVAL, TimerMode::Repeating);
        t.pause();
        t
    });

    if timer.paused() {
        timer.unpause();
    }

    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    let spawn_x = camera_tf.translation.x + SCREEN_WIDTH * 0.5 + ENEMY_ROBOT_WIDTH * 0.5;
    let spawn_y = GROUND_RECT_HEIGHT - SCREEN_HEIGHT * 0.5
        + ENEMY_ROBOT_HEIGHT * 0.5
        + ENEMY_A_CONFIG.spawn_ground_offset;
    let spawn_position = Vec3::new(spawn_x, spawn_y, Z_ENEMY_BASE);
    let player_x = player_q.iter().next().map(|tf| tf.translation.x);
    let facing_right = player_x.map(|px| px > spawn_x).unwrap_or(false);

    commands.spawn((
        EnemyRobotBundle::new(
            game_assets.enemy_a_run_a.clone(),
            spawn_position,
            facing_right,
        ),
        EnemySpawnProtection::default(),
        Name::new("EnemyA"),
        SpawnedFromEdge {
            edge: ScreenEdge::Right,
        },
    ));
}

pub fn edge_spawn_system(
    mut commands: Commands,
    mut manager: ResMut<EdgeSpawnManager>,
    camera_q: Query<&Transform, With<MainCamera>>,
    player_q: Query<&Transform, With<Player>>,
    game_assets: Res<GameAssets>,
    mut prev_cam_x: Local<Option<f32>>,
) {
    let Some(cam_tf) = camera_q.iter().next() else {
        return;
    };

    let cam_x = cam_tf.translation.x;
    let prev_center = prev_cam_x.unwrap_or(cam_x);
    let half_w = SCREEN_WIDTH * 0.5;

    let prev_right = prev_center + half_w;
    let curr_right = cam_x + half_w;
    let prev_left = prev_center - half_w;
    let curr_left = cam_x - half_w;

    let player_x = player_q.iter().next().map(|tf| tf.translation.x);
    for def in manager.definitions.iter_mut() {
        if def.spawned {
            continue;
        }
        let target = match def.edge {
            ScreenEdge::Right => def.spawn_x - def.width * 0.5,
            ScreenEdge::Left => def.spawn_x + def.width * 0.5,
        };
        let crossed = match def.edge {
            ScreenEdge::Right => prev_right < target && curr_right >= target,
            ScreenEdge::Left => prev_left < target && curr_left >= target,
        };

        if crossed {
            def.spawned = true;
            def.action.spawn(
                &mut commands,
                game_assets.as_ref(),
                def.spawn_position,
                def.spawn_x,
                player_x,
                def.edge,
            );
        }
    }

    *prev_cam_x = Some(cam_x);
}

pub fn enemy_edge_cleanup_system(
    mut commands: Commands,
    camera_q: Query<&Transform, With<MainCamera>>,
    query: Query<(
        Entity,
        &Transform,
        &SpawnedFromEdge,
        Option<&crate::enemy_a::robot_components::EnemyRobot>,
        Option<&EnemyB>,
        Option<&EnemyC>,
    )>,
) {
    let Some(camera_tf) = camera_q.iter().next() else {
        return;
    };

    let cam_x = camera_tf.translation.x;
    let half_width = SCREEN_WIDTH * 0.5;
    let left_edge = cam_x - half_width;
    let right_edge = cam_x + half_width;

    for (entity, transform, spawned_edge, enemy_a, enemy_b, enemy_c) in query.iter() {
        let width = if enemy_a.is_some() {
            ENEMY_ROBOT_WIDTH
        } else if enemy_b.is_some() {
            ENEMY_B_WIDTH
        } else if enemy_c.is_some() {
            ENEMY_C_WIDTH
        } else {
            continue;
        };

        match spawned_edge.edge {
            ScreenEdge::Left => {
                if transform.translation.x - width * 0.5 > right_edge {
                    commands.entity(entity).despawn();
                }
            }
            ScreenEdge::Right => {
                if transform.translation.x + width * 0.5 < left_edge {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
