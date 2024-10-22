fn main() {
    let  _x = 5;
    let _x = _x + 3;
    {
        let _x = _x * 2;
        println!("{_x}");
    }
    println!("{_x}");

    // Testing spaces on different values
    let  spaces = "    ";
    let spaces = spaces.len();

    println!("{spaces}")
}
