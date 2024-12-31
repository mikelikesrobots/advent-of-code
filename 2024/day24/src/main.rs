use std::{
    collections::{HashMap, HashSet}, fs::File, io::{BufWriter, Write}, str::FromStr
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
    fn value(wires: &HashMap<String, WireSource>, wire: &String) -> Option<u64> {
        CrossedWires::value_inner(wires, wire, &mut HashSet::new())
    }
    
    fn value_inner(wires: &HashMap<String, WireSource>, wire: &String, visited: &mut HashSet<String>) -> Option<u64> {
        if visited.contains(wire) {
            return None;
        }
        visited.insert(wire.to_string());
        match wires.get(wire) {
            None => None,
            Some(WireSource::Const(x)) => Some(*x),
            Some(WireSource::And(l, r)) => CrossedWires::value_inner(wires, l, visited).and_then(|l| CrossedWires::value_inner(wires, r, visited).map(|r| l & r)),
            Some(WireSource::Xor(l, r)) => CrossedWires::value_inner(wires, l, visited).and_then(|l| CrossedWires::value_inner(wires, r, visited).map(|r| l ^ r)),
            Some(WireSource::Or(l, r)) => CrossedWires::value_inner(wires, l, visited).and_then(|l| CrossedWires::value_inner(wires, r, visited).map(|r| l | r)),
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

    fn convert_starting_with(wires: &HashMap<String, WireSource>, start: char) -> Option<u64> {
        let mut keys: Vec<&String> = wires
            .keys()
            .filter(|key| key.starts_with(start))
            .collect();
        keys.sort_unstable();
        keys.reverse();

        keys.iter()
            .try_fold(0, |acc, gate| CrossedWires::value(&wires, gate).map(|val| (acc << 1) + val))
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
                if CrossedWires::convert_starting_with(&wire_sources, 'z').iter()
                .any(|z| *z == correct_result) {
                    return Some(next_acc.to_vec());
                }
            }
        }
        None
    }

    fn part_a(&self) -> Option<u64> {
        // Find all the keys starting with z
        CrossedWires::convert_starting_with(&self.wires, 'z')
    }

    fn part_b(&self, n_permutations: usize) -> Option<String> {
        // let x = self.convert_starting_with('x');
        // let y = self.convert_starting_with('y');
        // let z_correct = x.and_then(|x| y.map(|y| y + x));
        // let z_actual = self.convert_starting_with('z');

        // let (z_correct, z_actual) = match (z_correct, z_actual) {
        //     (Some(correct), Some(actual)) => (correct, actual),
        //     _ => return None,
        // };
        // let diff = z_correct ^ z_actual;

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

        // Can I get x, y, z value from the set? Maybe a set as well?
        // Let's just try with only the initial input as the way to do it.

        let z_wires: Vec<String> = self
            .wires
            .keys()
            .filter(|wire| wire.starts_with('z'))
            .sorted()
            .map(|s| s.to_owned())
            .collect();
        let swappables: Vec<String> = self
            .wires
            .iter()
            .filter(|(_, source)| matches!(source, WireSource::Const(_)))
            .map(|(name, _)| name.to_owned())
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
                .iter()
                .filter(|wire| wire.starts_with('x') || wire.starts_with('y'))
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        };
        let is_xy = |s: &String| s.starts_with('x') || s.starts_with('y');
        let contains_all = |allowed: &Vec<String>, test: &Vec<String>| {
            test.iter().all(|val| allowed.contains(val))
        };
        let mut swapped_wires: Vec<String> = vec![];

        for (z_idx, z_wire) in z_wires.iter().enumerate() {
            expected_xy_deps.extend(vec![format!("x{:#02}", z_idx), format!("y{:#02}", z_idx)]);
            while !contains_all(&expected_xy_deps, &xydeps(&all_deps, z_wire)) {
                // Figure out where to swap wires
                let mut root = z_wire.to_string();
                'outer: loop {
                    {
                        // For current node, check what gates it depends on
                        let deps = CrossedWires::deps_1(&wires, &root).unwrap().clone();
                        let (gates, wires): (Vec<_>, Vec<_>) = deps.iter().partition(|v| !is_xy(v));
                        // Are any wires not allowed? If not, swap this node

                        // Otherwise, go until you reach the point with not allowed wires
                        if deps.len() < 2 {
                            panic!("Infinite loop - got to a const value!");
                        }

                        // If NEITHER set contains too many signals, we've traversed into the right place.
                        // Swap root.
                        // If one dep contains a bad thing, set root to it and continue.
                        for (idx, set) in deps.iter().enumerate() {
                            if !contains_all(&expected_xy_deps, &xydeps(&all_deps, set)) {
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
                            .iter()
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

    fn contains_all(expected_x_deps: &[String], actual_x_deps: &[String]) -> bool {
        actual_x_deps.iter().all(|wire| expected_x_deps.contains(wire))
    }

    fn x_deps(wires: &HashMap<String, WireSource>, wire: &String) -> Vec<String> {
        CrossedWires::sources_of(wires, wire).unwrap().into_iter().filter(|wire| wire.starts_with('x')).collect()
    }

    fn net_value(wire: &str) -> Option<u32> {
        let digits: String = wire.chars().filter(|c| c.is_ascii_digit()).collect();
        if digits.is_empty() {
            return None;
        }
        let net = digits.parse::<u32>().unwrap();
        Some(net)
    }

    fn find_swap_wire(wires: &HashMap<String, WireSource>, banned_deps: &[String], expected_x_deps: &[String], min_net: u32) -> Option<String> {
        println!("BANNED DEPS: {:?}", banned_deps);
        // Find the wire that is contained by expected_x_deps but is NOT in banned_deps
        let matching: Vec<_> = wires.keys()
            .filter(|wire| !banned_deps.contains(wire))
            .filter(|wire| !wire.starts_with('y') && !wire.starts_with('x'))
            .filter(|wire| CrossedWires::contains_all(expected_x_deps, &CrossedWires::x_deps(wires, wire)))
            .filter(|wire| CrossedWires::net_value(wire).unwrap() >= min_net)
            .collect();
        println!("ALL MATCHING: {:?}", matching);
        assert_eq!(matching.len(), 1, "Too few/many wires found for swap!");
        Some(matching[0].to_string())
    }

    // Apparently, z0-zn are completely okay. They might have the right dependencies, but the wrong values.
    // That means wires inside each of zn are mixed up.
    // Maybe we're thinking about this wrong.
    // We know how a binary adder works.
    // We can supply x0, y0 and test if z0 is the right answer.
    // How does the adder work again?
    // if x & y are 1, we need to carry one.
    // Otherwise, we xor.

    // z0 = x0 ^ y0
    // z1 = (x1 ^ y1)

    // 00 + 10 -> 1
    // 10 + 00 -> 1
    // 10 + 10 -> 0
    // 11 + 11 -> 1

    fn get_swap(wires: &HashMap<String, WireSource>, z_wire: &String, expected_x_deps: &[String]) -> Option<(String, String)> {
        if !z_wire.starts_with('z') {
            return None;
        }

        // No need to swap if x deps are entirely contained
        let x_deps_of_z = CrossedWires::x_deps(wires, z_wire);
        let mut banned_x_deps: Vec<_> = x_deps_of_z.iter().filter(|x| !expected_x_deps.contains(x)).map(|s| s.to_string()).collect();
        if banned_x_deps.is_empty() {
            return None;
        }
        banned_x_deps.sort_unstable();
        let all_deps_of_z = Vec::from_iter(CrossedWires::sources_of(wires, z_wire).unwrap());
        let z_net = CrossedWires::net_value(z_wire).unwrap();

        // At least one swap is needed! Traverse the tree from the root until we located the bad node.
        let mut current = z_wire.clone();
        // We can't find swap wire to the left of where we are - but how do we say that?
        // A separate search space we prune as find more things?
        let swap_target = CrossedWires::find_swap_wire(wires, &all_deps_of_z, expected_x_deps, z_net).unwrap();
        loop {
            // Find the level 1 deps of the wire.
            let lvl1deps = CrossedWires::deps_1(wires, &current).unwrap();
            // Do we only have wires? If so, this needs to be swapped.
            if lvl1deps.iter().filter(|wire| wire.starts_with('x')).count() > 0 {
                return Some((current, swap_target));
            }

            // Check which node introduces the wires we don't want.
            let left = lvl1deps[0].clone();
            let left_x_deps = CrossedWires::x_deps(wires, &left);
            if CrossedWires::contains_all(&left_x_deps, &banned_x_deps) {
                current = left;
                continue;
            }
            let right = lvl1deps[1].clone();
            let right_x_deps = CrossedWires::x_deps(wires, &right);
            if CrossedWires::contains_all(&right_x_deps, &banned_x_deps) {
                current = right;
                continue;
            }

            // Neither node suitable for traversal; return!
            return Some((current, swap_target));
        }
    }

    fn part_b_alt(&self) -> Option<String> {
        let mut swapped_wires: Vec<String> = vec![];
        let mut updated_wires = self.wires.clone();

        // Get an ordered list of z wires to check through
        let z_wires: Vec<String> = self
            .wires
            .keys()
            .filter(|wire| wire.starts_with('z'))
            .sorted()
            .map(|s| s.to_owned())
            .collect();

        // For each, check if there is a swap to do
        let mut expected_x_deps = vec![];
        for (z_idx, z_wire) in z_wires.iter().enumerate() {
            expected_x_deps.push(format!("x{:#02}", z_idx));
            while let Some((left, right)) = CrossedWires::get_swap(&updated_wires, z_wire, &expected_x_deps) {
                // Track the swapped wires
                swapped_wires.push(left.clone());
                swapped_wires.push(right.clone());
                // Swap within updated wires
                let left_contents = updated_wires.get(&left).unwrap().clone();
                let right_contents = updated_wires.get(&right).unwrap().clone();
                updated_wires.insert(left, right_contents);
                updated_wires.insert(right, left_contents);
            }
        }

        swapped_wires.sort_unstable();
        Some(swapped_wires.iter().join(","))
    }

    // z0 = x0 ^ y0
    // z1 = (x1 ^ y1)

    // 00 + 10 -> 1
    // 10 + 00 -> 1
    // 10 + 10 -> 0
    // 11 + 11 -> 1
    fn part_b_alt2(&self) -> Option<String> {
        // Find the bits which don't pass the test cases
        let no_carry_cases = vec![
            ((0b0, 0b1), 0b1),
            ((0b1, 0b0), 0b1),
            ((0b1, 0b1), 0b0),
        ];
        let carry_cases = vec![
            ((0b00, 0b10), 0b1),
            ((0b10, 0b00), 0b1),
            ((0b10, 0b10), 0b0),
            ((0b11, 0b11), 0b1),
        ];
        let z_wires: Vec<String> = self
            .wires
            .keys()
            .filter(|wire| wire.starts_with('z'))
            .sorted()
            .map(|s| s.to_owned())
            .collect();
        let max_idx: u32 = z_wires.iter().max().unwrap().chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap();
        println!("max idx: {}", max_idx);
        let set_char = |c: char, val: i64, wires: &mut HashMap<String, WireSource>| {
            (0..max_idx).for_each(|idx| {
                // Get the bit from x
                let val = ((val >> idx) & 0b1) as u64;
                let wire = format!("{}{:#02}", c, idx);
                wires.insert(wire, WireSource::Const(val));
            })
        };
        let test_adder_bit_correct = |idx: usize, wire: &String, wires: &mut HashMap<String, WireSource>| {
            let (cases, right_shift) = match idx {
                0 => (&no_carry_cases, 0),
                _ => (&carry_cases, 1),
            };

            cases.iter().all(|((x, y), z)| {
                let x_val = (x << idx) >> right_shift;
                let y_val = (y << idx) >> right_shift;
                set_char('x', x_val, wires);
                set_char('y', y_val, wires);
                match CrossedWires::value(wires, wire) {
                    Some(z_val) => z_val == *z,
                    None => false,
                }
            })
        };
        
        // My test cases are not sufficient. I keep testing that it becomes correct
        // when it isn't correct. Is there something else I can do?
        // 
        let mut swapped_wires: Vec<String> = vec![];
        
        let mut wires = self.wires.clone();

        'outer: for (z_idx, z_wire) in z_wires.iter().enumerate() {
            if test_adder_bit_correct(z_idx, z_wire, &mut wires) {
                continue;
            }
            println!("{} failed test cases!", z_wire);

            // Figure out swaps by trying all switched pairs within dependencies of z_wire.
            // let raw_deps = CrossedWires::sources_of(&wires, z_wire).unwrap();
            // println!("{} has raw deps {:?}", z_wire, raw_deps);
            // let deps: Vec<String> = raw_deps.iter().filter(|wire| !wire.starts_with('x') && !wire.starts_with('y')).map(|s| s.to_string()).collect();
            // println!("{} has deps {:?}", z_wire, deps);
            // let keys = wires.iter().filter(|(_, val)| !matches!(val, WireSource::Const(_))).map(|(key, _)| key.to_string()).collect::<Vec<_>>();
            let keys = wires.keys().map(|key| key.to_string()).filter(|key| !swapped_wires.contains(key)).collect::<Vec<_>>();
            for swap_deps in keys.iter().permutations(2) {
                let left = swap_deps[0].to_string();
                let right = swap_deps[1].to_string();
                let left_val = wires.get(&left).unwrap().clone();
                let right_val = wires.get(&right).unwrap().clone();
                // Swap the values
                wires.insert(left.clone(), right_val.clone());
                wires.insert(right.clone(), left_val.clone());
                // Test the new values
                if test_adder_bit_correct(z_idx, z_wire, &mut wires) {
                    println!("Corrected by swapping {} and {}!", &left, &right);
                    swapped_wires.push(left);
                    swapped_wires.push(right);
                    continue 'outer;
                }
                // Swap the values back!
                wires.insert(left.clone(), left_val);
                wires.insert(right.clone(), right_val);
                // println!("Swapping {} and {} didn't work!", left, right);
            }
            println!("Didn't find a swap to make it work.");
            return None;
        }

        swapped_wires.sort_unstable();
        Some(swapped_wires.iter().join(","))
    }


    fn part_b_alt3(&self) -> Option<String> {
        // So the idea is to test from bit 0 up to bit n. For each, swap two wires until the test cases pass.
        // Very similar to alt2, only with different test cases and methods of testing.

        // There are many possibilities for each wire.
        // The top end won't be fixed until the bottom end is. Probably.
        // We don't know that for sure, but we should continue as if it is true.
        // Actually, a gap in the right place would make it work correctly again.
        // Then I only need to check wire swaps for that index - 1.
        // Maybe I could try that?

        let z_wires: Vec<String> = self
            .wires
            .keys()
            .filter(|wire| wire.starts_with('z'))
            .sorted()
            .map(|s| s.to_owned())
            .collect();
        let max_idx: u32 = z_wires.iter().max().unwrap().chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap();
        println!("max idx: {}", max_idx);

        let set_char = |c: char, val: i64, wires: &mut HashMap<String, WireSource>| {
            (0..max_idx).for_each(|idx| {
                // Get the bit from x
                let val = ((val >> idx) & 0b1) as u64;
                let wire = format!("{}{:#02}", c, idx);
                wires.insert(wire, WireSource::Const(val));
            })
        };
        let test_adder_bit_correct = |idx: usize, wire: &String, wires: &mut HashMap<String, WireSource>| {
            // Shift 1s left by idx + 1, then bit flip
            //  e.g. idx = 3; Want mask 1111 to get bottom bits;
            //  11111 << 4 = 10000; flip for 01111
            let xymask = !(!0 << (idx + 1));

            // Test that x 0s and y 1s gives z 0 followed by 1s.
            let x_val = 0;
            let y_val = !0 & xymask;
            let _expected_z = !0 & xymask;
            set_char('x', x_val, wires);
            set_char('y', y_val, wires);

            match CrossedWires::value(wires, wire) {
                // actual value is equal to expected value
                Some(_expected_z) => (),
                // None or actual value is different to expected value
                _ => return false,
            };

            // Repeat 0s and 1s for y, x
            let x_val = y_val;
            let y_val = 0;
            set_char('x', x_val, wires);
            set_char('y', y_val, wires);
            match CrossedWires::value(wires, wire) {
                // actual value is equal to expected value
                Some(_expected_z) => (),
                // None or actual value is different to expected value
                _ => return false,
            };

            // Check if carry bit is calculated correctly
            let x_val = xymask;
            let y_val = xymask;
            let _expected_z = 1 << (idx + 1);
            set_char('x', x_val, wires);
            set_char('y', y_val, wires);
            match CrossedWires::value(wires, wire) {
                // actual value is equal to expected value
                Some(_expected_z) => (),
                // None or actual value is different to expected value
                _ => return false,
            };

            true
        };

        let mut swapped_wires: Vec<String> = vec![];
        let mut swap_cands: Vec<(String, String)> = vec![];
        let mut wires = self.wires.clone();

        let mut swap_cands: HashSet<Vec<String>> = HashSet::new();
        let mut current_swap_cands: HashSet<Vec<String>> = HashSet::new();

        for (z_idx, z_wire) in z_wires.iter().enumerate() {
            if test_adder_bit_correct(z_idx, z_wire, &mut wires) {
                continue;
            }
            println!("{} failed test cases!", z_wire);

            current_swap_cands.clear();

            // let keys = wires.keys().map(|key| key.to_string()).filter(|key| !swapped_wires.contains(key)).collect::<Vec<_>>();
            let keys = wires
                .iter()
                .filter(|(key, _)| !key.starts_with('x'))
                .filter(|(key, _)| !key.starts_with('y'))
                .filter(|(key, _)| !key.starts_with('z'))
                .filter(|(_, val)| !matches!(val, WireSource::Const(_)))
                .map(|(key, _)| key.to_string())
                .collect::<Vec<_>>();
            // let keys = wires.keys().map(|key| key.to_string()).collect::<Vec<_>>();
            for swap_deps in keys.iter().permutations(2) {
                let left = swap_deps[0].to_string();
                let right = swap_deps[1].to_string();
                let left_val = wires.get(&left).unwrap().clone();
                let right_val = wires.get(&right).unwrap().clone();
                // Swap the values
                wires.insert(left.clone(), right_val.clone());
                wires.insert(right.clone(), left_val.clone());
                // Test the new values
                let success = test_adder_bit_correct(z_idx, z_wire, &mut wires);
                // Swap the values back!
                wires.insert(left.clone(), left_val);
                wires.insert(right.clone(), right_val);

                // Did the swap work? If so, store it for next iteration
                if success {
                    println!("New swap candidate found: {}, {}", left, right);
                    let mut swap_cand = vec![left.to_string(), right.to_string()];
                    swap_cand.sort_unstable();
                    current_swap_cands.insert(swap_cand);
                }
            }

            swap_cands = swap_cands.intersection(&current_swap_cands).cloned().collect();

            // Do we have only 1 candidate? Use that one!
            if swap_cands.len() == 1 {
                swap_cands.iter().for_each(|cand| {
                    let left = cand[0].to_string();
                    let right = cand[1].to_string();
                    println!("Corrected by swapping {} and {}!", &left, &right);
                    let left_val = wires.get(&left).unwrap().clone();
                    let right_val = wires.get(&right).unwrap().clone();
                    // Swap the values
                    wires.insert(left.clone(), right_val.clone());
                    wires.insert(right.clone(), left_val.clone());
                    swapped_wires.push(left);
                    swapped_wires.push(right);
                });
                swap_cands.clear();
            }

            println!("Didn't find a swap to make it work.");
            return None;
        }

        swapped_wires.sort_unstable();
        Some(swapped_wires.iter().join(","))
    }

    fn part_b_alt4(&self, n_permutations: usize) -> Option<String> {
        
        let z_wires: Vec<String> = self
            .wires
            .keys()
            .filter(|wire| wire.starts_with('z'))
            .sorted()
            .map(|s| s.to_owned())
            .collect();
        let swappables: Vec<String> = self
            .wires
            .iter()
            .filter(|(_, source)| !matches!(source, WireSource::Const(_)))
            .map(|(name, _)| name.to_owned())
            .collect();
        let swap = |left: &String, right: &String, wires: &mut HashMap<String, WireSource>| {
            let left_val = wires.get(left).unwrap().clone();
            let right_val = wires.get(right).unwrap().clone();
            wires.insert(left.clone(), right_val);
            wires.insert(right.clone(), left_val);
        };
        let mut wires = self.wires.clone();

        let x = CrossedWires::convert_starting_with(&wires, 'x').unwrap();
        let y = CrossedWires::convert_starting_with(&wires, 'y').unwrap();
        let z_correct = x + y;

        for perms in swappables.iter().permutations(n_permutations) {
            for mut wire_pair in &perms.iter().chunks(2) {
                // Swap
                swap(wire_pair.next().unwrap(), wire_pair.next().unwrap(), &mut wires);
            }

            // Test
            let z_test = CrossedWires::convert_starting_with(&wires, 'z').unwrap();
            if z_correct == z_test {
                return Some(perms.iter().sorted().join(","));
            }

            for mut wire_pair in &perms.iter().chunks(2) {
                // Swap back
                swap(wire_pair.next().unwrap(), wire_pair.next().unwrap(), &mut wires);
            }
        }

        None
    }

    // https://www.reddit.com/r/adventofcode/comments/1hl698z/comment/m3v5dfv
    fn visualize(&self) -> Result<String, std::io::Error>{
        let f = File::create("day24.dot")?;
        let mut out = BufWriter::new(f);
        writeln!(out, "digraph {{")?;
        writeln!(out, "  rankdir=\"LR\";")?;
        writeln!(out, "  node [style=filled];")?;

        for (wire, source) in self.wires.iter() {
            match source {
                // Ignore consts
                WireSource::Const(_) => (),
                WireSource::And(left, right) => {
                    writeln!(out, "  {wire} -> {left};")?;
                    writeln!(out, "  {wire} -> {right};")?;
                    writeln!(out, "  {wire} [label=\"{wire} (AND)\"];")?;    
                    writeln!(out, "  {wire} [color=\"red\"];")?;
                },
                WireSource::Or(left, right) => {
                    writeln!(out, "  {wire} -> {left};")?;
                    writeln!(out, "  {wire} -> {right};")?;
                    writeln!(out, "  {wire} [label=\"{wire} (OR)\"];")?;    
                    writeln!(out, "  {wire} [color=\"blue\"];")?;
                },
                WireSource::Xor(left, right) => {
                    writeln!(out, "  {wire} -> {left};")?;
                    writeln!(out, "  {wire} -> {right};")?;
                    writeln!(out, "  {wire} [label=\"{wire} (XOR)\"];")?;    
                    writeln!(out, "  {wire} [color=\"green\"];")?;
                },
            }
        }
        writeln!(out, "}}")?;

        let mut swapped_wires = vec![];
        let mut swap = |left: &str, right: &str, wires: &mut HashMap<String, WireSource>| {
            let left_val = wires.get(left).unwrap().clone();
            let right_val = wires.get(right).unwrap().clone();
            wires.insert(left.to_string(), right_val);
            wires.insert(right.to_string(), left_val);
            swapped_wires.push(left.to_string());
            swapped_wires.push(right.to_string());
        };
        let mut wires = self.wires.clone();
        swap("ggk", "rhv", &mut wires);
        swap("z20", "hhh", &mut wires);
        swap("z15", "htp", &mut wires);
        swap("z05", "dkr", &mut wires);

        let f = File::create("day24_swapped.dot")?;
        let mut out = BufWriter::new(f);
        writeln!(out, "digraph {{")?;
        writeln!(out, "  rankdir=\"LR\";")?;
        writeln!(out, "  node [style=filled];")?;

        for (wire, source) in wires.iter() {
            match source {
                // Ignore consts
                WireSource::Const(_) => (),
                WireSource::And(left, right) => {
                    writeln!(out, "  {wire} -> {left};")?;
                    writeln!(out, "  {wire} -> {right};")?;
                    writeln!(out, "  {wire} [label=\"{wire} (AND)\"];")?;    
                    writeln!(out, "  {wire} [color=\"red\"];")?;
                },
                WireSource::Or(left, right) => {
                    writeln!(out, "  {wire} -> {left};")?;
                    writeln!(out, "  {wire} -> {right};")?;
                    writeln!(out, "  {wire} [label=\"{wire} (OR)\"];")?;    
                    writeln!(out, "  {wire} [color=\"blue\"];")?;
                },
                WireSource::Xor(left, right) => {
                    writeln!(out, "  {wire} -> {left};")?;
                    writeln!(out, "  {wire} -> {right};")?;
                    writeln!(out, "  {wire} [label=\"{wire} (XOR)\"];")?;    
                    writeln!(out, "  {wire} [color=\"green\"];")?;
                },
            }
        }
        writeln!(out, "}}")?;

        let correct = swapped_wires.iter().sorted().join(",");
        Ok(correct)
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let wires = CrossedWires::from_str(puzzle).expect("Error parsing puzzle");
    println!("Part A: {:?}", wires.part_a());
    // println!("Part B: {:?}", wires.visualize());
    println!("Part B: {:?}", wires.part_b_alt4(8));
    // println!("{:?}", (0..4).permutations(3).collect::<Vec<_>>());
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
        assert_eq!(Some("z00,z01,z02,z05".to_string()), wires.part_b_alt());
    }

    #[test]
    fn test_test_ands_txt() {
        let puzzle = include_str!("../puzzle/test_ands.txt");
        let wires = CrossedWires::from_str(puzzle).unwrap();
        assert_eq!(Some("z00,z01".to_string()), wires.part_b_alt());
    }
}
