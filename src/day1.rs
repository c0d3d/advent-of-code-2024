use std::collections::HashMap;

const INPUT_1: &'static str = include_str!("day1-1.txt");

fn parse_input1(s: &'static str) -> (Vec<u32>, Vec<u32>) {
    s.split("\n")
        .into_iter()
        .filter_map(|line| {
            let mut splits = line.split_whitespace();
            let p1_1 = splits.next()?;
            let p2_1 = splits.next()?;

            let p1 = p1_1.parse::<u32>().unwrap();
            let p2 = p2_1.parse::<u32>().unwrap();
            return Some((p1, p2));
        })
        .unzip()
}

fn freq_count(nums: &Vec<u32>) -> HashMap<u32, u32> {
    let mut out_cnt = HashMap::new();
    for n in nums.iter() {
        if out_cnt.contains_key(n) {
            let x = out_cnt.get_mut(n).unwrap();
            *x = *x + 1;
        } else {
            out_cnt.insert(*n, 1);
        }
    }

    return out_cnt;
}

pub fn run_p1() {
    let (mut l1, mut l2) = parse_input1(INPUT_1);
    l1.sort();
    l2.sort();

    println!(
        "Day 1, Part 1: {}",
        l1.into_iter()
            .zip(l2.into_iter())
            .map(|(l, r)| l.abs_diff(r))
            .fold(0, std::ops::Add::add)
    );
}

pub fn run_p2() {
    let (l1, l2) = parse_input1(INPUT_1);
    let freq_count = freq_count(&l2);
    println!(
        "Day 1, Part 2: {}",
        l1.into_iter().fold(0, |acc, nxt| {
            acc + (nxt * freq_count.get(&nxt).unwrap_or(&0))
        })
    );
}
