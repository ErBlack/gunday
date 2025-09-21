use bevy::prelude::*;

pub fn rectangles_collide(pos1: Vec2, size1: Vec2, pos2: Vec2, size2: Vec2) -> bool {
    pos1.x < pos2.x + size2.x
        && pos1.x + size1.x > pos2.x
        && pos1.y < pos2.y + size2.y
        && pos1.y + size1.y > pos2.y
}
