use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;

// Okay. Let's think here.
// We need to figure out 8 places (4 pairs) where the design isn't being calculated correctly.
// We know it's meant to be an adder, which means we know what the start and end numbers are supposed to be.
// We can tell the difference between the actual and the correct result.
// We have the bits to determine that.
// We also know the exact number of pairs, and that they need to be SWAPPED.
// Naive solution would be to iterate over ALL non-const wires, testing swapping all of them
// But that would be a huge number of checks. There are thousands of wires, we'd need to check 1000s^4.
// Idk, that sounds doable, but still doesn't scale well.
// I'm sure there's a better way.

// We only need to consider wires that contribute to the points that are going wrong.
// How many of those are there? Maybe those^4 is acceptable?
// Let's figure out the bits that are wrong, then work out the wires that contribute to them
// and collect them into a big list and print that list out. Or its length.

#[derive(Debug, Clone)]
enum WireSource {
    And(String, String),
    Or(String, String),
    Xor(String, String),
    Const(u64),
}

struct CrossedWires {
    wires: HashMap<String, WireSource>,
}

#[derive(Debug)]
enum CrossedWiresParseErr {
    NoWiresFound,
    NoGatesFound,
    BadlyFormedWire,
    BadlyFormedGate,
}

impl FromStr for CrossedWires {
    type Err = CrossedWiresParseErr;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let mut wires = HashMap::new();
        let lines = &mut puzzle.lines();
        for line in &mut lines.take_while(|l| !l.is_empty()) {
            let wire = line
                .split(": ")
                .nth(0)
                .ok_or(CrossedWiresParseErr::BadlyFormedWire)?;
            let val = line
                .split(": ")
                .nth(1)
                .ok_or(CrossedWiresParseErr::BadlyFormedWire)?
                .parse::<u64>()
                .map_err(|_| CrossedWiresParseErr::BadlyFormedWire)?;
            wires.insert(wire.to_string(), WireSource::Const(val));
        }

        let const_len = wires.len();
        if const_len == 0 {
            return Err(CrossedWiresParseErr::NoWiresFound);
        }

        for line in lines {
            let mut parts = line.split_ascii_whitespace();
            let source0 = parts
                .next()
                .ok_or(CrossedWiresParseErr::BadlyFormedGate)?
                .to_string();
            let op = parts.next().ok_or(CrossedWiresParseErr::BadlyFormedGate)?;
            let source1 = parts
                .next()
                .ok_or(CrossedWiresParseErr::BadlyFormedGate)?
                .to_string();
            let result = parts
                .nth(1)
                .ok_or(CrossedWiresParseErr::BadlyFormedGate)?
                .to_string();

            let gate = match op {
                "AND" => WireSource::And(source0, source1),
                "XOR" => WireSource::Xor(source0, source1),
                "OR" => WireSource::Or(source0, source1),
                _ => return Err(CrossedWiresParseErr::BadlyFormedGate),
            };
            wires.insert(result, gate);
        }

        if wires.len() == const_len {
            return Err(CrossedWiresParseErr::NoGatesFound);
        }

        Ok(Self { wires })
    }
}

impl<'a> CrossedWires {
    fn value(&self, wire: &String) -> Option<u64> {
        match self.wires.get(wire) {
            None => None,
            Some(WireSource::Const(x)) => Some(*x),
            Some(WireSource::And(l, r)) => self.value(l).and_then(|l| self.value(r).map(|r| l & r)),
            Some(WireSource::Xor(l, r)) => self.value(l).and_then(|l| self.value(r).map(|r| l ^ r)),
            Some(WireSource::Or(l, r)) => self.value(l).and_then(|l| self.value(r).map(|r| l | r)),
        }
    }

    fn sub_sources_of(
        wires: &HashMap<String, WireSource>,
        left_wire: &String,
        right_wire: &String,
    ) -> Option<HashSet<String>> {
        let current = vec![left_wire.to_string(), right_wire.to_string()];
        let left = CrossedWires::sources_of(wires, left_wire);
        let right = CrossedWires::sources_of(wires, right_wire);
        let (left, right) = match (left, right) {
            (Some(left), Some(right)) => (left, right),
            _ => return None,
        };

        Some(current.into_iter().chain(left).chain(right).collect())
    }

    fn sources_of(wires: &HashMap<String, WireSource>, wire: &String) -> Option<HashSet<String>> {
        match wires.get(wire) {
            None => None,
            Some(WireSource::Const(_)) => Some(HashSet::new()),
            Some(WireSource::And(l, r))
            | Some(WireSource::Or(l, r))
            | Some(WireSource::Xor(l, r)) => CrossedWires::sub_sources_of(wires, l, r),
        }
    }

    fn convert_starting_with(&self, start: char) -> Option<u64> {
        let mut keys: Vec<&String> = self
            .wires
            .keys()
            .filter(|key| key.starts_with(start))
            .collect();
        keys.sort_unstable();
        keys.reverse();

        keys.iter()
            .try_fold(0, |acc, gate| self.value(gate).map(|val| (acc << 1) + val))
    }

    fn deps_1(wires: &HashMap<String, WireSource>, wire: &String) -> Option<Vec<String>> {
        match wires.get(wire) {
            None => None,
            Some(WireSource::Const(_)) => Some(vec![]),
            Some(WireSource::And(l, r))
            | Some(WireSource::Or(l, r))
            | Some(WireSource::Xor(l, r)) => Some(vec![l.to_string(), r.to_string()]),
        }
    }

    fn search_from_depth(
        &'a self,
        depth_limit: usize,
        search_space: &'a HashSet<&'a String>,
        acc: Vec<&'a String>,
        depth: usize,
        correct_result: u64,
    ) -> Option<Vec<&String>> {
        for cand in search_space.iter() {
            if acc.contains(cand) {
                continue;
            }
            let next_acc: Vec<_> = acc.iter().copied().chain(vec![*cand]).collect();
            if depth < depth_limit - 1 {
                // Search space from deeper in
                if let Some(x) = self.search_from_depth(
                    depth_limit,
                    search_space,
                    next_acc,
                    depth + 1,
                    correct_result,
                ) {
                    return Some(x);
                }
            } else {
                // println!("Checking: {:?}", next_acc);
                // if next_acc.contains(&&"z00".to_string()) && next_acc.contains(&&"z01".to_string()) && next_acc.contains(&&"z02".to_string()) && next_acc.contains(&&"z05".to_string()) {
                //     println!("Found the right entries");
                // }
                // We have a full accumulator. Check swapping wires in order.
                let mut wire_sources = self.wires.clone();
                for chunk in next_acc.chunks(2) {
                    let mut left = match wire_sources.get(chunk[0]) {
                        Some(x) => x,
                        None => return None,
                    }
                    .clone();
                    let mut right = match wire_sources.get(chunk[1]) {
                        Some(x) => x,
                        None => return None,
                    }
                    .clone();
                    wire_sources.get_mut(chunk[0]).replace(&mut right);
                    wire_sources.get_mut(chunk[1]).replace(&mut left);
                }

                // Now check against the wire sources
                let other = CrossedWires {
                    wires: wire_sources,
                };
                if other
                    .convert_starting_with('z')
                    .iter()
                    .any(|z| *z == correct_result)
                {
                    return Some(next_acc.to_vec());
                }
            }
        }
        None
    }

    fn part_a(&self) -> Option<u64> {
        // Find all the keys starting with z
        self.convert_starting_with('z')
    }

    fn part_b(&self, depth_limit: usize) -> Option<String> {
        let x = self.convert_starting_with('x');
        let y = self.convert_starting_with('y');
        let z_correct = x.and_then(|x| y.map(|y| y + x));
        let z_actual = self.convert_starting_with('z');

        let (z_correct, z_actual) = match (z_correct, z_actual) {
            (Some(correct), Some(actual)) => (correct, actual),
            _ => return None,
        };
        let diff = z_correct ^ z_actual;

        let mut bad_wires = vec![];
        for idx in 0.. {
            let mask = 1 << idx;
            if mask > diff {
                break;
            }
            if diff & (1 << idx) > 0 {
                bad_wires.push(format!("z{:#02}", idx));
            }
        }

        // Okay! Even narrowing down to 132 entries, we're iterating over it 8 times
        // So 132^8 possibilities. Which is a lot.
        // Hence it taking freaking forever. There has to be a better way..
        // Do we try to figure out the intermediate steps? Compare the design to a correct one?
        // Can we alter the inputs to see how it relates to the outputs?
        // Can we simulate each layer of the network, finding what we need to swap to get the right output?
        // Maybe we start with the surface wires - z wires
        // We figure out which ones we need to swap in order to make the correct number
        // How many are there? Quite a few, I thought
        // 10 bits are wrong
        // Can we swap bits around in x/y until we get something that's correct?
        // How many bits are involved for each z bit? Presumably it's two max, right?
        // So we can partition them a bit?
        // Let's see what z0 and z1 depend on

        // let xydeps = |s: &str| {
        //     self.sources_of(&s.to_string()).unwrap().into_iter().filter(|wire| wire.starts_with('x') || wire.starts_with('y')).map(|s| s.to_string()).collect::<Vec<_>>()
        // };
        // let z0deps = xydeps("z00");
        // let z1deps = xydeps("z02");
        // println!("z0 xydeps: {:?}", z0deps);
        // println!("z1 xydeps: {:?}", z1deps);

        // z0 should depend on x0, y0.
        // z1 should depend on x0, x1, y0, y1.
        // z2 should depend on x0, x1, x2, y0, y1, y2.
        // Every bit n should add x_n, y_n to the list of dependencies.
        // If extra bits are added, we need to get rid of them.
        // Go through the network above it to find the point where the extra dependencies get added.
        // That's now a swap candidate. Now I need to find the wires to swap with that to correct it.
        // Oh, also, there might be two sets of wires wrong, and there's nothing we can do to detect it. :-)
        // I guess we keep looping until we find the right answer.
        // The first gate where we get new additions doesn't mean that's the one which needs swapping. It could be
        // one of its children. So we probably need to recurse through that network to find the broken point.

        // Let's try precalculating the tree under a node for all nodes.
        // We could try doing a tree - we're guaranteed for that to be enough connections.
        // Then we can traverse the tree to

        // Or just do it with hash map. Tree does mean moving around the tree for all the checks.

        let z_wires: Vec<String> = self
            .wires
            .keys()
            .filter(|wire| wire.starts_with('z'))
            .sorted()
            .map(|s| s.to_owned())
            .collect();
        let mut expected_xy_deps = vec![];
        let mut wires = self.wires.clone();
        let mut all_deps: HashMap<String, _> = self
            .wires
            .keys()
            .filter_map(|wire| CrossedWires::sources_of(&wires, wire).map(|sources| (wire.to_string(), sources)))
            .collect();
        let xydeps = |deps: &HashMap<String, HashSet<String>>, s: &String| {
            deps
                .get(s)
                .unwrap()
                .into_iter()
                .filter(|wire| wire.starts_with('x') || wire.starts_with('y'))
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        };
        let mut swapped_wires: Vec<String> = vec![];

        for (z_idx, z_wire) in z_wires.iter().enumerate() {
            expected_xy_deps.extend(vec![format!("x{:#02}", z_idx), format!("y{:#02}", z_idx)]);
            // TODO: fix this option handling
            while expected_xy_deps.len() < xydeps(&all_deps, z_wire).len() {
                // Figure out where to swap wires
                let mut root = z_wire.to_string();
                'outer: loop {
                    {
                        let deps = CrossedWires::deps_1(&wires, &root).unwrap().clone();
                        if deps.len() < 2 {
                            panic!("Infinite loop - got to a const value!");
                        }

                        // If NEITHER set contains too many signals, we've traversed into the right place.
                        // Swap root.
                        // If one dep contains a bad thing, set root to it and continue.
                        for (idx, set) in deps.iter().enumerate() {
                            // I can't use length here! I have to check if any xy wires have an index beyond the allowed
                            let xy_of_set = xydeps(&all_deps, set);
                            if xy_of_set
                                .iter()
                                .any(|wire| !expected_xy_deps.contains(wire))
                            {
                                root = deps[idx].to_string();
                                continue 'outer;
                            }
                        }
                    }

                    // Neither dep contains the nasty thing - that means we swap root.
                    // Find the right node to swap the root with.
                    // We're looking for a node that only has these xydeps that ISN'T in our current search.
                    // Convert to hashset for comparison?
                    let root_deps = all_deps.get(z_wire).unwrap();
                    let expected: HashSet<String> = HashSet::from_iter(expected_xy_deps.iter().map(|s| s.to_string()));
                    let (other_key, _) = all_deps.iter().find(|(key, deps)| {
                        let wire_not_already_used = !root_deps.contains(*key);
                        let set_xy = deps
                            .into_iter()
                            .filter(|wire| wire.starts_with('x') || wire.starts_with('y'))
                            .map(|s| s.to_string())
                            .collect::<HashSet<_>>();
                        let matches_expected = set_xy == expected;
                        wire_not_already_used && matches_expected
                    }).unwrap();

                    // Do the swap. Need to update wires and all_deps, plus store these two to be returned
                    swapped_wires.push(other_key.to_string());
                    swapped_wires.push(root.to_string());
                    {
                        let root_contents = wires.get(&root).unwrap().clone();
                        let other_contents = wires.get(other_key).unwrap().clone();
                        wires.insert(root, other_contents);
                        wires.insert(other_key.to_string(), root_contents);
                    }

                    all_deps = wires
                        .keys()
                        .filter_map(|wire| CrossedWires::sources_of(&wires, wire).map(|sources| (wire.to_string(), sources)))
                        .collect();

                    break 'outer;
                }
            }
        }
        //     let mut actual_deps = all_deps.get(z_wire).expect("Could not get z wire");
        //     while actual_deps.len() != expected_deps.len() {
        //         loop {
        //             let root_deps = CrossedWires::deps_1(&wires, root).unwrap();
        //             for deps in root_deps {}
        //             // let left_deps = CrossedWires::deps_1(&wires, wire)
        //             // let left_deps = wires.get(z_wire).unwrap();
        //             break;
        //         }
        //     }
        //     println!("deps for idx {} are {:?}", z_idx, expected_deps);
        // }

        // let candidates: HashSet<_> = bad_wires
        //     .iter()
        //     .filter_map(|wire| self.sources_of(wire))
        //     .flatten()
        //     // .chain(bad_wires.iter())
        //     .collect();
        // if candidates.contains(&&"z00".to_string()) && candidates.contains(&&"z01".to_string()) && candidates.contains(&&"z02".to_string()) && candidates.contains(&&"z05".to_string()) {
        //     println!("Candidates has the right entries");
        // }
        // let mut result = match self.search_from_depth(depth_limit, &candidates, vec![], 0, z_correct) {
        //     Some(x) => x,
        //     None => return None,
        // };
        // result.sort_unstable();
        // Some(result.iter().join(","))

        // println!("x + y = z; {:?} + {:?} = {}", x, y, z_correct);
        // println!("Correct: {:#048b} ({:#016})", z_correct, z_correct);
        // println!("Actual : {:#048b} ({:#016})", z_actual, z_actual);
        // println!("Diff   : {:#048b} ({:#016})", diff, diff);
        swapped_wires.sort_unstable();
        Some(swapped_wires.iter().join(","))
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let wires = CrossedWires::from_str(puzzle).expect("Error parsing puzzle");
    println!("Part A: {:?}", wires.part_a());
    println!("Part B: {:?}", wires.part_b(8));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_small_txt() {
        let puzzle = include_str!("../puzzle/test_small.txt");
        let wires = CrossedWires::from_str(puzzle).unwrap();
        assert_eq!(Some(4), wires.part_a());
    }

    #[test]
    fn test_test_large_txt() {
        let puzzle = include_str!("../puzzle/test_large.txt");
        let wires = CrossedWires::from_str(puzzle).unwrap();
        assert_eq!(Some(2024), wires.part_a());
    }

    #[test]
    fn test_test_part_b_txt() {
        let puzzle = include_str!("../puzzle/test_part_b.txt");
        let wires = CrossedWires::from_str(puzzle).unwrap();
        assert_eq!(Some("z00,z01,z02,z05".to_string()), wires.part_b(4));
    }
}
