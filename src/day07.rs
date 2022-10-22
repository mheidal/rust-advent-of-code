use crate::read_input;

fn get_input() -> Vec<i32> {
    let input = read_input::read("inputs/07.txt");
    let mut crabs = Vec::new();
    for crab_string in input.split(",") {
        crabs.push(
            crab_string
                .parse::<i32>()
                .expect("Integer")
        )
    }
    crabs
}

fn linear_cost_of_move(crabs: &Vec<i32>, index: i32) -> i32 {
    let mut cost = 0;
    for crab in crabs {
        let dist = *crab - index;
        if dist < 0 {
            cost += dist * -1
        } else {
            cost += dist
        }
    }
    cost
}

fn triangular_number(num: i32) -> i32 {
    num * (num + 1) / 2
}

fn triangular_cost_of_move(crabs: &Vec<i32>, index: i32) -> i32 {
    let mut cost = 0;
    for crab in crabs {
        let dist = *crab - index;
        if dist < 0 {
            cost += triangular_number(dist * -1)
        } else {
            cost += triangular_number(dist)
        }
    }
    cost
}

fn get_min_and_max_crab(crabs: &Vec<i32>) -> (i32, i32) {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for crab in crabs {
        if *crab < min {
            min = *crab;
        }
        if *crab > max {
            max = *crab;
        }
    }
    (min, max)
}

fn get_cost_of_move(cost_func: fn(&Vec<i32>, i32) -> i32) -> i32 {
    let crabs = get_input();
    let (min, max) = get_min_and_max_crab(&crabs);
    let mut min_cost = i32::MAX;
    for index in min..=max {
        let cost = cost_func(&crabs, index);
        if cost < min_cost {
            min_cost = cost;
        }
    }
    min_cost
}

fn part_1() -> i32 {
    get_cost_of_move(linear_cost_of_move)
}

fn part_2() -> i32 {
    get_cost_of_move(triangular_cost_of_move)
}

pub fn solve_puzzle() {
    println!("Day 7");
    let ans_1 = part_1();
    println!("Part 1: {}", ans_1);
    let ans_2 = part_2();
    println!("Part 2: {}", ans_2);
}