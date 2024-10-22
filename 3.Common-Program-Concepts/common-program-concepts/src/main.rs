fn main(){
    let mut x: i32 = 5;
    println!("This is 5 {x}");
    x = 6;
    println!("This is 6 {x}");

    const SECONDS_IN_A_DAY:u32 =  60 * 60 * 24;
    println!("{SECONDS_IN_A_DAY}");
}