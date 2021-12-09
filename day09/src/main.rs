use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fmt;

#[derive(Debug)]
struct HeightMap {
    data: Vec<u8>,
    dim_x: u8,
    dim_y: u8,
}

static EMPTY: u8 = 255;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (version, filename) = parse_config(&args);
    println!("In file {}", filename);
    let input = read_input(filename);
    if version == "1" {
        return v1(&input);
    }
    //return v2(&input);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let version = &args[1];
    let filename = &args[2];

    (version, filename)
}

fn abs_to_xy(i: usize, dim: u8)->(u8,u8) {
    let y: u8 = (i / (dim as usize)) as u8;
    let x: u8 = (i - (y as usize*dim as usize)) as u8;
    (x,y)
}

fn get_at(data: &HeightMap, x: u8, y: u8)->u8 {
    data.data[y as usize * data.dim_x as usize + x as usize]
}

fn get_adjacent(data: &HeightMap, i: usize)->[u8; 4] {
    let mut out = [EMPTY, EMPTY, EMPTY, EMPTY];
    let (x,y) = abs_to_xy(i, data.dim_x);
    if y > 0 {
        out[0] = get_at(data, x, y-1);
    }
    if x + 1 < data.dim_x {
        out[1] = get_at(data, x+1, y);
    }
    if y + 1 < data.dim_y {
        out[2] = get_at(data, x, y+1);
    }
    if x > 0 {
        out[3] = get_at(data, x-1, y);
    }
    //println!("{}={},{}",i,x,y);
    out
}

fn v1(input: &HeightMap) {
    println!("{:?}", input);
    let mut minimums : Vec<usize> = Vec::new();
    for (i, x) in input.data.iter().enumerate() {
        let adj = get_adjacent(input, i);
        let neighbors: Vec<&u8> = adj.iter().filter(|&x| *x != EMPTY).collect();
        if x < neighbors.iter().min().unwrap() {
            println!("min {} {} {:?}",i, x, adj);
            minimums.push(i);
        }
    }
    let risk_lows: u32 = minimums.iter().map(|i| (input.data[*i] as u32 + 1) as u32).sum();
    println!("low points {:?} risk {}", minimums, risk_lows);
}

fn read_input(filename: &str) -> HeightMap {
    let mut out = HeightMap{data: Vec::new(), dim_x: 0, dim_y: 0};
    let file = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines = file
        .trim()
        .lines()
        .collect::<Vec<&str>>();
    out.dim_x = lines[0].len() as u8;
    out.dim_y = lines.len() as u8;
    out.data = lines
        .join("")
        .split("")
        .map(|x| x.parse())
        .flat_map(|x| x)
        .collect();

    out
}
