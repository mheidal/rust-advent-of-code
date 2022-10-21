use std::iter::zip;
use crate::read_input;

struct Coord {
    x: i32,
    y: i32
}

fn get_input() -> String {
    let input = read_input::read("inputs/05.txt");
    input
}

fn parse_str_to_i32(string: &str) -> i32 {
    string
        .trim()
        .parse::<i32>()
        .expect("String wasn't an integer")
}

fn coord_pair_to_struct(pair: &str) -> Coord {
    let mut pair_iter = pair.split(",");
    let x = parse_str_to_i32(pair_iter.next().expect("No first string in pair"));
    let y = parse_str_to_i32(pair_iter.next().expect("No second string in pair"));
    Coord { x, y }
}

fn get_coord_list() -> Vec<[Coord; 2]> {
    let mut coords = Vec::new();
    let input = get_input();
    for line in input.split("\n") {
        let mut line_iter = line.split(" -> ");
        let start: Coord = coord_pair_to_struct(
            line_iter
                .next()
                .expect("No first coordinate")
        );
        let end: Coord = coord_pair_to_struct(
            line_iter
                .next()
                .expect("No second coordinate")
        );
        coords.push([start, end])
    }
    coords
}

fn get_base_board(coords: &Vec<[Coord; 2]>) -> Vec<Vec<i32>> {
    let mut min_x: i32 = 0;
    let mut min_y: i32 = 0;
    let mut max_x: i32 = i32::MIN;
    let mut max_y: i32 = i32::MIN;
    for coord_pair in coords {
        for coord in coord_pair {
            if coord.x < min_x {
                min_x = coord.x;
            }
            if coord.y < min_y {
                min_y = coord.y
            }
            if coord.x > max_x {
                max_x = coord.x;
            }
            if coord.y > max_y {
                max_y = coord.y;
            }
        }
    }
    let mut grid = Vec::new();
    for _row_index in min_x..(max_x + 1) {
        let mut row = Vec::new();
        for _col_index in min_y..(max_y + 1) {
            row.push(0);
        }
        grid.push(row);
    }
    grid
}

// from https://stackoverflow.com/questions/70329833/range-where-start-end
fn range_inclusive(a: i32, b: i32) -> impl Iterator<Item = i32> {
    let x: Box<dyn Iterator<Item = i32>>;
    if b > a {
        x = Box::new(a..=b)
    } else {
        x = Box::new((b..=a).rev())
    }
    x
}

fn get_points_in_cardinal_line(coord_pair: &[Coord; 2]) -> Vec<Coord> {
    let mut point_list = Vec::new();
    if coord_pair[0].x == coord_pair[1].x  {
        for y in range_inclusive(coord_pair[0].y, coord_pair[1].y) {
            point_list.push(Coord {x: coord_pair[0].x, y})
        }
    }
    if coord_pair[0].y == coord_pair[1].y {
        for x in range_inclusive(coord_pair[0].x, coord_pair[1].x) {
            point_list.push(Coord {x, y: coord_pair[0].y})
        }
    }
    point_list
}

fn get_points_in_diagonal_line(coord_pair: &[Coord; 2]) -> Vec<Coord> {
    let mut points = Vec::new();
    for (x, y) in zip(
        range_inclusive(coord_pair[0].x, coord_pair[1].x),
        range_inclusive(coord_pair[0].y, coord_pair[1].y),
    ) {
        points.push(Coord {x, y})
    }
    points
}

fn get_points_in_line(coord_pair: &[Coord; 2]) -> Vec<Coord> {
    if (coord_pair[0].x == coord_pair[1].x) || (coord_pair[0].y == coord_pair[1].y) {
        get_points_in_cardinal_line(&coord_pair)
    } else {
        get_points_in_diagonal_line(&coord_pair)
    }
}

fn add_to_board(board: &mut Vec<Vec<i32>>, index: &Coord) {
    let prev_value = board.get_mut(index.x as usize)
        .expect("X index")
        .get_mut(index.y as usize)
        .expect("Y index");
    *prev_value += 1;
}

fn get_num_of_line_intersections(board: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for row in board {
        for entry in row {
            if entry > 1 {
                count += 1;
            }
        }
    }
    count
}

fn display_board(board: &Vec<Vec<i32>>) {
    let mut whole_string = String::new();
    for row in board {
        let mut row_string = String::new();
        for entry in row {
            if *entry == 0 {
                row_string.push('.');
            } else {
                row_string.push_str(entry.to_string().as_str());
            }
        }
        whole_string.push_str(row_string.as_str());
        whole_string.push('\n');
    }
    println!("{}", whole_string);
}

fn part_1() -> i32 {
    let coords = get_coord_list();
    let mut board = get_base_board(&coords);
    for coord_pair in &coords {
        for point in get_points_in_cardinal_line(coord_pair) {
            add_to_board(&mut board, &point);
        }
    }
    get_num_of_line_intersections(board)
}

fn part_2() -> i32 {
    let coords = get_coord_list();
    let mut board = get_base_board(&coords);
    for coord_pair in &coords {
        for point in get_points_in_line(coord_pair) {
            add_to_board(&mut board, &point);
        }
    }
    // display_board(&board);
    get_num_of_line_intersections(board)
}


pub fn solve_puzzle() {
    println!("Day 4");
    let ans_1 = part_1();
    println!("Part 1: {}", ans_1);
    let ans_2 = part_2();
    println!("Part 2: {}", ans_2);
}