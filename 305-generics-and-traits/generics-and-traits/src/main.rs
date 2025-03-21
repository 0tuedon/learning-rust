// Generics

use std::ops::Add;

trait Overview {
    fn overview(&self) -> String {
        String::from("This is Rust")
    }
}
struct Course {
    heading: String,
    author: String,
}

struct Course2 {
    heading: String,
    author: String,
}
impl Overview for Course {}

// Debug
impl Drop for Course {
    fn drop(&mut self) {
        println!("For Drop");
    }
}

// Operation Overloading

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let course_one = Course {
        heading: String::from("What A DAy"),
        author: String::from("Tuedon"),
    };
    let course_two = Course {
        heading: String::from("Halle"),
        author: String::from("Tuoyo"),
    };

    course_overview(&course_one);
    let point_a = Point { x: 3.0, y: 3.0 };
    let point_b = Point { x: 2.0, y: 2.0 };

    let sum = point_a + point_b;

    println!("{:?}", sum)
    // Operator Overloading
}

fn course_overview<T: Overview>(course_1: &T) {
    println!("{}", course_1.overview())
}
