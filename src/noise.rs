use opensimplex2::fast::noise2;

pub(crate) fn simplex2d(
    seed: i64,
    size: usize,
    frequency: f64,
    amplitude: f32,
) -> impl Fn(usize, usize) -> f32 {
    let scaler = frequency / size as f64;
    let amplitude = amplitude / 2.0;

    move |x, y| (noise2(seed, x as f64 * scaler, y as f64 * scaler) + 1.0) * amplitude
}

pub(crate) fn simplex2d_octaves(
    seed: i64,
    size: usize,
    frequency: f64,
    amplitude: f32,
    octaves: u32,
    persistance: f32,
) -> impl Fn(usize, usize) -> f32 {
    let octaves = octaves as i32;
    let scaler = 1.0 / size as f64;
    let max_amplitude: f32 = (0..octaves).map(|o| amplitude * persistance.powi(o)).sum();
    println!("{}", max_amplitude);

    move |x, y| {
        let x = x as f64 * scaler;
        let y = y as f64 * scaler;
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
