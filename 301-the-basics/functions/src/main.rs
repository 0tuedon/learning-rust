fn main() {
    // print_phase();
    println!("{}", gcd(20,5));
}

//fn print_phase(){
//    println!("Hello world from print_phase")
//}

fn gcd (mut a : u32, mut b : u32) -> u32 {
    while a != 0{
        if a < b {
            let c = a;
            a =b ;
            b = c;
        }
        a = a% b;

    }
    b
} 
