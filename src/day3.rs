use std::iter::Peekable;

const INPUT_1: &'static str = include_str!("day3-1.txt");

#[derive(Debug)]
struct Mul(u32, u32);

impl Mul {
    fn compute(&self) -> u32 {
        return self.0 * self.1;
    }
}

struct MulSearcher<It>
where
    It: Iterator,
{
    inner_iter: Peekable<It>,
    cond: CondBehavior,
}

impl<T> MulSearcher<T>
where
    T: Iterator,
{
    fn new(i: T, cond: CondBehavior) -> MulSearcher<T> {
        MulSearcher {
            inner_iter: i.peekable(),
            cond,
        }
    }
}

fn take_through<I>(char_iter: &mut Peekable<I>, nxt: &[char]) -> bool
where
    I: Iterator<Item = char>,
{
    for expected in nxt.iter() {
        if let Some(actual) = char_iter.peek() {
            if *actual == *expected {
                // Actually eat this one.
                char_iter.next();
            } else {
                return false;
            }
        } else {
            // Reached the end.
            return false;
        }
    }

    return true;
}

// Returns None if the next character is not a digit, or if the number
// of sequential digits is > 3
fn read_digit_until<I>(char_iter: &mut Peekable<I>, marker: char) -> Option<u32>
where
    I: Iterator<Item = char>,
{
    let mut acc = String::new();
    loop {
        match char_iter.peek() {
            Some(d) if d.is_digit(10) => {
                // Eat the digit
                acc.extend(std::iter::once(d));
            }
            Some(c) if *c == marker => {
                break;
            }
            // Either it wasn't a digit, or we reached the end.
            Some(_) | None => {
                return None;
            }
        }
        char_iter.next();
    }

    // Getting here means we found the terminal marker, but we still need to eat it.
    // Can't .next inside match bc of borrow for peek()
    char_iter.next();
    if acc.len() > 3 {
        return None;
    } else {
        return acc.parse().ok();
    }
}

impl<T> Iterator for MulSearcher<T>
where
    T: Iterator<Item = char>,
{
    type Item = Mul;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner_iter.next() {
                Some('m') => {
                    if !take_through(&mut self.inner_iter, &['u', 'l', '(']) {
                        continue;
                    }

                    // Now we parse digits ...
                    let left = match read_digit_until(&mut self.inner_iter, ',') {
                        Some(n) => n,
                        None => {
                            continue;
                        }
                    };

                    let right = match read_digit_until(&mut self.inner_iter, ')') {
                        Some(n) => n,
                        None => {
                            continue;
                        }
                    };

                    // Skip while not included.
                    if self.cond.should_include() {
                        return Some(Mul(left, right));
                    }
                },
                Some('d') => {
                    if !take_through(&mut self.inner_iter, &['o']) {
                        continue;
                    }

                    let nxt_peek = if let Some(c) = self.inner_iter.peek() {
                        *c
                    } else {
                        continue;
                    };
                    match nxt_peek {
                        'n' => {
                            // Take the char
                            self.inner_iter.next();
                            if !take_through(&mut self.inner_iter, &['\'', 't', '(', ')']) {
                                continue;
                            }
                            self.cond.set(false);
                        },
                        '(' => {
                            // Take the char
                            self.inner_iter.next();
                            if !take_through(&mut self.inner_iter, &[')']) {
                                continue;
                            }
                            self.cond.set(true);
                        },
                        _ => {
                            continue;
                        },
                    }
                }
                // We're just looking for 'm' to start a mul
                Some(_) => {
                    continue;
                }
                None => {
                    return None;
                }
            }
        }
    }
}

enum CondBehavior {
    Ignore,
    Enabled(bool),
}

impl CondBehavior {
    fn should_include(&self) -> bool {
        match self {
            CondBehavior::Ignore => true,
            CondBehavior::Enabled(e) => *e,
        }
    }

    fn set(&mut self, b: bool) {
        *self = match self {
            CondBehavior::Ignore => CondBehavior::Ignore,
            CondBehavior::Enabled(_) => CondBehavior::Enabled(b),
        }
    }
}

fn read_muls(chars: impl Iterator<Item = char>, cond_behavior: CondBehavior) -> impl Iterator<Item = Mul> {
    MulSearcher::new(chars, cond_behavior)
}

pub fn run_p1() {
    let mut total = 0;
    for found_mul in read_muls(INPUT_1.chars(), CondBehavior::Ignore) {
        total += found_mul.compute();
    }
    println!("Day 3, Part 1: {}", total);
}

pub fn run_p2() {
    let mut total = 0;
    for found_mul in read_muls(INPUT_1.chars(), CondBehavior::Enabled(true)) {
        total += found_mul.compute();
    }
    println!("Day 3, Part 2: {}", total);
}
