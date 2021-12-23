use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
struct Cuboid {
    on: bool,
    x0: i32,
    x1: i32,
    y0: i32,
    y1: i32,
    z0: i32,
    z1: i32,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32
}

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

fn v1(input: &Vec<Cuboid>) {
    let limit = 50;
    let mut set = HashSet::new();

    let filtered: Vec<&Cuboid> = input.iter().filter(|c|
        !(c.x0.abs() > limit ||
        c.x1.abs() > limit ||
        c.y0.abs() > limit ||
        c.y1.abs() > limit ||
        c.z0.abs() > limit ||
        c.z1.abs() > limit)
    ).collect();

    for item in filtered {
        for x in item.x0..item.x1+1 {
            for y in item.y0..item.y1+1 {
                for z in item.z0..item.z1+1 {
                    let p = Point{x,y,z};
                    if item.on {
                        set.insert(p);
                        //println!("on {:?}", p);
                    } else {
                        set.remove(&p);
                        //println!("off {:?}", p);
                    }
                }
            }
        }
    }

    println!("set size {}", set.len());
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let version = &args[1];
    let filename = &args[2];

    (version, filename)
}

fn as_int(x: regex::Match)-> i32 {
    x.as_str().parse().unwrap()
}

fn read_input(filename: &str) -> Vec<Cuboid> {

    let reader = BufReader::new(File::open(filename).expect("Cannot open file"));

    let mut contents: Vec<Cuboid> = Vec::new();

    let line_regex: Regex = Regex::new(r"x=([-0-9]+)\.\.([-0-9]+),y=([-0-9]+)\.\.([-0-9]+),z=([-0-9]+)\.\.([-0-9]+)").unwrap();

    for line in reader.lines() {
        let l = &line.unwrap();
        let captured: Vec<Cuboid> = line_regex
        .captures_iter(l)
        .filter_map(|cap| {
            let points = (cap.get(1), cap.get(2), cap.get(3), cap.get(4), cap.get(5), cap.get(6));
            match points {
                (Some(x0), Some(x1), Some(y0), Some(y1), Some(z0), Some(z1)) => {
                    Some(Cuboid {
                        on: l.starts_with("on"),
                        x0: as_int(x0),
                        x1: as_int(x1),
                        y0: as_int(y0),
                        y1: as_int(y1),
                        z0: as_int(z0),
                        z1: as_int(z1) })
                }
                _ => None,
            }
        })
        .collect();
        if captured.len() > 0 {
            contents.push(captured[0])
        }
    }

    contents
}

