use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Line {
    start: Point,
    end: Point,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (version, filename) = parse_config(&args);
    println!("In file {}", filename);
    let input = read_input(filename);
    if version == "1" {
        return v1(&input);
    }
    return v2(&input);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let version = &args[1];
    let filename = &args[2];

    (version, filename)
}

fn is_horizontal(line: &Line) -> bool {
    line.start.y == line.end.y
}

fn is_vertical(line: &Line) -> bool {
    line.start.x == line.end.x
}

fn v1(input: &Vec<Line>) {
    println!("input {:?}", input);
    let straight_lines: Vec<&Line> = input
        .iter()
        .filter(|line| is_horizontal(*line) || is_vertical(*line))
        .collect();
    println!("straight {:?}", straight_lines);
    let board = paint(&straight_lines);
    println!("board {:?}", board);
    for y in 0..10 {
        for x in 0..10 {
            let p = Point { x, y };
            let val = board.get(&p).unwrap_or(&0);
            print!("{}", *val);
        }
        print!("\n");
    }

    let result: u16 = board.values().filter(|&x| *x > 1).map(|_| 1).sum();
    println!("result {}", result);
}

fn paint(input: &Vec<&Line>) -> HashMap<Point, u16> {
    let mut board: HashMap<Point, u16> = HashMap::new();
    for line in input {
        //println!("plotting line {:?}", line);
        if is_horizontal(*line) {
            let y = line.start.y;
            for x in line.start.x..line.end.x + 1 {
                let key = Point { x, y };
                if let Some(val) = board.get_mut(&key) {
                    *val += 1;
                } else {
                    //println!("insert 1 {:?}", key);
                    board.insert(key, 1);
                }
            }
        } else if is_vertical(*line) {
            let x = line.start.x;
            for y in line.start.y..line.end.y + 1 {
                let key = Point { x, y };
                if let Some(val) = board.get_mut(&key) {
                    *val += 1;
                } else {
                    //println!("insert 2 {:?}", key);
                    board.insert(key, 1);
                }
            }
        } else {
            // assumed 45 degree line
            let mut x = line.start.x;
            let mut y = line.start.y;
            let mut xdir: i16 = 1;
            let mut ydir: i16 = 1;
            if line.start.x > line.end.x {
                xdir = -1;
            }
            if line.start.y > line.end.y {
                ydir = -1;
            }
            loop {
                let key = Point { x, y };
                if let Some(val) = board.get_mut(&key) {
                    *val += 1;
                } else {
                    //println!("insert 2 {:?}", key);
                    board.insert(key, 1);
                }
                if x == line.end.x && y == line.end.y {
                    break;
                }
                x = (x as i16 + xdir) as u16;
                y = (y as i16 + ydir) as u16;
            }
        }
    }
    board
}

fn v2(input: &Vec<Line>) {
    println!("input {:?}", input);
    let all_lines: Vec<&Line> = input.iter().filter(|_| true).collect();
    println!("all {:?}", all_lines);
    let board = paint(&all_lines);
    println!("board {:?}", board);
    for y in 0..10 {
        for x in 0..10 {
            let p = Point { x, y };
            let val = board.get(&p).unwrap_or(&0);
            print!("{}", *val);
        }
        print!("\n");
    }

    let result: u16 = board.values().filter(|&x| *x > 1).map(|_| 1).sum();
    println!("result {}", result);
}

fn make_point(x: regex::Match, y: regex::Match) -> Point {
    Point {
        x: x.as_str().parse().unwrap(),
        y: y.as_str().parse().unwrap(),
    }
}

fn read_input(filename: &str) -> Vec<Line> {
    let reader = BufReader::new(File::open(filename).expect("Cannot open file"));

    let mut contents = Vec::new();

    let point_regex: Regex = Regex::new(r"(\d+),(\d+)\s->\s(\d+),(\d+)").unwrap();

    for line in reader.lines() {
        let lines: Vec<Line> = point_regex
            .captures_iter(&line.unwrap())
            .filter_map(|cap| {
                let points = (cap.get(1), cap.get(2), cap.get(3), cap.get(4));
                match points {
                    (Some(x1), Some(y1), Some(x2), Some(y2)) => {
                        let p1 = make_point(x1, y1);
                        let p2 = make_point(x2, y2);
                        if p1.x > p2.x || p1.y > p2.y {
                            return Some(Line { start: p2, end: p1 });
                        }
                        Some(Line { start: p1, end: p2 })
                    }
                    _ => None,
                }
            })
            .collect();
        if lines.len() > 0 {
            contents.push(lines[0]);
        }
    }

    contents
}
