use fnv::FnvHashMap;
use grid::GridUnity;
use modules::physical_modules::hd;

pub fn calculations(
  initial_grid: FnvHashMap<(u32, u32, u32), GridUnity>,
  size: Vec<u32>,
  time_step: f64,
) -> FnvHashMap<(u32, u32, u32), GridUnity> {
  return hd::calculations(initial_grid, size, time_step);
}
