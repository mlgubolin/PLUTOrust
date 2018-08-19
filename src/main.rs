mod structs;
use structs::grid;
extern crate fnv;

fn main() {
    let mut initial_grid = grid::create_blank_grid();
    let next_grid = grid::create_blank_grid();
    initial_grid = grid::set_initial_condition(initial_grid);
}
