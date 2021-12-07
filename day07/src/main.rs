use std::env;
use std::fs;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::HashMap;

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

fn v1(input: &Vec<i32>) {
    println!("input {:?}", input);
    let mut arr = input.clone();
    for (i, _) in input.iter().enumerate() {
        let mut sum = 0;
        for (_, y) in input.iter().enumerate() {
            sum += (y - i as i32).abs()
        }
        arr[i] = sum;
    }
    let minimum_index = arr
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index).unwrap();
    println!("{:?} {} {}", arr, minimum_index, arr[minimum_index]);
}

fn sum_from_zero( n: i32, cache: &mut HashMap<i32,i32>) -> i32 {
        if cache.contains_key(&n) {
            return *cache.get(&n).unwrap()
        }
        let result = (0 .. n+1).fold(0, |a, b| a + b);
        cache.insert(n, result);
        result
    }

fn v2(input: &Vec<i32>) {
    let mut cache: HashMap<i32, i32> = HashMap::new();
    println!("input {:?}", input);
    let mut arr = input.clone();
    for (i, _) in input.iter().enumerate() {
        let mut sum = 0;
        for (_, y) in input.iter().enumerate() {
            let distance = (y - i as i32).abs();
            let fuel = sum_from_zero(distance, &mut cache);
            sum += fuel
        }
        arr[i] = sum;
    }
    let minimum_index = arr
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index).unwrap();
    println!("{:?} {} {}", arr, minimum_index, arr[minimum_index]);

}

fn parse_config(args: &[String]) -> (&str, &str) {
    let version = &args[1];
    let filename = &args[2];

    (version, filename)
}


fn read_input(filename: &str) -> Vec<i32> {
    let contents: Vec<i32> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .trim()
        .split(',')
        .map(|x| x.parse())
        .flat_map(|x| x)
        .collect();

    contents
}
