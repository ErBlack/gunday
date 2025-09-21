use bevy::prelude::*;

#[derive(Event, Debug, Clone, Copy)]
pub struct BossStageTransitionEvent;

#[derive(Event, Debug, Clone, Copy)]
pub struct BossDefeatedEvent;
