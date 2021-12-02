use std::env;
use std::fs;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (version, filename) = parse_config(&args);

    println!("In file {}", filename);

    let contents = read_numbers::<u32>(filename);

    if version == "2" {
        return v2(&contents);
    }

    let mut increases = 0;
    contents.iter().fold(0u32, |acc, &x| {
        if acc > 0 && x > acc {
            increases = increases + 1;
        }
        x
    });

    println!("Input\n{:?}\nIncreases: {}", contents, increases);
}

fn v2(contents: &Vec<u32>) {
    let mut sums = Vec::new();
    for i in 1..contents.len() - 1 {
        sums.push(contents[i - 1] + contents[i] + contents[i + 1])
    }
    let mut increases = 0;
    sums.iter().fold(0u32, |acc, &x| {
        if acc > 0 && x > acc {
            increases = increases + 1;
        }
        x
    });
    println!("Increases: {}", increases);
}

fn read_numbers<T: FromStr>(filename: &str) -> Vec<T> {
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
