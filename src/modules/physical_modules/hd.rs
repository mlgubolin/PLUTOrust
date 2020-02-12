use cip_rust::GridParameters;
use grid::GridUnity;
use multiarray::{Array3D, Dim3, MultiArray};

pub fn calculations(
    initial_grid: MultiArray<GridUnity, Dim3>,
    axis_size: &Vec<usize>,
    time_step: f64,
    dx: f64,
) -> MultiArray<GridUnity, Dim3> {
    let mut grid_parameters_conservative = Array3D::new(
        [axis_size[0] + 4, axis_size[1] + 4, axis_size[2] + 4],
        GridParameters {
            f: vec![0.0],
            g: vec![0.0],
            u: vec![0.0],
            df_star: vec![0.0],
            dx: 0.,
            dt: time_step,
        },
    );
    for i in 0..(axis_size[0] + 4) {
        grid_parameters_conservative[[i, 0, 0]] =
            conservative_function_builder(&initial_grid[[i, 0, 0]], time_step, dx);
    }
    let mut f_result = Vec::new();
    for i in 2..axis_size[0] {
        if i != 2 {
            f_result.append(&mut cip_rust::cip_1d(cip_rust::Grids {
                previous_previous: grid_parameters_conservative[[i - 2, 0, 0]].clone(),
                previous: grid_parameters_conservative[[i - 1, 0, 0]].clone(),
                current: grid_parameters_conservative[[i, 0, 0]].clone(),
                next: grid_parameters_conservative[[i + 1, 0, 0]].clone(),
                next_next: grid_parameters_conservative[[i + 2, 0, 0]].clone(),
            }))
        }
    }
    println!("{}", f_result[0]);
    return initial_grid;
}

fn conservative_function_builder(grid: &GridUnity, time_step: f64, dx: f64) -> GridParameters {
    let f = vec![grid.density];
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
