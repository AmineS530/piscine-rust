use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Circle {
            center: Point(a, b),
            radius: c,
        }
    }
    pub fn diameter(self) -> f64 {
        self.radius * 2.0
    }
    pub fn area(self) -> f64 {
        self.radius.powi(2) * PI
    }
    pub fn intersect(self, other: Circle) -> bool {
        let distance = self.center.distance(other.center);
        distance < self.radius + other.radius
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(self, other: Point) -> f64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        // let dz = self.z - other.z; // Uncomment for 3D points
        (dx * dx + dy * dy).sqrt() // Add + dz * dz for 3D
    }
}
