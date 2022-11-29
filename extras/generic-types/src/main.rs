// use std::cmp::{PartialOrd};

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<i64, i64> {
    fn y(&self) -> &i64 {
        &self.y
    }
}

 
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn main() {

    let number_list = vec![34, 1, 2, 26, 3, 4, 42, 5];
    println!("Largest number is {}", get_largest(number_list));

    let number_list = vec!['c', 'a', 'r', 'g', 'o'];
    println!("Largest number is {}", get_largest(number_list));

    let p1 = Point {x: 5, y: 10};
    let p2 = Point {x: 5.0, y: 10.0};
    let p3 = Point {x: 5, y: 10.4};
    let p4 = Point {x: "Hello", y: 'b' };
    let mixed34 = p3.mixup(p4);

    println!("X value {} {}", p1.x(), p1.y());
    println!("Mixup x: {}, y: {}", mixed34.x, mixed34.y);

    // error[E0599]: no method named `y` found for struct `Point<{float}, {float}>` in the current scope
    // println!("X value {} {}", p2.x(), p2.y());

}
