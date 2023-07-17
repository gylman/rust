enum Shape {Triangle, Square, Pentagon, Octagon}

impl Shape {
    fn corners(&self) -> i8 {
        match self {
            Shape::Triangle => 3,
            Shape::Square => 4,
            Shape::Pentagon => 5,
            _ => 8,
        }
    }
}

fn main() {
    let shape = Shape::Octagon;
    println!("{}", shape.corners());
}
