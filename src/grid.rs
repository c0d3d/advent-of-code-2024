use std::str::FromStr;

pub const DOWN_RIGHT: Direction = Direction(1, 1);
pub const UP_LEFT: Direction = Direction(-1, -1);
pub const DOWN_LEFT: Direction = Direction(-1, 1);
pub const UP_RIGHT: Direction = Direction(1, -1);
pub const UP: Direction = Direction(0, -1);
pub const DOWN: Direction = Direction(0, 1);
pub const RIGHT: Direction = Direction(1, 0);
pub const LEFT: Direction = Direction(-1, 0);

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point(isize, isize);

#[derive(Eq, PartialEq)]
/// Construction intentionally left private, we only expose a set directions (each 8th)
pub struct Direction(isize, isize);

impl Direction {
    pub fn rotate90(&'static self) -> &'static Direction {
        match self {
            &DOWN_LEFT => &UP_LEFT,
            &DOWN => &LEFT,
            &DOWN_RIGHT => &DOWN_LEFT,
            &RIGHT => &DOWN,
            &UP_RIGHT => &DOWN_RIGHT,
            &UP => &RIGHT,
            &UP_LEFT => &UP_RIGHT,
            &LEFT => &UP,
            _ => panic!("Impossible")
        }
    }
}

impl Point {
    pub fn add(&self, d: &Direction) -> Point {
        return Point(self.0 + d.0, self.1 + d.1);
    }
}

pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

pub struct GridSearch<'a, T, P> {
    g: &'a Grid<T>,
    pred: P,
    x: usize,
    y: usize,
}

impl<'a, T, P> Iterator for GridSearch<'a, T, P>
where
    P: Fn(&'a Grid<T>, &Point) -> bool,
{
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let mut last_x = self.x;
        for sy in self.y..self.g.data.len() {
            for sx in last_x..self.g.data[sy].len() {
                if (self.pred)(&self.g, &Point(sx as isize, sy as isize)) {
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

impl<T> Grid<T>
where
    T: Eq,
{
    pub fn search<'b, P>(&'b self, pred: P) -> impl Iterator<Item = Point> + 'b
    where
        P: Fn(&'b Grid<T>, &'_ Point) -> bool + 'b,
    {
        return GridSearch {
            g: self,
            x: 0,
            y: 0,
            pred,
        };
    }

    pub fn match_direction(&self, start: &Point, dir: &Direction, to_match: &[T]) -> bool {
        let mut nxt_point = start.clone();
        for nxt_char in to_match.iter() {
            match self.at(&nxt_point) {
                Some(actual_char) if *actual_char == *nxt_char => {
                    nxt_point = nxt_point.add(dir);
                },
                _ => {
                    return false;
                },
            }
        }

        return true;
    }
}

impl<T> Grid<T> {
    pub fn at<'a>(&'a self, p: &Point) -> Option<&'a T> {
        if p.1 < 0 || p.1 as usize >= self.data.len() {
            return None;
        }

        if p.0 < 0 || p.0 as usize >= self.data[p.1 as usize].len() {
            return None;
        }

        return Some(&self.data[p.1 as usize][p.0 as usize]);
    }

    pub fn in_bounds(&self, p: &Point) -> bool {
        return self.at(p).is_some();
    }
}

impl<T> FromStr for Grid<T>
where
    T: FromStr,
{
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = Vec::new();
        let mut scratch = [0u8; 4];
        for line in s.lines() {
            let mut cols = Vec::new();
            for c in line.chars() {
                cols.push(c.encode_utf8(&mut scratch).parse()?);
            }
            rows.push(cols);
        }
        return Ok(Grid { data: rows });
    }
}
