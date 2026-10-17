//! Reproduzierbare Zufallsquelle.
//!
//! SplitMix64: klein, schnell, auf allen Plattformen bitgenau gleich.
//! Jedes Teilsystem bekommt einen eigenen Strom, damit eine Änderung im
//! Wetter nicht die Zufallsfolge der Vegetation verschiebt.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Eigener Strom für ein Teilsystem, abgeleitet aus Welt-Seed und Name.
    pub fn stream(seed: u64, name: &str) -> Self {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for b in name.bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
        let mut r = Self::new(seed ^ h);
        r.next_u64();
        r
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// Gleichverteilt in [0, 1).
    pub fn next_f32(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 / (1u64 << 24) as f32
    }

    /// Gleichverteilt in [0, n).
    pub fn below(&mut self, n: u32) -> u32 {
        (((self.next_u64() >> 32) * n as u64) >> 32) as u32
    }

    pub fn chance(&mut self, p: f32) -> bool {
        self.next_f32() < p
    }
}

/// Zustandsloser Hash zweier Koordinaten mit Seed (für Rauschen).
pub fn hash2(x: i32, y: i32, seed: u64) -> u64 {
    let mut z = seed
        ^ (x as u32 as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ (y as u32 as u64).wrapping_mul(0xc2b2_ae3d_27d4_eb4f);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_sequence() {
        let mut a = Rng::stream(42, "wetter");
        let mut b = Rng::stream(42, "wetter");
        for _ in 0..1000 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn streams_differ() {
        let mut a = Rng::stream(42, "wetter");
        let mut b = Rng::stream(42, "vegetation");
        assert_ne!(a.next_u64(), b.next_u64());
    }

    #[test]
    fn f32_in_range() {
        let mut r = Rng::new(1);
        for _ in 0..10_000 {
            let v = r.next_f32();
            assert!((0.0..1.0).contains(&v));
        }
    }
}
