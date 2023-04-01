use opensimplex2::fast::noise2;

use crate::map::Pos;

// pub(crate) fn simplex2d(
//     seed: i64,
//     size: usize,
//     frequency: f64,
//     amplitude: f32,
// ) -> impl Fn(Pos) -> f32 {
//     let scaler = frequency / size as f64;
//     let amplitude = amplitude / 2.0;

//     move |pos| (noise2(seed, pos.x as f64 * scaler, pos.y as f64 * scaler) + 1.0) * amplitude
// }

pub(crate) fn simplex2d_octaves(
    seed: i64,
    size: usize,
    frequency: f64,
    amplitude: f32,
    octaves: u32,
    persistance: f32,
) -> impl Fn(Pos) -> f32 {
    let octaves = octaves as i32;
    let scaler = 1.0 / size as f64;
    let max_amplitude: f32 = (0..octaves).map(|o| amplitude * persistance.powi(o)).sum();
    println!("{}", max_amplitude);

    move |pos| {
        let x = pos.x as f64 * scaler;
        let y = pos.y as f64 * scaler;
        let mut total = 0.;
        let mut current_amplitude = amplitude;
        let mut frequency = frequency;

        for _ in 0..octaves {
            let noise = noise2(seed, x * frequency, y * frequency);
            total += (noise * 0.5 + 0.5) * current_amplitude;
            frequency *= 2.0;
            current_amplitude *= persistance;
        }

        total / max_amplitude * amplitude
    }
}
