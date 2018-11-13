use fnv::FnvHashMap;
use modules::utils;

pub struct GridUnity {
  pub density: f64,
  pub x1: f64,
  pub x2: f64,
  pub x3: f64,
  pub vx1: f64,
  pub vx2: f64,
  pub vx3: f64,
  pub temperature: f64,
  pub pression: f64,
}

// impl GridUnity {
//   fn new() -> GridUnity {
//     return GridUnity {
//       density: 0.,
//       x1: 0.,
//       x2: 0.,
//       x3: 0.,
//       vx1: 0.,
//       vx2: 0.,
//       vx3: 0.,
//       temperature: 0.,
//       pression: 0.,
//     };
//   }
// }

pub fn set_initial_condition(
  init: impl Fn(f64, f64, f64) -> GridUnity,
) -> FnvHashMap<(u32, u32, u32), GridUnity> {
  let mut grid = FnvHashMap::default();
  let axis_size = utils::get_axis_size();

  let x1_size = utils::get_float_in_eniroment("X1_SIZE");
  let x2_size = utils::get_float_in_eniroment("X2_SIZE");
  let x3_size = utils::get_float_in_eniroment("X3_SIZE");

  for i in 0..(axis_size[0]+4) {
    for j in 0..(axis_size[1]+4) {
      for k in 0..(axis_size[2]+4) {
        let x1 = x1_size * (i-2) as f64;
        let x2 = x2_size * (j-2) as f64;
        let x3 = x3_size * (k-2) as f64;
        let grid_unity = init(x1,x2,x3);

        grid.insert((i, j, k), grid_unity);
      }
    }
  }

  return grid;
}

// pub fn create_blank_grid() -> FnvHashMap<(u32, u32, u32), GridUnity> {
//   let mut grid = FnvHashMap::default();
//   let axis_size = utils::get_axis_size();
//   for i in 0..axis_size[0] {
//     for j in 0..axis_size[1] {
//       for k in 0..axis_size[2] {
//         grid.insert((i, j, k), GridUnity::new());
//       }
//     }
//   }
//   return grid;
// }
