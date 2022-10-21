use crate::read_input;

fn get_input() -> String {
    let input = read_input::read("inputs/06.txt");
    input
}

fn get_fish_array() -> [i128; 9] {
    let mut fish = [0; 9];
    for fish_index_string in get_input().split(",") {
        let fish_index = fish_index_string
            .parse::<usize>()
            .expect("usize");
        let val = fish
            .get_mut(fish_index)
            .expect("Value at index");
        *val += 1;
    }
    fish
}

fn update_fish_array(fish: &mut [i128; 9]) {
    let zeroed_fish = fish[0];
    for i in 1..fish.len() {
        fish[i-1] = fish[i];
    }
    fish[6] += zeroed_fish;
    fish[8] = zeroed_fish;
}

fn find_fish_count_after_days(days: i32) -> i128 {
    let mut fish = get_fish_array();
    for _ in 0..days {
        update_fish_array(&mut fish);
    }
    let mut count = 0;
    for i in fish {
        count += i;
    }
    count
}

fn part_1() -> i128 {
    find_fish_count_after_days(80)
}

fn part_2() -> i128 {
    find_fish_count_after_days(256)
}

pub fn solve_puzzle() {
    println!("Day 6");
    let ans_1 = part_1();
    println!("Part 1: {}", ans_1);
    let ans_2 = part_2();
    println!("Part 2: {}", ans_2);
}