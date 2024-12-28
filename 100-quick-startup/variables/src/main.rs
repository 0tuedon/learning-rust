fn rust_types(){
    //  unsigned integers
    let unsigned_integers:ua = 32
}
fn main() {
    // Definition
    println!("Hello, world!");
    let x: i128 = 2797979797979977979790;
    println!("{x}")

    // Mutability
    let mut y: i64 = 50050050505;
    println!("y is now:  {y}");
    y = 60;
    println!("y is now:  {y}");

    // SCOPE
    {
        let z: i8 = 127;
    }
    // Cannot be used outside
    // println!("{z}")

    // SHADOWING

    let v: i32 = 60;
    println!("{v}");
    let v:f32 = 60.024;

    println!("{v}");

    // Constant
}
