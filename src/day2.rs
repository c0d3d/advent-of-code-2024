use std::{fmt::Display, ops::Sub, str::FromStr};

const INPUT_1: &'static str = include_str!("day2-1.txt");

type E = &'static str;

#[derive(Debug)]
struct Report(Vec<u32>);

impl FromStr for Report {
    type Err = E;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Report(
            s.split_whitespace()
                .map(u32::from_str)
                .filter_map(Result::ok)
                .collect(),
        ));
    }
}

#[derive(Debug)]
enum Direction {
    Asc,
    Desc,
}

impl Direction {
    fn is_fine<A: Display + Ord + Sub>(&self, n1: A, n2: A) -> bool
    where
        <A as Sub>::Output: Ord,
        <A as Sub>::Output: From<u32>,
    {
        let diff = match self {
            Direction::Asc => {
                if n2 > n1 {
                    n2 - n1
                } else {
                    return false;
                }
            }
            Direction::Desc => {
                if n1 > n2 {
                    n1 - n2
                } else {
                    return false;
                }
            }
        };

        return diff <= 3.into();
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        // Trivial cases.
        if self.0.len() == 0 || self.0.len() == 1 {
            return true;
        }

        let mut prev = self.0[0];
        if self.0[1] == prev {
            // Not increasing or decreasing ...
            return false;
        }
        let dir = if prev > self.0[1] {
            Direction::Desc
        } else {
            Direction::Asc
        };

        for nxt in &self.0[1..] {
            if dir.is_fine(prev, *nxt) {
                prev = *nxt;
            } else {
                return false;
            }
        }

        return true;
    }

    fn without(&self, idx: usize) -> Report {
        let mut v = self.0.clone();
        v.remove(idx);
        return Report(v);
    }

    fn is_safe_with_dampener(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for i in 0..self.0.len() {
            if self.without(i).is_safe() {
                return true;
            }
        }

        return false;
    }
}

fn parse_input() -> Vec<Report> {
    return INPUT_1
        .lines()
        .filter_map(|x| Report::from_str(x).ok())
        .collect();
}

pub fn run_p1() {
    let report_list = parse_input();
    println!(
        "Day 2, Part 1: {}",
        report_list.into_iter().filter(Report::is_safe).count()
    )
}

pub fn run_p2() {
    let report_list = parse_input();
    println!(
        "Day 2, Part 2: {}",
        report_list
            .into_iter()
            .filter(Report::is_safe_with_dampener)
            .count()
    )
}

#[cfg(test)]
mod test_p1 {
    use crate::day2::Report;

    #[test]
    fn report_is_bad_up_and_down() {
        assert!(!Report(vec![61, 60, 51, 54, 51]).is_safe(), "Up and down");
    }

    #[test]
    fn report_is_safe_all_up() {
        assert!(
            Report(vec![52, 53, 54, 55, 56, 58]).is_safe(),
            "All up 1 or 2"
        );
    }

    #[test]
    fn report_is_safe_all_down() {
        assert!(Report(vec![53, 52, 51]).is_safe(), "All down 1");
    }

    #[test]
    fn report_is_bad_diff_too_big() {
        assert!(!Report(vec![53, 49, 47]).is_safe(), "Gap too big");
    }

    #[test]
    fn report_is_bad_diff_too_small() {
        assert!(!Report(vec![5, 5, 7, 9, 12, 16]).is_safe(), "Gap too small");
    }
}

#[cfg(test)]
mod test_p2 {
    use crate::day2::Report;

    #[test]
    fn report_is_bad_up_and_down_same_damper() {
        assert!(
            !Report(vec![61, 60, 51, 54, 51]).is_safe_with_dampener(),
            "Up and down"
        );
    }

    #[test]
    fn report_is_good_up_and_down_damper() {
        assert!(
            !Report(vec![61, 60, 51, 54, 49]).is_safe_with_dampener(),
            "Up and down"
        );
    }

    #[test]
    fn report_is_safe_all_up() {
        assert!(
            Report(vec![52, 53, 54, 55, 56, 58]).is_safe_with_dampener(),
            "All up 1 or 2"
        );
    }

    #[test]
    fn report_is_safe_all_down() {
        assert!(
            Report(vec![53, 52, 51]).is_safe_with_dampener(),
            "All down 1"
        );
    }

    #[test]
    fn report_is_good_diff_too_big() {
        assert!(
            Report(vec![53, 49, 52]).is_safe_with_dampener(),
            "Gap too big"
        );
    }

    #[test]
    fn report_is_bad_two_diffs_too_big() {
        assert!(
            !Report(vec![53, 49, 42]).is_safe_with_dampener(),
            "Gap too big"
        );
    }

    #[test]
    fn report_is_bad_one_diff_too_big_after_dampened_first() {
        assert!(
            !Report(vec![5, 5, 7, 9, 12, 16]).is_safe_with_dampener(),
            "Gap too big"
        );
    }

    #[test]
    fn report_is_good_one_diff_too_small() {
        assert!(
            Report(vec![5, 5, 7, 9, 12, 13]).is_safe_with_dampener(),
            "Gap too small"
        );
    }

    #[test]
    fn report_is_bad_two_diff_too_small1() {
        assert!(
            !Report(vec![5, 5, 5, 7, 9, 12, 16]).is_safe_with_dampener(),
            "Gap too small"
        );
    }

    #[test]
    fn report_is_bad_two_diff_too_small2() {
        assert!(
            !Report(vec![5, 5, 7, 7, 12, 16]).is_safe_with_dampener(),
            "Gap too small"
        );
    }

    #[test]
    fn report_is_bad_wrong_dir_to_start_asc_with_big_gap() {
        assert!(
            !Report(vec![5, 4, 8, 12, 13]).is_safe_with_dampener(),
            "Ascending with dampener big gap"
        );
    }

    #[test]
    fn report_is_good_wrong_dir_to_start_asc() {
        assert!(
            Report(vec![5, 4, 8, 9, 10]).is_safe_with_dampener(),
            "Ascending with dampener"
        );
    }

    #[test]
    fn report_is_good_wrong_dir_to_start_desc() {
        assert!(
            Report(vec![7, 9, 4, 1]).is_safe_with_dampener(),
            "Desc with dampener"
        );
    }

    #[test]
    fn is_safe_with_first_removed() {
        assert!(Report(vec![432,45,44,43,42]).is_safe_with_dampener())
    }

    #[test]
    fn is_safe_with_second_removed() {
        assert!(Report(vec![45,432,46,47,48]).is_safe_with_dampener())
    }

    #[test]
    fn is_safe_with_cyclops() {
        assert!(Report(vec![28, 25, 26, 25, 24]).is_safe_with_dampener())
    }

    #[test]
    fn sample_examples() {
        assert!(Report(vec![7,6,4,2,1]).is_safe_with_dampener());
        assert!(!Report(vec![1,2,7,8,9]).is_safe_with_dampener());
        assert!(!Report(vec![9,7,6,2,1]).is_safe_with_dampener());
        assert!(Report(vec![1,3,2,4,5]).is_safe_with_dampener());
        assert!(Report(vec![8,6,4,4,1]).is_safe_with_dampener());
        assert!(Report(vec![1,3,6,7,9]).is_safe_with_dampener());
    }

}
