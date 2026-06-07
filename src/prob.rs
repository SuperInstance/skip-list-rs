#![allow(dead_code)]
//! Probability utilities for skip list level generation.

/// Generate a random level using a simple LCG PRNG seeded from the key.
///
/// Probability of level `l` is `p^l` where `p = 0.5`.
/// Maximum level is capped at `max_level`.
pub fn random_level(key: i64, max_level: usize) -> usize {
    // Simple LCG PRNG from key
    let mut state = (key as u64).wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    let mut level = 1;
    for _ in 1..max_level {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        if state.is_multiple_of(2) {
            level += 1;
        } else {
            break;
        }
    }
    level
}

/// Expected level for a given probability `p` and max level.
pub fn expected_level(p: f64, max_level: usize) -> f64 {
    let mut sum = 0.0;
    for l in 1..=max_level {
        sum += l as f64 * p.powi(l as i32 - 1) * (1.0 - p);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_in_range() {
        for key in 0..1000i64 {
            let level = random_level(key, 16);
            assert!(level >= 1 && level <= 16, "level={level} for key={key}");
        }
    }

    #[test]
    fn deterministic() {
        assert_eq!(random_level(42, 16), random_level(42, 16));
    }

    #[test]
    fn different_keys_likely_different() {
        let mut same = 0;
        for key in 0..1000 {
            if random_level(key, 16) == random_level(key + 1, 16) {
                same += 1;
            }
        }
        // Unlikely that all 1000 pairs get the same level
        assert!(same < 999);
    }

    #[test]
    fn expected_level_reasonable() {
        let e = expected_level(0.5, 16);
        assert!(e > 1.0 && e < 3.0, "expected level = {e}");
    }
}
