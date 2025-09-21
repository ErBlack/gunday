#[inline]
pub fn shortest_angle_diff(target: f32, current: f32) -> f32 {
    let diff = (target - current).rem_euclid(std::f32::consts::TAU);
    if diff > std::f32::consts::PI {
        diff - std::f32::consts::TAU
    } else {
        diff
    }
}

#[inline]
pub fn approach_angle(current: f32, target: f32, max_delta: f32) -> f32 {
    let delta = shortest_angle_diff(target, current);
    let clamped = delta.clamp(-max_delta, max_delta);
    current + clamped
}
