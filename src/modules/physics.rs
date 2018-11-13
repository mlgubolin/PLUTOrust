use fnv::FnvHashMap;
use grid::GridUnity;
use modules::physical_modules::hd;

pub fn calculations(initial_grid: FnvHashMap<(u32, u32, u32), GridUnity>)
 ->FnvHashMap<(u32, u32, u32), GridUnity>{
  return hd::calculations(initial_grid);
}