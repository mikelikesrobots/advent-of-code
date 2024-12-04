pub struct Point(pub i32, pub i32);
impl Point {
    pub fn add(&self, point: &Point) -> Point {
        Point(self.0 + point.0, self.1 + point.1)
    }
}
