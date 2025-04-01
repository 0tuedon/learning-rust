use std::collections::{BinaryHeap, HashMap, HashSet};

use rand::rng;
use rand::seq::SliceRandom;

fn main() {
    // Vectors
    let mut nums: Vec<i32> = vec![];

    nums.push(1);
    nums.push(2);
    nums.push(3);

    let popped = nums.pop();

    println!("{:?}", popped);

    nums.insert(0, 8);
    nums.insert(3, 20);

    println!("{:?}", nums);

    nums.first();

    nums.last();

    nums.is_empty();

    nums.len();

    nums.remove(2);

    nums.sort();

    println!("{:?}", nums);

    nums.reverse();

    println!("{:?}", nums);

    nums.shuffle(&mut rng());

    println!("{:?}, for next", nums);

    let mut bheap = BinaryHeap::new();

    bheap.push(10);
    bheap.push(20);
    bheap.push(30);
    bheap.push(15);

    bheap.pop();
    println!("{:?}", bheap);

    println!("{:?}", bheap.peek());

    // Maps

    let mut hm = HashMap::new();

    hm.insert(1, 1);

    hm.insert(10, 2);

    hm.insert(20, 3);

    println!("{:?}", hm);

    hm.remove(&10); // returns only the key;

    hm.remove_entry(&20);

    hm.contains_key(&1);

    hm.is_empty();

    hm.clear();

    // SETS

    let mut hs = HashSet::new();

    hs.insert(1);
    hs.insert(2);
    hs.insert(3);
    hs.insert(4);
    hs.insert(5);

    for x in hs.iter() {
        println!("{}", x);
    }

    // FOR SETS LIKE IN MATHS
    let mut hs2 = HashSet::new();

    hs2.insert(2);
    hs2.insert(4);
    hs2.insert(6);
    hs2.insert(8);
    hs.insert(10);

    for x in hs.intersection(&hs2) {
        println!("{}", x);
    }

    for x in hs.union(&hs2) {
        println!("{}", x);
    }

    for x in hs.difference(&hs2) {
        println!("{}", x);
    }
    // SHORTHANDS FOR THE ABOVE

    let intersection = &hs & &hs2;
    let union = &hs | &hs2;

    let diff = &hs - &hs2;

    println!("{:?}", intersection);
    println!("{:?}", union);
    println!("{:?}", diff);
}
