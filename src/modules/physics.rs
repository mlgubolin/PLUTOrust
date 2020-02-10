use multiarray::{Dim3, MultiArray};
use grid::GridUnity;
use modules::physical_modules::hd;

pub fn calculations(
  initial_grid: MultiArray<GridUnity,Dim3>,
  size: &Vec<usize>,
  time_step: f64,
) -> MultiArray<GridUnity, Dim3> {
  return hd::calculations(initial_grid, &size, time_step,1.0);
}
