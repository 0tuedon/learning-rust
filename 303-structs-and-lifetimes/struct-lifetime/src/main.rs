struct Square {
    width: u32,
    height: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn change_width(self: &mut Square, width: u32) {
        self.width = width
    }
}

fn main() {
    let mut sq = Square {
        width: 20,
        height: 20,
    };

    println!("{}", sq.area());
    sq.change_width(30);
    println!("{}", sq.width);
}
