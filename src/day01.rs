use std::collections::VecDeque;
use crate::read_input;

fn get_depths() -> Vec<i32> {
    let input = read_input::read("inputs/01.txt");
    let mut depths = Vec::new();
    let split = input.split("\n");
    for line in split {
        match line.trim().parse::<i32>() {
            Ok(depth) => depths.push(depth),
            Err(_e) => continue,
        }
    }
    depths
}

fn part_1() -> i32 {
    let depths = get_depths();
    let mut count = 0;
    let mut prev = i32::MAX;
    for d in depths {
        if prev < d {
            count += 1;
        }
        prev = d;
    }
    count
}

fn part_2() -> i32 {
    let depths = get_depths();
    let mut window = VecDeque::new();
    let mut count = 0;
    for d in depths {
        window.push_front(d);
        if window.len() < 4 {
            continue;
        } else {
            let old = window.pop_back()
                .expect("No first element of three-set");
            if d > old {
                count += 1
            }
        }
    }
    count
}

pub fn solve_puzzle() {
    println!("Day 1");
    let ans_1 = part_1();
    println!("Part 1: {}", ans_1);

    let ans_2 = part_2();
    println!("Part 2: {}", ans_2);
}

