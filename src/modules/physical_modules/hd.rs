use fnv::FnvHashMap;
use grid::GridUnity;
use cip_rust::GridParameters;

pub fn calculations(
    initial_grid: FnvHashMap<(u32, u32, u32), GridUnity>,
    size: Vec<u32>, time_step: f64
) -> FnvHashMap<(u32, u32, u32), GridUnity> {
    for i in 2..(size[0]+2) {
        let grids = create_grids_parameters_struct(&initial_grid,i);
    }
    return initial_grid;
}

fn create_grids_parameters_struct(initial_grid: &FnvHashMap<(u32, u32, u32), GridUnity>,indexes: u32){
    let mut grids : cip_rust::Grids;

    let previous_previous = cip_rust::GridParameters{
        f:
    }

    // previous_previous


}