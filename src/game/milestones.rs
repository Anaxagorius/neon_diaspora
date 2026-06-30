/// Shared milestone scaling: every N purchases, per-unit output gains +1× base power.
pub const MILESTONE_INTERVAL: u32 = 25;

pub fn milestone_tier(owned: u32) -> u32 {
    if owned == 0 {
        0
    } else {
        1 + owned / MILESTONE_INTERVAL
    }
}

pub fn per_unit_power(base_power: f64, owned: u32) -> f64 {
    if owned == 0 {
        0.0
    } else {
        base_power * milestone_tier(owned) as f64
    }
}

pub fn total_output(base_power: f64, owned: u32, multiplier: f64) -> f64 {
    if owned == 0 {
        0.0
    } else {
        per_unit_power(base_power, owned) * owned as f64 * multiplier
    }
}

pub fn next_milestone_at(owned: u32) -> u32 {
    if owned == 0 {
        MILESTONE_INTERVAL
    } else {
        ((owned / MILESTONE_INTERVAL) + 1) * MILESTONE_INTERVAL
    }
}