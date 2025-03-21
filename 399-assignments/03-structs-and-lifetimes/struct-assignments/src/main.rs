/*
* Create a struct called Car with the fields: mpg, color, and top_speed. Once the struct is created, implement the following methods: set_mpg, set_color, and set_top_speed.
* Once you have created these methods, create a car, provide it default values, and then set the mpg, color, and top speed and then print them out.
*/

struct Car {
    mpg: u8,
    top_speed: u16, // This is measured in killmeters
    color: String,
}

impl Car {
    fn set_mpg(&mut self, mpg: u8) {
        self.mpg = mpg;
    }

    fn set_color(&mut self, color: String) {
        self.color = color;
    }

    fn set_top_speed(&mut self, top_speed: u16) {
        self.top_speed = top_speed;
    }
}

fn main() {
    let mut samuel_car = Car {
        color: "Red".to_string(),
        mpg: 40,
        top_speed: 256,
    };

    println!(
        "color:{}, top_speed:{}, mpg:{}",
        samuel_car.color, samuel_car.top_speed, samuel_car.mpg
    );

    // Methods Implementation

    samuel_car.set_mpg(200);

    samuel_car.set_color("Pink".to_string());

    samuel_car.set_top_speed(300);

    println!(
        "color:{}, top_speed:{}, ,mpg:{}",
        samuel_car.color, samuel_car.top_speed, samuel_car.mpg
    );
}
