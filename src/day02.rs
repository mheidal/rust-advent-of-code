use crate::read_input;

fn get_steps() -> Vec<(String, i64)> {
    let input = read_input::read("inputs/02.txt");
    let lines = input.split("\n");
    let mut steps = Vec::new();

    for line in lines {
        let mut step = line.split(" ");
        let string: String;
        let val: i64;

         match step.next() {
            Some(s) => string = String::from(s),
            None => continue,
        }

        match step.next() {
            Some(string) => val = string
                .trim()
                .parse::<i64>()
                .expect("Value wasn't a int."),
            None => continue,
        }

        steps.push((string, val))
    }
    steps
}

fn part_1() -> i64 {

    let steps = get_steps();

    let mut pos_h = 0;
    let mut pos_v = 0;

    for step in steps {
        let (step_string, val) = step;
        match step_string.as_str() {
            "forward" => pos_h += val,
            "up" => pos_v -= val,
            "down" => pos_v += val,
            _ => continue,
        }
    }
    let ans = pos_h * pos_v;
    ans
}

fn part_2() -> i64 {
    let steps = get_steps();
    let mut pos_h = 0;
    let mut pos_v = 0;
    let mut aim = 0;
    for step in steps {
        let (step_string, val) = step;
        match step_string.as_str() {
            "forward" => {
                pos_v += val * aim;
                pos_h += val;
            },
            "up" => aim -= val,
            "down" => aim += val,
            _ => continue,
        }
    }
    let ans = pos_h * pos_v;
    ans
}


pub fn solve_puzzle() {
    println!("Day 2");
    let ans_1 = part_1();
    println!("Part 1: {}", ans_1);
    let ans_2 = part_2();
    println!("Part 2: {}", ans_2);
}