

struct gridUnity{
  density: f64,
  vx: f64,
  vy: f64,
  vz: f64,
  temperature: f64,
  pression: f64,
}
 
pub struct Grid{
  firstAxis: u64,
  secondAxis: u64,
  thirdAxis: u64,
  grid: 
    [gridUnity,
      [firstAxis,
       secondAxis,
       thirdAxis]
    ],
}