use std::env;

struct gridUnity {
  density: f64,
  vx: f64,
  vy: f64,
  vz: f64,
  temperature: f64,
  pression: f64,
}
pub fn create_grid() {}

fn get_int_in_eniroment(input: &str) -> u32 {
  match env::var(input) {
    Ok(val) => val.to_string().parse::<u32>().unwrap(),
    Err(e) => panic!(),
  }
}
