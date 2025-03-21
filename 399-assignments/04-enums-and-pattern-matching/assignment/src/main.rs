enum Shapes {
    Triangle,
    Square,
    Pentagon,
    Octagon,
}

impl Shapes {
    fn corners(&self) -> i32 {
        match self {
            Shapes::Triangle => 3,
            Shapes::Square => 4,
            Shapes::Pentagon => 5,
            Shapes::Octagon => 8,
        }
    }
}
fn main() {
    let triangle = Shapes::Triangle;
    println!("{}", triangle.corners());

    let square = Shapes::Square;
    println!("{}", square.corners());

    let pentagon = Shapes::Pentagon;
    println!("{}", pentagon.corners());

    let octagon = Shapes::Octagon;
    println!("{}", octagon.corners());
}
