use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug, PartialEq)]
struct Command {
    direction: Direction,
    amount: i32,
}

#[derive(Debug, PartialEq)]
struct Position {
    depth: i32,
    position: i32,
    aim: i32,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        let items: Vec<&str> = input.split_whitespace().collect();
        match items.len() {
            2 => {
                let direction = Direction::from_str(items[0]);
                let amount = items[1].parse::<i32>();
                if direction.is_ok() && amount.is_ok() {
                    return Ok(Command {
                        direction: direction.unwrap(),
                        amount: amount.unwrap(),
                    });
                }
                return Err(());
            }
            _ => Err(()),
        }
    }
}

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let (version, filename) = parse_config(&args);
    println!("In file {}", filename);
    let input = read_input::<Command>(filename);
    if version == "1" {
        return v1(&input);
    }
    return v2(&input);
}

fn v1(input: &Vec<Command>) {
    println!("input: {:?}", &input);
    let mut position = Position {
        depth: 0,
        position: 0,
        aim: 0,
    };
    for x in input {
        if x.direction == Direction::Up {
            position.depth -= x.amount
        } else if x.direction == Direction::Down {
            position.depth += x.amount
        } else if x.direction == Direction::Forward {
            position.position += x.amount
        }
    }
    let a: i32 = position.position.into();
    let b: i32 = position.depth.into();
    let result: i32 = a * b;
    println!("result: {:?}", result);
}

fn v2(input: &Vec<Command>) {
    println!("input: {:?}", &input);
    let mut position = Position {
        depth: 0,
        position: 0,
        aim: 0,
    };
    for x in input {
        if x.direction == Direction::Up {
            position.aim -= x.amount
        } else if x.direction == Direction::Down {
            position.aim += x.amount
        } else if x.direction == Direction::Forward {
            position.position += x.amount;
            let delta: i32 = position.aim * x.amount;
            position.depth += delta;
        }
    }
    let a: i64 = position.position.into();
    let b: i64 = position.depth.into();
    let result: i64 = a * b;
    println!("result: {:?}", result);
}

fn read_input<T: FromStr>(filename: &str) -> Vec<T> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.parse())
        .flat_map(|x| x)
        .collect();
    contents
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let version = &args[1];
    let filename = &args[2];

    (version, filename)
}
