use std::collections::HashMap;
use std::env;
use std::vec::Vec;

pub struct grid_unity {
  density: f64,
  x: f64,
  y: f64,
  z: f64,
  vx: f64,
  vy: f64,
  vz: f64,
  temperature: f64,
  pression: f64,
}

impl grid_unity {
  fn new() -> grid_unity {
    return grid_unity {
      density: 0.,
      x: 0.,
      y: 0.,
      z: 0.,
      vx: 0.,
      vy: 0.,
      vz: 0.,
      temperature: 0.,
      pression: 0.,
    };
  }
}
pub fn set_initial_condition() -> (
  HashMap<(u32, u32, u32), grid_unity>,
  HashMap<(u32, u32, u32), grid_unity>,
) {
  let grid = create_blank_grid();
  let next_grid = create_blank_grid();
  return (grid, next_grid);
}

fn create_blank_grid() -> HashMap<(u32, u32, u32), grid_unity> {
  let mut grid = HashMap::new();
  let axis_size = get_axis_size();
  for i in 0..axis_size[0] {
    for j in 0..axis_size[1] {
      for k in 0..axis_size[2] {
        grid.insert((i, j, k), grid_unity::new());
      }
    }
  }
  return grid;
}

fn get_axis_size() -> Vec<u32> {
  let mut axis_size = Vec::new();
  axis_size.push(get_int_in_eniroment("FIRST_AXIS"));
  axis_size.push(get_int_in_eniroment("SECOND_AXIS"));
  axis_size.push(get_int_in_eniroment("THIRD_AXIS"));
  return axis_size;
}

fn get_int_in_eniroment(input: &str) -> u32 {
  match env::var(input) {
    Ok(val) => val.to_string().parse::<u32>().unwrap(),
    Err(e) => panic!(),
  }
}
