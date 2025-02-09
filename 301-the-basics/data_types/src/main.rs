fn main() {
//    let x: i8 = -2;
//    let y: u8 = 2;
//
//    println!("{} <- signed integer, {} <- unsigned integer", x, y);
//// base system 
//
//    let decimal = 02_111;
//    let octal = 0o377;
//    let hex = 0xff_ff;
//    let binary = 0b111;
//    println!("{} <- decimal, {} <- octal, {} <- hex, {} <- binary",decimal,octal,hex,binary);
//
//    let byte = b'B';
//    println!("{} <- ASCII Value of B", byte);
//
//    let z = 2.0; // f64 by default;
//
//    let w :f32 = 1.0;
//
//    // boolean 
//
//    let t = true; // rust infers the type;
//    let f : bool = false; // rust also does infer to this
//    
//    println!("{} <- f64 float, {} <- f32 float, {} <- true , {} <- false", z, w, t, f );
//    // arithmetic * + - / %
//    let g = 10;
//    let h = 4;
//
//    let remainder = g % h;
//
//    println!("{} <- remainder", remainder);
//      Tuples 
    let tup = ("Hello", 23, true);

    let (x,y,z) = tup;

    println!("{} <- first, {} <- second, {} <- third",x,y,z);

    // arrays [same type]
    
    let arrays = [3,4,5];

    println!("{} <- first, {}<- second, {} <- third", arrays[0], arrays[1], arrays[2]);

    let arrays2 : [i32;3] =  [5,6,7]; 

    println!("{} <- first , {} <- second, {} <- third", arrays2[0], arrays2[1], arrays2[2]);

    // Vectors [dynamic arrays]

    let mut nums = vec![1,2,3,4];
    nums.push(5);

    println!("{:?}", nums);

    // other ways to delcare a vect;

    let mut vect  = Vec::<i32>::new();
    vect.push(5);
    vect.push(3);
    println!("{:?}", vect);
    
    // how much memory is allocated
    let mut vectors =  Vec::<i32>::with_capacity(5);
    vectors.push(3);
    vectors.push(2);
    vectors.push(1);
    vectors.push(4);
    vectors.push(6);
    vectors.push(7);

    println!("{:?}, {} <- capacity", vectors, vectors.capacity());

    let new_array : Vec<i32> = (0..5).collect();
    
    let ref_array : &[i32] = &new_array[1..5];

    println!("{:?} <- new array, {:?} <- ref_array", new_array, ref_array );
        
    // strings and string slice 
    let s = String::from("Hello From Text"); // Ownership this is a string;
   // all strings slice
    let me =  "tuedon";
    let other = me.to_string();
    let other_me = &me;

    println!("{} <- Ownership, {} <- string_slice , {} <- now owns, {} <- string_slice ", s,me,other,other_me);
    

}
