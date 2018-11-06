use std::env;
use std::vec::Vec;

pub fn get_int_in_eniroment(input: &str) -> u32 {
  match env::var(input) {
    Ok(val) => val.to_string().parse::<u32>().unwrap(),
    Err(e) => panic!(e),
  }
}

pub fn get_float_in_eniroment(input: &str) -> f64 {
  match env::var(input) {
    Ok(val) => val.to_string().parse::<f64>().unwrap(),
    Err(e) => panic!(e),
  }
}

pub fn get_axis_size() -> Vec<u32> {
  let mut axis_size = Vec::new();
  axis_size.push(get_int_in_eniroment("FIRST_AXIS"));
  axis_size.push(get_int_in_eniroment("SECOND_AXIS"));
  axis_size.push(get_int_in_eniroment("THIRD_AXIS"));
  return axis_size;
}
