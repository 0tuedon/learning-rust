fn main() {
    let a = 1;

    let result = if a == 1 { true } else { false };

    println!("{}", result);

    // Loop, Nested Loop
    let mut number = 0;
    // Loop equivalent to JS while(true){}
    'counter: loop {
        let mut decrease: i32 = 5;
        println!("Count : {}", number);

        loop {
            println!("Decrease : {}", decrease);
            if decrease == 4 {
                break;
            }
            if number == 2 {
                break 'counter;
            }
            decrease -= 1;
        }
        number += 1;
    }

    // While loop

    let mut while_num = 0;

    while while_num < 5 {
        println!("The number is : {}", while_num);
        while_num += 1
    }

    // FOR loops : best for looping collections of stuffs
    let for_array: Vec<i32> = (0..5).collect();

    for element in for_array {
        println!("{}", element);
    }

    for element in (1..4).rev() {
        println!("{}", element)
    }
    println!("LIFT OFF");
}
