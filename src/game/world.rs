extern crate sfml;
extern crate rand;
extern crate num;

use sfml::system::{Vector2f};
use rand::{thread_rng, Rng};
use num::Complex;
use std::f32::consts::PI;

fn random_complex(rng: &mut rand::prelude::ThreadRng) -> Complex<f32> {
    let r : f32 = rng.gen_range(0.0, 1.0);
    let t = rng.gen_range(0.0, 2.0 * PI);
    return Complex::new(r.sqrt() * t.cos(), r.sqrt() * t.sin());
}

pub struct World {
    pub surface: Vec<Vector2f>
}

impl World {
    pub fn new() -> Self {
        // Generate a random world
        let mut rng = thread_rng();
        let num_segments = 180;
        let mut surface_pts = Vec::with_capacity(num_segments);

        let fourier_coeff : Vec<Complex<f32>> = (1..18).map(|_| random_complex(&mut rng) ).collect();
        for n in 0..num_segments {
            let theta = 2.0 * PI * (n as f32) / (num_segments as f32);
            let z = (theta * Complex::i()).exp();
            let val = fourier_coeff.iter().enumerate().map(|(k, c)| c * z.powf(k as f32)).sum::<Complex<f32>>().norm();
            let radius = 20.0 + 1.0 * val;
            surface_pts.push(Vector2f::new(radius * theta.cos(), radius * theta.sin()));
        }

        return World { surface: surface_pts };
    }
}

