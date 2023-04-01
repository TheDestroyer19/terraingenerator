use std::collections::VecDeque;

use crate::map::{Map, Pos};

const MAX_WATER_ELEVATION: f32 = 10000.0;

pub(crate) fn calculate_lakes(elevation: &Map<f32>, standing_water: &mut Map<f32>, ocean_level: f32) {
    fill_depressions(elevation, standing_water);

    standing_water.map(|pos, v| {
        let target_level = v.max(ocean_level);
        (target_level - *elevation.get(pos)).max(0.0)
    });
}

pub(crate) fn fill_depressions(elevation: &Map<f32>, standing_water: &mut Map<f32>) {
    let size = elevation.size();

    // first off start by setting the edges to their elevation, 
    // and other cells to a 'max lake depth'
    let generator = |pos: Pos| {
        if pos.x == 0 || pos.y == 0 || pos.x + 1 == size || pos.y + 1 == size {
            *elevation.get(pos)
        } else {
            MAX_WATER_ELEVATION
        }
    };
    if standing_water.size() != elevation.size() {
        *standing_water = Map::<f32>::with_generator(size, generator);
    } else {
        standing_water.map(|pos, _| generator(pos));
    }

    //Now setup cells to start evaluating from
    let mut queue = VecDeque::with_capacity(size * 4);
    queue.extend((0..size).map(|i| Pos::new(i, 0)));
    queue.extend((0..size).map(|i| Pos::new(i, size - 1)));
    queue.extend((0..size).map(|i| Pos::new(0, i)));
    queue.extend((0..size).map(|i| Pos::new(size - 1, i)));

    //and we start working in from them removing water that would 'run off the map'
    while let Some(pos) = queue.pop_front() {
        let elevation = *elevation.get(pos);
        let water_level = *standing_water.get(pos);
        let drainage_level = elevation.max(water_level);

        let neighbors = pos.neighbors(size, size);
        for neighbor in neighbors {
            let nwl = *standing_water.get(neighbor);

            if nwl > drainage_level {
                standing_water.set(neighbor, drainage_level);
                queue.push_back(neighbor);
            }
        }
    }

}