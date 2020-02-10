use multiarray::*;
use utils;

#[derive(Clone)]
pub struct GridUnity {
  pub x1: f64,
  pub x2: f64,
  pub x3: f64,
  pub vx1: f64,
  pub vx2: f64,
  pub vx3: f64,
  pub temperature: f64,
  pub pressure: f64,
  pub density: f64,
}

impl GridUnity {
  pub fn new(
    x1: f64,
    x2: f64,
    x3: f64,
    vx1: f64,
    vx2: f64,
    vx3: f64,
    temperature: f64,
    pressure: f64,
    density: f64,
  ) -> GridUnity {
    return GridUnity {
      x1: x1,
      x2: x2,
      x3: x3,
      vx1: vx1,
      vx2: vx2,
      vx3: vx3,
      temperature: temperature,
      pressure: pressure,
      density: density,
    };
  }
}
pub fn set_initial_condition(
  init: impl Fn(f64, f64, f64) -> GridUnity,
) -> (MultiArray<GridUnity, Dim3>, Vec<usize>) {
  let axis_size = utils::get_axis_size();
  let mut grid = Array3D::new(
    [axis_size[0] + 4, axis_size[1] + 4, axis_size[2] + 4],
    GridUnity {
      x1: 0.,
      x2: 0.,
      x3: 0.,
      vx1: 0.,
      vx2: 0.,
      vx3: 0.,
      temperature: 0.,
      pressure: 0.,
      density: 0.,
    },
  );
  let x1_size = utils::get_float_in_eniroment("X1_SIZE");
  let x2_size = utils::get_float_in_eniroment("X2_SIZE");
  let x3_size = utils::get_float_in_eniroment("X3_SIZE");

  for i in 0..(axis_size[0] + 4) {
    for j in 0..(axis_size[1] + 4) {
      for k in 0..(axis_size[2] + 4) {
        let x1 = x1_size * (i - 2) as f64;
        let x2 = x2_size * (j - 2) as f64;
        let x3 = x3_size * (k - 2) as f64;
        let grid_unity = init(x1, x2, x3);

        grid[[i, j, k]] = grid_unity;
      }
    }
  }

  return (grid, axis_size);
}
