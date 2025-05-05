fn main() {
    let vec: Vec<u32> = vec![1, 3, 5, 7, 9];

    let mul_ten: Vec<u32> = vec.into_iter().map(|n| n * 10).collect();

    println!("{:?}", mul_ten);
}
