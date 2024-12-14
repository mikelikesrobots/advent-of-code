#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
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

// impl Ord for Point {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         match (self.0.cmp(&other.0), self.1.cmp(&other.1)) {
//             (Ordering::Greater, _) => Ordering::Greater,
//             (Ordering::Less, _) => Ordering::Less,
//             (_, Ordering::Greater) => Ordering::Greater,
//             (_, Ordering::Less) => Ordering::Less,
//             (Ordering::Equal, Ordering::Equal) => Ordering::Equal,
//         }
//     }
// }
