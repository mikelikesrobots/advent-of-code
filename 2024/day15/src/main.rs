use std::str::FromStr;

mod warehouse_err;
mod warehouse;
mod wide_warehouse;
use warehouse::Warehouse;
use wide_warehouse::WideWarehouse;

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let mut warehouse = Warehouse::from_str(puzzle).expect("Could not parse warehouse");
    println!("Part A: {}", warehouse.part_a());
    let mut wide_warehouse = WideWarehouse::from_str(puzzle).expect("Could not parse wide warehouse");
    println!("Part B: {}", wide_warehouse.part_b());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_small_txt() {
        let puzzle = include_str!("../puzzle/test_small.txt");
        let mut warehouse = Warehouse::from_str(puzzle).unwrap();
        println!("{:?}", warehouse);
        assert_eq!(2028, warehouse.part_a());
    }

    #[test]
    fn test_test_large_txt() {
        let puzzle = include_str!("../puzzle/test_large.txt");
        let mut warehouse = Warehouse::from_str(puzzle).unwrap();
        assert_eq!(10092, warehouse.part_a());
    }
}
