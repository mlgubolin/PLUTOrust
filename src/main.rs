mod modules;
use modules::grid;
use modules::utils;
use modules::physics;
extern crate fnv;
use std::{thread, time};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let initial_grid = grid::set_initial_condition(init);

    let mut tf = utils::get_float_in_eniroment("FINAL_TIME");
    let mut time_step = utils::get_float_in_eniroment("TIME_STEP");
    let mut t = 0.;

    while t < tf {
        let next_grid = physics::calculations(initial_grid);
        initial_grid = next_grid;
        t += time_step;
    }
}

//Function for inital conditions
fn init() -> grid::GridUnity {
    let grid = grid::GridUnity {
        density: 1.,
        x1: 0.,
        x2: 0.,
        x3: 0.,
        vx1: 0.,
        vx2: 0.,
        vx3: 0.,
        temperature: 0.,
        pression: 0.,
    };

    return grid;
}

//Function for potential
