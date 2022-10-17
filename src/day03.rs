use crate::read_input;

fn get_lines() -> Vec<String> {
    let input = read_input::read("inputs/03.txt");
    let lines: Vec<&str> = input
        .split("\n")
        .collect();

    let mut string_lines = Vec::new();
    for line in lines {
        string_lines.push(String::from(line))
    }
    string_lines
}

fn filter_at_index(strings: Vec<String>, index_bit: usize) -> (Vec<String>, Vec<String>) {
    let mut zeroes = Vec::new();
    let mut ones = Vec::new();
    for string in strings {
        if string.chars().nth(index_bit).unwrap() == '0' {
            zeroes.push(string);
        } else {
            ones.push(string);
        }
    }
    return (zeroes, ones)
}

fn get_zeroth_element(list: &Vec<String>) -> String {
    let element;
    match list.get(0) {
        Some(s) => element = s,
        None => panic!("No elements in the input!"),
    }
    (*element).clone()
}

fn from_binary_string(s: String) -> i32 {
    let r = i32::from_str_radix(s.trim(), 2)
        .expect("Not a binary string...");
    r
}

fn part_1() -> i32 {
    let lines = get_lines();
    let mut ones_count = [0; 12];
    for line in lines.iter() {
        for (index, character) in line.chars().enumerate() {
            if character == '1' {
                ones_count[index] += 1
            }
        }
    }

    let mut g_str = String::new();
    let mut e_str = String::new();
    for c in ones_count {
        if c > lines.len() / 2 {
            g_str.push_str("1");
            e_str.push_str("0");
        } else {
            g_str.push_str("0");
            e_str.push_str("1");
        }
    }

    let g_val = from_binary_string(g_str);
    let e_val = from_binary_string(e_str);
    let ans = g_val * e_val;
    ans
}

fn part_2() -> i32 {
    let mut oxy_lines = get_lines();
    let mut co2_lines = oxy_lines.clone();
    let string_length: usize = get_zeroth_element(&oxy_lines).len();

    for index in 0..string_length {
        if oxy_lines.len() <= 1 {
            break;
        }
        let (zeroes, ones) = filter_at_index(oxy_lines.clone(), index);
        if ones.len() >= zeroes.len() {
            oxy_lines = ones;
        } else {
            oxy_lines = zeroes;
        }
    }

    for index in 0..string_length {
        if co2_lines.len() <= 1 {
            break;
        }
        let (zeroes, ones) = filter_at_index(co2_lines.clone(), index);
        if zeroes.len() <= ones.len() {
            co2_lines = zeroes;
        } else {
            co2_lines = ones;
        }
    }
    let oxy_rating: String = get_zeroth_element(&oxy_lines);
    let co2_rating: String = get_zeroth_element(&co2_lines);
    let oxy_val = from_binary_string(oxy_rating);
    let co2_val = from_binary_string(co2_rating);
    let ans = oxy_val * co2_val;
    ans
}


pub fn solve_puzzle() {
    println!("Day 3");
    let ans_1 = part_1();
    println!("Part 1: {}", ans_1);
    let ans_2 = part_2();
    println!("Part 2: {}", ans_2);
}