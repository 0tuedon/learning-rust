fn main() {
    let mut vectors = vec![1, 3, 5, 7];
    print_rust_vec(vectors.clone());
    vectors.push(15);
    println!("{:?}", vectors);
    adds_two(8);
}

fn adds_two(num_1: i8) -> i8 {
    num_1 + 2
}
fn print_rust_vec(nums: Vec<i32>) -> bool {
    println!("{}", nums[0]);
    if nums[0] == 1 { true } else { false }
}
