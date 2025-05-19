use std::rc::Rc;

struct NumberHeap {
    is_number:u32,
}

fn main() {
    //Question 1: Create a variable on the stack and a variable on the heap. Multiply their values and print out the results.
        let stack_var =  5;
        let heap_var = NumberHeap{is_number:6};
        let result =  stack_var * heap_var.is_number;
            println!("{}", result);
    //Question 2: Create a variable that holds a String
        let string_hold =  String::from("Tuedon");
   // {
        //Create a reference counting smart pointer that points to the above String.
        let string_ref = Rc::new(string_hold);
        
        
        //Print out how many references the smart pointer has.
            println!("{}", Rc::<String>::strong_count(&string_ref));
        //Code block
       // {
            //Create another reference counting smart pointer that points to our first smart pointer
                let string_ref_2 = string_ref.clone();
            //Print out how many references each smart pointer has
            println!("{}", Rc::<String>::strong_count(&string_ref));

        //}
        //What value is dropped here?
        //Print out how many references out first smart pointer has


    //} //What value is dropped here?
    //Comment out the line below. What do you think will happen when you try to run the program now?
    //println!("rc_value: {}", rc_value);
}
