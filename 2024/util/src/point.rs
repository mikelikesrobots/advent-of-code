#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point(pub i32, pub i32);
impl Point {
    pub fn add(&self, point: &Point) -> Point {
        Point(self.0 + point.0, self.1 + point.1)
    }

    pub fn sub(&self, point: &Point) -> Point {
        Point(self.0 - point.0, self.1 - point.1)
    }
    
    pub fn diff(&self, point: &Point) -> Point {
        Point(point.0 - self.0, point.1 - self.1)
    }
}
