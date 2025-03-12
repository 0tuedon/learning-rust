fn main() {
    /*
        Lifetimes in RUST

    */

    let r;
    {
        let x = 6;
        r = x;
    } // x is dropped

    // SOlution is to use an owned type instead.
    println!("{}", r);

    let x: &str = "Tuedon";
    let y: &str = "Amos";
    example(x, y);
}

// Showing lifetimes

fn example<'a>(_x: &'a str, y: &'a str) -> &'a str {
    y
}
