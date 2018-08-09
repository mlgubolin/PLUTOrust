mod structs;

fn main() {
    let firstAxis = 10;
    let secondAxis = 10;
    let thirdAxis = 10;
    let grid = structs::Grid::grid {
        firstAxis: firstAxis,
        secondAxis: secondAxis,
        thirdAxis: thirdAxis,
    };
}
