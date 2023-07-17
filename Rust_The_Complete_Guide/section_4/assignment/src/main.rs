#[derive(Debug)]
struct Car {
    mpg: i8,
    color: String,
    top_speed: i16,
}

impl Car {
    fn set_mpg(&mut self, mpg: i8) {
        self.mpg = mpg
    }

    fn set_color(&mut self, color: String) {
        self.color = color
    }

    fn set_top_speed(&mut self, top_speed: i16) {
        self.top_speed = top_speed
    }
}

fn main() {
    let mut car = Car {
        mpg: 0,
        color: String::from("Red"),
        top_speed: 0,
    };

    car.set_mpg(15);
    car.set_color(String::from("blue"));
    car.set_top_speed(250);

    println!("{:?}", car);
}
