
fn main() {
    let val1 = 2;
    let val2 = 5;
    let ans = val2 % val1;
    println!("{}",ans);

    let mut vec1 =  vec!(1,2,3,4);
    println!("{:?}", vec1);
    vec1.pop();
    vec1.push(12);
    println!("{:?}", vec1);

    let word_to_concat = "Hello";

    let new_word = concat(word_to_concat).to_string();

    println!("{:?}", new_word);

    control_flow(30);
}

fn concat(word:&str) -> String {
    let CURRENT: &str = "World";

    let new_word: String = word.to_owned() + CURRENT;
    
    new_word
}

fn control_flow(num:i32){
    if num == 1 {
        println!("The Value is {}", num);
    }
    else if num  > 50 {
        println!("The Value is greater than 50 {}", num);
    }
    else if num < 25 {
        println!("The Value is less than 25 {}", num);
    }
    else {
        println!("The Value is Less than 25 but between than 50 {}", num);
    }
}
