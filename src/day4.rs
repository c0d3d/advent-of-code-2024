use crate::grid::Direction;
use crate::grid::Grid;
use crate::grid::Point;
use crate::grid::DOWN;
use crate::grid::DOWN_LEFT;
use crate::grid::DOWN_RIGHT;
use crate::grid::LEFT;
use crate::grid::RIGHT;
use crate::grid::UP;
use crate::grid::UP_LEFT;
use crate::grid::UP_RIGHT;

const INPUT_1: &'static str = include_str!("day4-1.txt");
const TGT_STR: [char; 4] = ['X', 'M', 'A', 'S'];
const P2_TGT_STR: [char; 3] = ['M', 'A', 'S'];

const SEARCH_DIRECTIONS: [Direction; 8] = [
    UP_LEFT, UP, UP_RIGHT, LEFT, RIGHT, DOWN_LEFT, DOWN, DOWN_RIGHT,
];

fn eq_char(c: char) -> impl Fn(&Grid<char>, &Point) -> bool {
    return move |g, p| {
        return if let Some(actual) = g.at(p) {
            *actual == c
        } else {
            false
        };
    };
}

pub fn run_p1() {
    let g: Grid<char> = INPUT_1.parse().unwrap();
    let mut total = 0;
    for x_start in g.search(eq_char('X')) {
        for search_dir in SEARCH_DIRECTIONS.iter() {
            if g.match_direction(&x_start, &search_dir, &TGT_STR) {
                total += 1;
            }
        }
    }

    println!("Day 4, Part 1: {}", total);
}

pub fn run_p2() {
    let g: Grid<char> = INPUT_1.parse().unwrap();
    let mut total = 0;

    for x_start in g.search(eq_char('A')) {
        // MAS Start top left
        let down_right = g.match_direction(&x_start.add(&UP_LEFT), &DOWN_RIGHT, &P2_TGT_STR);
        let up_left = g.match_direction(&x_start.add(&DOWN_RIGHT), &UP_LEFT, &P2_TGT_STR);
        let down_left = g.match_direction(&x_start.add(&UP_RIGHT), &DOWN_LEFT, &P2_TGT_STR);
        let up_right = g.match_direction(&x_start.add(&DOWN_LEFT), &UP_RIGHT, &P2_TGT_STR);

        if (down_right || up_left) && (up_right || down_left) {
            total += 1;
        }
    }

    println!("Day 4, Part 2: {}", total);
}
