use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;

struct LANParty {
    network: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
enum LANPartyParseErr {
    DashNotFound,
}

impl FromStr for LANParty {
    type Err = LANPartyParseErr;
    fn from_str(puzzle: &str) -> Result<Self, Self::Err> {
        let mut network: HashMap<String, Vec<String>> = HashMap::new();
        for line in puzzle.lines() {
            let mut tokens = line.split("-");
            let (left, right) = match (tokens.next(), tokens.next()) {
                (Some(l), Some(r)) => (l.to_string(), r.to_string()),
                _ => return Err(LANPartyParseErr::DashNotFound),
            };
            network.entry(left.clone()).or_default().push(right.clone());
            network.entry(right).or_default().push(left);
        }
        Ok(Self { network })
    }
}

impl LANParty {
    fn part_a(&self) -> usize {
        let mut triples = HashSet::new();
        for (key, val) in self.network.iter() {
            for root in val.iter() {
                for other in val.iter() {
                    if root == other {
                        continue;
                    }
                    let conns = match self.network.get(root) {
                        Some(conns) => conns,
                        None => continue,
                    };
                    if conns.contains(other) {
                        let mut triple = vec![key, root, other];
                        triple.sort_unstable();
                        triples.insert(triple);
                    }
                }
            }
        }

        triples
            .iter()
            .filter(|triple| triple.iter().any(|el| el.starts_with('t')))
            .count()
    }

    fn part_b(&self) -> Option<String> {
        let mut full_networks: HashSet<Vec<&String>> = HashSet::new();
        for (key, val) in self.network.iter() {
            'outer: for candidate_network in val.iter().powerset() {
                if candidate_network.len() < 2 {
                    continue;
                }
                // Check interconnection
                for root in &candidate_network {
                    for other in &candidate_network {
                        if root == other {
                            continue;
                        }
                        if !self.network.get(*root).unwrap().contains(other) {
                            continue 'outer;
                        }
                    }
                }
                // Got to here? Network is okay, add it
                let mut full_net: Vec<_> = vec![key]
                    .into_iter()
                    .chain(candidate_network.into_iter())
                    .collect();
                full_net.sort_unstable();
                full_networks.insert(full_net);
            }
        }

        full_networks
            .iter()
            .map(|net| (net, net.len()))
            .max_by(|(_, l), (_, r)| l.cmp(r))
            .map(|(n, _)| n.iter().join(","))
    }

    fn part_b_alt(&self) -> String {
        let mut biggest_net = vec![];
        for (key, val) in self.network.iter() {
            'outer: for candidate_network in val.iter().powerset() {
                if candidate_network.len() < 2.max(biggest_net.len().saturating_sub(1)) {
                    continue;
                }
                // Check interconnection
                for root in &candidate_network {
                    for other in &candidate_network {
                        if root == other {
                            continue;
                        }
                        if !self.network.get(*root).unwrap().contains(other) {
                            continue 'outer;
                        }
                    }
                }
                // Got to here? Network is okay, add it
                let mut full_net: Vec<_> = vec![key]
                    .into_iter()
                    .chain(candidate_network.into_iter())
                    .collect();
                full_net.sort_unstable();
                biggest_net = full_net;
            }
        }
        biggest_net.iter().join(",")
    }
}

fn main() {
    let puzzle = include_str!("../puzzle/input.txt");
    let party = LANParty::from_str(puzzle).expect("Could not parse puzzle");
    println!("Part A: {}", party.part_a());
    println!("Part B: {:?}", party.part_b());
    println!("Part B: {:?}", party.part_b_alt());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_txt_part_a() {
        let puzzle = include_str!("../puzzle/test.txt");
        let party = LANParty::from_str(puzzle).unwrap();
        assert_eq!(7, party.part_a());
    }

    #[test]
    fn test_test_txt_part_b() {
        let puzzle = include_str!("../puzzle/test.txt");
        let party = LANParty::from_str(puzzle).unwrap();
        assert_eq!(Some("co,de,ka,ta".to_string()), party.part_b());
    }

    #[test]
    fn test_test_txt_part_b_alt() {
        let puzzle = include_str!("../puzzle/test.txt");
        let party = LANParty::from_str(puzzle).unwrap();
        assert_eq!("co,de,ka,ta".to_string(), party.part_b_alt());
    }
}
