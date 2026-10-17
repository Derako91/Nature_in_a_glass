//! Kleine, plattformunabhängige Rechenhilfen.
//!
//! Im Kern werden bewusst nur Grundrechenarten und `sqrt` verwendet
//! (IEEE-754 garantiert dafür bitgleiche Ergebnisse). Winkel- und
//! Exponentialfunktionen der Standardbibliothek können sich zwischen
//! Plattformen minimal unterscheiden und würden den Determinismus brechen.

use crate::rng::hash2;

pub fn clamp01(v: f32) -> f32 {
    v.clamp(0.0, 1.0)
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn smoothstep(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// Jahreszeitkurve: −1 Mitte Januar (Winter), +1 Mitte Juli (Sommer).
/// Geglättete Dreieckswelle statt Sinus.
pub fn season(day_of_year: u32) -> f32 {
    let p = ((day_of_year + 365 - 15) % 365) as f32 / 365.0;
    let tri = 1.0 - 4.0 * (p - 0.5).abs();
    tri * (1.5 - 0.5 * tri * tri)
}

/// Gitterwert-Rauschen in [0, 1).
fn lattice(x: i32, y: i32, seed: u64) -> f32 {
    (hash2(x, y, seed) >> 40) as f32 / (1u64 << 24) as f32
}

/// Glattes Value-Noise in [0, 1).
pub fn value_noise(x: f32, y: f32, seed: u64) -> f32 {
    let x0 = x.floor();
    let y0 = y.floor();
    let tx = smoothstep(x - x0);
    let ty = smoothstep(y - y0);
    let (xi, yi) = (x0 as i32, y0 as i32);
    let a = lattice(xi, yi, seed);
    let b = lattice(xi + 1, yi, seed);
    let c = lattice(xi, yi + 1, seed);
    let d = lattice(xi + 1, yi + 1, seed);
    lerp(lerp(a, b, tx), lerp(c, d, tx), ty)
}

/// Mehrere Rauschoktaven überlagert, Ergebnis in [0, 1).
pub fn fbm(x: f32, y: f32, seed: u64, octaves: u32) -> f32 {
    let mut sum = 0.0;
    let mut amp = 1.0;
    let mut freq = 1.0;
    let mut norm = 0.0;
    for o in 0..octaves {
        sum += amp * value_noise(x * freq, y * freq, seed.wrapping_add(o as u64 * 7919));
        norm += amp;
        amp *= 0.5;
        freq *= 2.0;
    }
    sum / norm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn season_extremes() {
        assert!(season(15) < -0.99);
        assert!(season(197) > 0.99);
        for d in 0..365 {
            let s = season(d);
            assert!((-1.0..=1.0).contains(&s));
        }
    }

    #[test]
    fn noise_in_range() {
        for i in 0..1000 {
            let v = fbm(i as f32 * 0.37, i as f32 * 0.11, 7, 5);
            assert!((0.0..1.0).contains(&v));
        }
    }
}
