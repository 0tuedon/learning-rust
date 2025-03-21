#[allow(dead_code)]

enum Pet {
    Dog,
    Cat,
    Bird,
}
// We can impl (add methods to enums)

impl Pet {
    fn what_is_name(&self) -> &str {
        match self {
            Pet::Dog => "I am a Dog",
            Pet::Cat => " I am a Cat",
            Pet::Bird => "I am a Bird",
        }
    }
}
fn main() {
    let dog = Pet::Dog;

    println!("{:?}", dog.what_is_name());

    // FOr options
    // let x: i32 = 5;
    // let y: Option<i32> = 5;

    // let z = x + y;

    // println!("{}", x);

    // For Matching and Options

    print_one("Tuedons");

    // we can use with ifs too;

    let dog2: Option<Pet> = Some(Pet::Dog);
    if let Some(Pet::Dog) = dog2 {
        println!("Dog");
    } else {
        println!("Not A DOG");
    }

    // Stack
    let mut stack: Vec<i32> = Vec::new();
    stack.push(3);
    stack.push(10);
    stack.push(15);

    while let Some(top) = stack.pop() {
        println!("Matches- {}", top)
    }
}

fn print_one(x: &str) {
    match x {
        "Tuedon" => println!("It is tuedon"),
        _ => println!(" I don't know you"),
    }
}
