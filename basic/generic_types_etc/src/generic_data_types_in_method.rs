// Implementing a method named x on the Point<T> struct that will return a reference to the x field of type T
#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// An impl block that only applies to a struct with a particular concrete type for the generic type parameter T
#[allow(dead_code)]
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2).sqrt()
    }
}

//  A method that uses generic types that are different from its struct’s method_definition
#[allow(dead_code)]
struct PointNew<X1, Y1> {
    x: X1,
    y: Y1,
}

#[allow(dead_code)]
impl<X1, Y1> PointNew<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointNew<X2, Y2>) -> PointNew<X1, Y2> {
        PointNew {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn method_definition() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = PointNew { x: 5, y: 10.4 };
    let p2 = PointNew { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
