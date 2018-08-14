

struct gridUnity{
  density: f64,
  vx: f64,
  vy: f64,
  vz: f64,
  temperature: f64,
  pression: f64,
}
 
pub struct Grid{
  grid: 
    [gridUnity,
      [firstAxis,
       secondAxis,
       thirdAxis]
    ],
}