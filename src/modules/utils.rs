use std::env;

pub fn get_int_in_eniroment(input: &str) -> usize {
  match env::var(input) {
    Ok(val) => val.to_string().parse::<usize>().unwrap(),
    Err(e) => panic!(e),
  }
}

pub fn get_float_in_eniroment(input: &str) -> f64 {
  match env::var(input) {
    Ok(val) => val.to_string().parse::<f64>().unwrap(),
    Err(e) => panic!(e),
  }
}

pub fn get_axis_size() -> Vec<usize> {
  let mut axis_size = Vec::new();
  axis_size.push(get_int_in_eniroment("FIRST_AXIS"));
  axis_size.push(get_int_in_eniroment("SECOND_AXIS"));
  axis_size.push(get_int_in_eniroment("THIRD_AXIS"));
  return axis_size;
}