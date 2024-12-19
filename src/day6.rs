use std::collections::HashSet;
use std::str::FromStr;

use crate::grid::Direction;
use crate::grid::Grid;
use crate::grid::Point;
use crate::grid::DOWN;
use crate::grid::LEFT;
use crate::grid::RIGHT;
use crate::grid::UP;

const INPUT_1: &'static str = include_str!("day6-1.txt");

#[derive(Eq, PartialEq)]
enum Loc {
    Guard(&'static Direction),
    Obstruction,
    Empty,
}

impl Loc {
    fn is_guard(&self) -> bool {
        if let Loc::Guard(_) = self {
            true
        } else {
            false
        }
    }

    fn direction(&self) -> Option<&'static Direction> {
        match self {
            Loc::Guard(x) => Some(x),
            Loc::Obstruction | Loc::Empty => None,
        }
    }
}

impl FromStr for Loc {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err("Too long or too short!");
        }

        return Ok(match s.chars().nth(0).unwrap() {
            '^' => Loc::Guard(&UP),
            '>' => Loc::Guard(&RIGHT),
            '<' => Loc::Guard(&LEFT),
            'V' => Loc::Guard(&DOWN),
            '#' => Loc::Obstruction,
            '.' => Loc::Empty,
            _ => return Err("Unrecognized!"),
        });
    }
}

fn guard(g: &Grid<Loc>, p: &Point) -> bool {
    return g.at(p).map_or(false, Loc::is_guard);
}

pub fn run_p1() {
    let g: Grid<Loc> = INPUT_1.parse().unwrap();

    let mut guard_pos = g.search(guard).nth(0).expect("A Guard");
    let mut guard_dir = g.at(&guard_pos).unwrap().direction().unwrap();
    let mut visited_posns = HashSet::new();

    'full_search: loop {
        visited_posns.insert(guard_pos.clone());
        let mut potential_nxt = guard_pos.add(guard_dir);

        // A corner means we might need to turn a few times.
        // In a real solution I'd check that we aren't fully boxed in
        // But this only requires solving for a single input where that isn't the case
        loop {
            match g.at(&potential_nxt) {
                Some(Loc::Obstruction) => {
                    // Turn and (try) move forward
                    guard_dir = guard_dir.rotate90();
                    potential_nxt = guard_pos.add(guard_dir);
                },
                Some(Loc::Guard(_)) | Some(Loc::Empty) => {
                    guard_pos = potential_nxt.clone();
                    // Found a spot so we break inner loop.
                    break;
                },
                None => {
                    break 'full_search;
                },
            }
        }
    }

    println!("Day 6, Part 1: {}", visited_posns.len());
}

pub fn run_p2() {}
