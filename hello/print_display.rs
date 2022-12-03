use std::fmt;

#[derive(Debug)]
struct MixMax(i64, i64);

impl fmt::Display for MixMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: i64,
    y: i64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x:{},y:{})", self.x, self.y)
    }
}

fn main() {
    let minmax = MixMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MixMax(-300, 300);
    let small_range = MixMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3, y: 7 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}",point);
    //println!("What does Point2D look like in binary: {:b}?", point);
}
