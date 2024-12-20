use std::{num::ParseIntError, str::FromStr};

use pathfinding::{grid::Grid, prelude::dijkstra};

struct MemoryRegion {
    all_cells: Vec<(usize, usize)>,
}
impl FromStr for MemoryRegion {
    type Err = ParseIntError;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let cells = puzzle
            .lines()
            .map(|l| {
                l.split(",")
                    .map(|x| x.parse())
                    .collect::<Result<Vec<usize>, _>>()
            })
            .map(|v| v.map(|v| (v[0], v[1])))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(MemoryRegion { all_cells: cells })
    }
}

impl MemoryRegion {
    fn part_a(&self, bytes: usize) -> Option<usize> {
        let cells = self.all_cells.iter().copied().take(bytes).collect::<Vec<_>>();
        let mut grid = Grid::from_coordinates(&cells)?;
        grid.invert();

        let end = (grid.width - 1, grid.height - 1);
        let shortest = dijkstra(
            &(0, 0), 
            |&p| {
                grid.neighbours(p).into_iter().map(|v| (v, 1))
            },
            |&p| p == end);
        shortest.map(|(_, cost)| cost)
    }

    fn part_b(&self, skip_bytes: usize) -> Option<(usize, usize)> {
        let (mut width, mut height) = (0, 0);
        for cell in &self.all_cells {
            width = width.max(cell.0);
            height = height.max(cell.1);
        }
        let end = (width, height);
        let mut grid = Grid::new(width + 1, height + 1);
        grid.fill();
        for cell in self.all_cells.iter().take(skip_bytes) {
            grid.remove_vertex(*cell);
        }

        self.all_cells.iter().skip(skip_bytes).find(|&&x| {
            grid.remove_vertex(x);
            dijkstra(
                &(0, 0), 
                |&p| {
                    grid.neighbours(p).into_iter().map(|v| (v, 1))
                }, 
                |&p| p == end).is_none()
        }).copied()
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let mem = MemoryRegion::from_str(puzzle).expect("Error parsing puzzle");
    println!("Part A: {:?}", mem.part_a(1024));
    // 62,32 wrong
    // 50,28 correct
    println!("Part B: {:?}", mem.part_b(1024));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_txt_part_a() {
        let puzzle = include_str!("../puzzle/test.txt");
        let mem = MemoryRegion::from_str(puzzle).unwrap();
        assert_eq!(Some(22), mem.part_a(12));
    }

    #[test]
    fn test_test_txt_part_b() {
        let puzzle = include_str!("../puzzle/test.txt");
        let mem = MemoryRegion::from_str(puzzle).unwrap();
        assert_eq!(Some((6, 1)), mem.part_b(0));
    }
}
