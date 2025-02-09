#[allow(unused_variables)]
#[allow(dead_code)]
fn rust_types() {
    //  unsigned integers
    let _unsigned_integers: u8 = 32;
    // signed integers
    let _signed_integers: i8 = -32;
    // floating point numbers
    let _floating_point_numbers: f32 = 5.0;

    // Platform specific integers
    let _arch_1: isize = 32;
    let _arch_2: usize = 32;

    // Boolean
    let _boolean: bool = true;

    // Character
    let _character: char = 'a';

    // Tuple
    let _tuple: (i32, f64, u8) = (500, 6.4, 1);

    // Array
    let _array: [i32; 5] = [1, 2, 3, 4, 5];

    // Type Alias
    type Age = u8;
    let _peter_age: Age = 20;

    // Type Conversion
    let tuedon:Tuple = [];
    // Mutability
    let mut y: i64 = 50050050505;
    println!("y is now:  {y}");
    y = 60;
    println!("y is now:  {y}");

    // SCOPE
    {
        let _z: i8 = 127;
    }
    // Cannot be used outside
    // println!("{z}")

    // SHADOWING

    let v: i32 = 60;
    println!("{v}");
    let v: f32 = 60.024;

    println!("{v}");

    // Constant
}
