struct Rectangle {
    width: u64,
    length: u64,
}

impl Rectangle {
    fn area(&self) -> u64 {
        return self.width * self.length;
    }

    fn perimeter(&self) -> u64 {
        return 2 * (self.width + self.length);
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.length > other.length;
    }
}

fn main() {
    let rectangle1 = Rectangle {
        width: 50,
        length: 50,
    };

    let area1 = rectangle1.area();
    let per1 = rectangle1.perimeter();

    let rectangle2 = Rectangle {
        width: 100,
        length: 100,
    };
    let holdme = rectangle2.can_hold(&rectangle1);

    println!("Area: {area1}");
    println!("Perimeter: {per1}");
    println!("2 in 1: {holdme}");
}
