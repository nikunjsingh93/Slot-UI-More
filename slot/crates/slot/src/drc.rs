const MAX_CORRECTION: f64 = 0.005;

pub fn drc_ratio(queued_frames: usize, target_frames: usize) -> f64 {
    if target_frames == 0 {
        return 1.0;
    }
    let error = target_frames as f64 - queued_frames as f64;
    let ratio = 1.0 + MAX_CORRECTION * error / target_frames as f64;
    ratio.clamp(1.0 - MAX_CORRECTION, 1.0 + MAX_CORRECTION)
}

pub fn drc_target(capacity_frames: usize) -> usize {
    capacity_frames / 2
}
