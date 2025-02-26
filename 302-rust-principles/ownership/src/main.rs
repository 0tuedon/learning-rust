fn main() {
    // Ownership in Rust
    //    let x = vec![1, 2, 3];
    //    let _y = x;
    //
    //    println!("{:?}", x);

    // Clone (works on heap)

    //    let x = vec![1, 2, 3, 4];
    //    let y = x.clone();
    //
    //    println!("{:?}", x);
    //    println!("{:?}", y);

    // Copy in rust (STACKS)

    let y: i32 = 1;
    let z = y;
    println!("{} and  {}", y, z);
    // This makes  a copy

    // More Move
    let s = String::from("Tuedon"); // create a string tuedon;
    takes_ownership(s);
    println!("{}", y);

    let str4: String = give_ownership();

    let str5 = take_and_give(str4);

    println!("{}", str5);

    // Move in control statements

    if true {
        let str6 = str5;
        println!("{}", str6);
    } else {
        let str6 = str5;
        println!("{}", str6);
    }

    // Loop;
    // loop {
    //     let str6 = str5;
    // }
    // Borrowing and Referencing &
    let mut brw = String::from("tuedon");
    change_string(&mut brw);
    println!("{}", brw);
}

fn change_string(s: &mut String) {
    s.push_str("Tuedon");
}

fn takes_ownership(s: String) {
    let str1 = s;
    println!("{}", str1);
}

fn give_ownership() -> String {
    "Tuedon".to_string()
}

fn take_and_give(s: String) -> String {
    s
}
