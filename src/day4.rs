use std::str::FromStr;

const INPUT_1: &'static str = include_str!("day4-1.txt");
const TGT_STR: [char; 4] = ['X', 'M', 'A', 'S'];
const P2_TGT_STR: [char; 3] = ['M', 'A', 'S'];

const DOWN_RIGHT: Direction = Direction(1, 1);
const UP_LEFT: Direction = Direction(-1, -1);
const DOWN_LEFT: Direction = Direction(-1, 1);
const UP_RIGHT: Direction = Direction(1, -1);

const SEARCH_DIRECTIONS: [Direction; 8] = [
    UP_LEFT,
    Direction(0, -1),
    UP_RIGHT,
    Direction(-1, 0),
    Direction(1, 0),
    DOWN_LEFT,
    Direction(0, 1),
    DOWN_RIGHT,
];

#[derive(Clone, Debug)]
struct Point(isize, isize);

struct Direction(isize, isize);

impl Point {
    fn add(&self, d: &Direction) -> Point {
        return Point(self.0 + d.0, self.1 + d.1);
    }
}

struct Grid {
    data: Vec<Vec<char>>,
}

struct GridSearch<'a> {
    g: &'a Grid,
    tgt: char,
    x: usize,
    y: usize,
}

impl<'a> Iterator for GridSearch<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let mut last_x = self.x;
        for sy in self.y..self.g.data.len() {
            for sx in last_x..self.g.data[sy].len() {
                if self.tgt == self.g.data[sy][sx] {
                    if sx + 1 == self.g.data[sy].len() {
                        self.x = 0;
                        self.y = sy + 1;
                    } else {
                        self.x = sx + 1;
                        self.y = sy;
                    }

                    return Some(Point(sx as isize, sy as isize));
                }
            }
            last_x = 0;
        }

        return None;
    }
}

impl Grid {
    fn search_char(&self, tgt: char) -> impl Iterator<Item = Point> + '_ {
        return GridSearch {
            g: self,
            x: 0,
            y: 0,
            tgt,
        };
    }

    fn match_direction(&self, start: &Point, dir: &Direction, to_match: &[char]) -> bool {
        let mut nxt_point = start.clone();
        for nxt_char in to_match.iter() {
            match self.at(&nxt_point) {
                Some(actual_char) if actual_char == *nxt_char => {
                    nxt_point = nxt_point.add(dir);
                }
                _ => {
                    return false;
                }
            }
        }

        return true;
    }

    fn at(&self, p: &Point) -> Option<char> {
        if p.1 < 0 || p.1 as usize >= self.data.len() {
            return None;
        }

        if p.0 < 0 || p.0 as usize >= self.data[p.1 as usize].len() {
            return None;
        }

        return Some(self.data[p.1 as usize][p.0 as usize]);
    }
}

impl FromStr for Grid {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = Vec::new();
        for line in s.lines() {
            let mut cols = Vec::new();
            for c in line.chars() {
                cols.push(c);
            }
            rows.push(cols);
        }
        return Ok(Grid { data: rows });
    }
}

pub fn run_p1() {
    let g: Grid = INPUT_1.parse().unwrap();
    let mut total = 0;
    for x_start in g.search_char('X') {
        for search_dir in SEARCH_DIRECTIONS.iter() {
            if g.match_direction(&x_start, &search_dir, &TGT_STR) {
                total += 1;
            }
        }
    }

    println!("Day 4, Part 1: {}", total);
}

pub fn run_p2() {
    let g: Grid = INPUT_1.parse().unwrap();
    let mut total = 0;

    for x_start in g.search_char('A') {
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
