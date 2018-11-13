mod modules;
use modules::grid;
use modules::utils;
use modules::physics;
extern crate fnv;

fn main() {
    let mut initial_grid = grid::set_initial_condition(init);

    let tf = utils::get_float_in_eniroment("FINAL_TIME");
    let time_step = utils::get_float_in_eniroment("TIME_STEP");
    let mut t = 0.;

    while t < tf {
        let next_grid = physics::calculations(initial_grid);
        initial_grid = next_grid;
        t += time_step;
    }
}

//Function for inital conditions
fn init(x1: f64, x2: f64, x3: f64) -> grid::GridUnity {

  let grid = grid::GridUnity {
    density: 1.,
    x1: x1,
    x2: x2,
    x3: x3,
    vx1: 0.,
    vx2: 0.,
    vx3: 0.,
    temperature: 0.,
    pression: 0.,
  };
    return grid;
}

//Function for potential
