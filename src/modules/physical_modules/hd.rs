use cip_rust::GridParameters;
use std::collections::HashMap;
use grid::GridUnity;

pub fn calculations(
    initial_grid: HashMap<(u32, u32, u32), GridUnity>,
    size: Vec<u32>,
    time_step: f64,
    dx: f64,
) -> HashMap<(u32, u32, u32), GridUnity> {
    let grid_parameters_conservative: Vec<cip_rust::GridParameters>;
    for i in 0..(size[0] + 4) {
        let key = &(i,0,0);
        grid_parameters_conservative = vec!(conservative_function_builder(
            initial_grid.get(&key),
            time_step: f64,
            dx: f64,
        ));
    }
    return initial_grid;
}

fn conservative_function_builder(
    grid: GridUnity,
    time_step: f64,
    dx: f64,
) -> cip_rust::GridParameters {
    let f = vec![grid.density];
    let u = vec![grid.vx1];
    let g = vec![0.0];

    let mut grid_parameter = GridParameters {
        f: f,
        u: u,
        g: g,
        df_star: vec![0.0],
        dx: dx,
        dt: time_step,
    };

    return grid_parameter;
}

fn momentum_function_builder(grid: GridUnity, time_step: f64, dx: f64) -> cip_rust::GridParameters {
    let f = vec![grid.density * grid.vx1];
    let u = vec![grid.vx1];
    let g = vec![0.0];

    let grid_parameter = GridParameters {
        f: f,
        u: u,
        g: g,
        df_star: vec![0.0],
        dx: dx,
        dt: time_step,
    };

    return grid_parameter;
}
