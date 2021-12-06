use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Fish {
    age: u16,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (version, filename) = parse_config(&args);
    println!("In file {}", filename);
    let mut input = read_input(filename);
    if version == "1" {
        return v1(&mut input);
    }
    return v2(&input);
}

fn v1(input: &mut Vec<Fish>) {
    //println!("input {:?}", input);
    let days = 80;
    for _ in 0..days {
        step(input)
    }
    //println!("after {:?}", input);
    println!("amount after {} days: {}", days, input.len());
}

fn step(input: &mut Vec<Fish>) {
    for i in 0..input.len() {
        if input[i].age == 0 {
            input[i].age = 6;
            input.push(Fish { age: 8 })
        } else {
            input[i].age -= 1;
        }
    }
}

fn v2(input: &Vec<Fish>) {
    let mut amounts = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    for i in 0..input.len() {
        amounts[input[i].age as usize] += 1;
    }
    let days = 256;
    for _ in 0..days {
        let zeros = amounts[0];
        amounts[0] = amounts[1];
        amounts[1] = amounts[2];
        amounts[2] = amounts[3];
        amounts[3] = amounts[4];
        amounts[4] = amounts[5];
        amounts[5] = amounts[6];
        amounts[6] = amounts[7] + zeros;
        amounts[7] = amounts[8];
        amounts[8] = zeros;
        //println!("{:?}", amounts);
    }
    let result: u64 = amounts.iter().sum();
    println!("amount after {} days: {}", days, result);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let version = &args[1];
    let filename = &args[2];

    (version, filename)
}

impl FromStr for Fish {
    type Err = ();
    fn from_str(input: &str) -> Result<Fish, Self::Err> {
        let age = input.parse();
        if age.is_ok() {
            return Ok(Fish { age: age.unwrap() });
        }
        return Err(());
    }
}

fn read_input(filename: &str) -> Vec<Fish> {
    let contents: Vec<Fish> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .trim()
        .split(',')
        .map(|x| x.parse())
        .flat_map(|x| x)
        .collect();

    contents
}
