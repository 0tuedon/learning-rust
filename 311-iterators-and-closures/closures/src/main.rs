#[derive(Debug)]
struct City {
    name: String,
    population: u64,
}

fn sort_pop(city: &mut Vec<City>) {
    // using closure we can access city

    city.sort_by_key(|c| c.population)
}

struct Range {
    start: u32,
    end: u32,
}

impl Iterator for Range {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        let result = Some(self.start);
        self.start += 1;
        result
    }
}
fn main() {
    let a = City {
        name: String::from("a"),
        population: 20,
    };
    let b = City {
        name: String::from("b"),
        population: 10,
    };
    let c = City {
        name: String::from("c"),
        population: 120,
    };
    let d = City {
        name: String::from("d"),
        population: 30,
    };
    let e = City {
        name: String::from("e"),
        population: 60,
    };
    let f = City {
        name: String::from("f"),
        population: 15,
    };

    let mut vec: Vec<City> = Vec::new();
    vec.push(a);
    vec.push(b);
    vec.push(c);
    vec.push(d);
    vec.push(e);
    vec.push(f);

    sort_pop(&mut vec);

    println!("{:?}", vec);

    // create iterators for a range
    let range = Range { start: 0, end: 5 };

    for i in range {
        println!("{}", i);
    }
}
