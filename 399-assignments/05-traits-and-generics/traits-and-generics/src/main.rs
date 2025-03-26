trait CarMethods {
    fn set_mpg(&mut self, _mpg: i32);

    fn set_color(&mut self, _color: String);

    fn set_top_speed(&mut self, _top_speed: i32);
}

#[derive(Debug)]
struct Car {
    mpg: i32,
    top_speed: i32,
    color: String,
}

impl CarMethods for Car {
    fn set_mpg(&mut self, _mpg: i32) {
        self.mpg = _mpg
    }

    fn set_color(&mut self, _color: String) {
        self.color = _color
    }

    fn set_top_speed(&mut self, _top_speed: i32) {
        self.top_speed = _top_speed
    }
}

fn print<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value)
}
fn main() {
    let mut vehicle = Car {
        color: String::from("red"),
        top_speed: 30,
        mpg: 40,
    };

    println!("{:?}", vehicle);

    vehicle.set_color(String::from("green"));
    vehicle.set_mpg(300);
    vehicle.set_top_speed(320);

    println!("{:?}", vehicle);

    print(vehicle);
}
