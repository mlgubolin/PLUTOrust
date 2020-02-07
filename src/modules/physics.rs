use std::collections::HashMap;
use grid::GridUnity;
use modules::physical_modules::hd;

pub fn calculations(
  initial_grid: HashMap<(u32, u32, u32), GridUnity>,
  size: Vec<u32>,
  time_step: f64,
) -> HashMap<(u32, u32, u32), GridUnity> {
  return hd::calculations(initial_grid, size, time_step,1.0);
}
