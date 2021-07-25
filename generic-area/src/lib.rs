use std::f64::consts::PI;

pub trait HasArea {
    fn area(&self) -> f64;
}

pub fn print_area<T: HasArea>(area: T) {
    println!("area = {}", area.area());
}

pub struct Circle {
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * PI
    }
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Circle { radius }
    }
}

pub struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl HasArea for Triangle {
    fn area(&self) -> f64 {
        let p = (self.a + self.b + self.c) / 2.0;
        (p * (p - self.a) * (p - self.b) * (p - self.c)).sqrt()
    }
}

impl Triangle {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Triangle { a, b, c }
    }
}

pub struct Square {
    length: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.length * self.length
    }
}

impl Square {
    pub fn new(length: f64) -> Self {
        Square { length }
    }
}
