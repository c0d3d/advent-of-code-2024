use std::collections::{HashMap, HashSet};

const INPUT_1: &'static str = include_str!("day5-1.txt");

struct OrderRules {
    // Maps a particular page to all pages that must come before that one.
    rule_map: HashMap<u32, HashSet<u32>>,
    forward_rule_map: HashMap<u32, HashSet<u32>>,
}

impl OrderRules {
    fn new() -> OrderRules {
        return OrderRules {
            rule_map: HashMap::new(),
            forward_rule_map: HashMap::new(),
        };
    }

    fn topo(&self, pu: &PageUpdate) -> Option<PageUpdate> {
        if self.validate_update(pu) {
            return None;
        }

        let mut upd2 = pu.clone();
        while !self.validate_update(&upd2) {
            let mut seen: HashSet<u32> = HashSet::with_capacity(upd2.0.len());
            for idx in 0..upd2.0.len() {
                let dep_list = self.forward_rule_map.get(&upd2.0[idx]).unwrap();
                // Have we seen one's that I must be placed before?
                if !seen.is_disjoint(dep_list) {
                    // Find first one ...
                    let must_be_before = upd2.0.iter().position(|x| dep_list.contains(x)).unwrap();
                    let old = upd2.0.remove(idx);
                    upd2.0.insert(must_be_before, old);
                    break;
                } else {
                    seen.insert(upd2.0[idx]);
                }
            }
        }

        return Some(upd2);
    }

    fn add_rule(&mut self, before: u32, after: u32) {
        if let Some(pages) = self.rule_map.get_mut(&after) {
            pages.insert(before);
        } else {
            let mut h = HashSet::new();
            h.insert(before);
            self.rule_map.insert(after, h);
        }

        if let Some(pages) = self.forward_rule_map.get_mut(&before) {
            pages.insert(after);
        } else {
            let mut h = HashSet::new();
            h.insert(after);
            self.forward_rule_map.insert(before, h);
        }
    }

    fn validate_update(&self, upd: &PageUpdate) -> bool {
        let mut seen: HashSet<u32> = HashSet::new();
        for (_, nxt) in upd.iter().rev() {
            if let Some(found_illegal) = self
                .rule_map
                .get(&nxt)
                .map(|pages| pages.iter().any(|only_before| seen.contains(only_before)))
            {
                if found_illegal {
                    return false;
                }
            }
            seen.insert(*nxt);
        }
        return true;
    }
}

#[derive(Debug, Clone)]
struct PageUpdate(Vec<u32>);

impl PageUpdate {
    fn midpoint(&self) -> u32 {
        return self.0[self.0.len() / 2];
    }

    fn iter(&self) -> impl DoubleEndedIterator<Item = (usize, &u32)> + ExactSizeIterator + '_ {
        return self.0.iter().enumerate();
    }
}

impl FromStr for PageUpdate {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut u = PageUpdate(Vec::new());
        for n in s.split(',') {
            u.0.push(n.parse().map_err(|_| "A number")?);
        }

        return Ok(u);
    }
}

fn parse_input() -> Result<(OrderRules, Vec<PageUpdate>), &'static str> {
    let mut order = OrderRules::new();
    let mut updates = Vec::new();
    let mut passed_rules = false;
    for l in INPUT_1.lines() {
        // Empty line indicates swap to update reading mode.
        if l == "" {
            passed_rules = true;
            continue;
        }

        if !passed_rules {
            let (before, after) = {
                let mut i = l.split('|');
                (
                    i.next()
                        .ok_or("No before")?
                        .parse()
                        .map_err(|_| "Before bad")?,
                    i.next()
                        .ok_or("No after")?
                        .parse()
                        .map_err(|_| "After bad")?,
                )
            };
            order.add_rule(before, after);
        } else {
            updates.push(l.parse()?);
        }
    }

    return Ok((order, updates));
}

pub fn run_p1() {
    let mut total = 0;
    let (order, updates) = parse_input().expect("It to parse");
    for upd in updates.iter() {
        if let None = order.topo(upd) {
            total += upd.midpoint();
        }
    }

    println!("Day 5, Part 1: {}", total);
}

pub fn run_p2() {
    let mut total = 0;
    let (order, updates) = parse_input().expect("It to parse");
    for upd in updates.iter() {
        if let Some(new_order) = order.topo(upd) {
            total += new_order.midpoint();
        }
    }

    println!("Day 5, Part 2: {}", total);
}
