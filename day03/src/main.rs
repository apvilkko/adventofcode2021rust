use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn read_input(filename: &str) -> Vec<Vec<char>> {

    let reader = BufReader::new(File::open(filename).expect("Cannot open file"));

    let mut contents = Vec::new();

    for line in reader.lines() {
        let chars: Vec<char> = line.unwrap().chars().collect();
        contents.push(chars)
    }

    contents
}

fn v1(input: &Vec<Vec<char>>) {
    println!("{:?}", input);

    let bit_depth: u8 = input[0].len() as u8;
    println!("bit depth {}", bit_depth);

    let mut gamma_rate: Vec<u16> = Vec::new();
    let mut epsilon_rate: Vec<u16> = Vec::new();
    for i in 0..bit_depth {
        let input_bits: Vec<bool> = input.iter().map(|x| x[i as usize] == '1').collect();
        let mut value = 0;
        let mut e_value = 0;
        let one_count = input_bits.iter().filter(|&x| *x).count();
        let zero_count = input_bits.iter().filter(|&x| !*x).count();
        if one_count > zero_count {
            value = 1;
            e_value = 0;
        } else {
            value = 0;
            e_value = 1;
        }
        gamma_rate.push(value);
        epsilon_rate.push(e_value);
    }
    let gamma_rate_value = from_binary(&gamma_rate, bit_depth);
    let epsilon_rate_value = from_binary(&epsilon_rate, bit_depth);
    println!("{:?} {} {:?} {}", gamma_rate, gamma_rate_value, epsilon_rate, epsilon_rate_value);
    let result: u64 = gamma_rate_value as u64 * epsilon_rate_value as u64;
    println!("result {}", result);
}

fn from_binary(input: &Vec<u16>, bits: u8)->u16 {
    let mut value: u16 = 0;
    for i in 0..bits {
        value += (input[(bits-i-1) as usize] as u16) << i
    }
    value
}

fn from_binary_c(input: &Vec<char>, bits: u8)->u16 {
    let mut value: u16 = 0;
    for i in 0..bits {
        let mut val: u16 = 0;
        if input[(bits-i-1) as usize] == '1' {
            val = 1
        }
        value += val << i
    }
    value
}

fn v2(input: &Vec<Vec<char>>) {

    let bit_depth: u8 = input[0].len() as u8;
    println!("bit depth {}", bit_depth);

    let mut oxygen_indexes: Vec<usize> = input.iter().enumerate().map(|(i,_)| i).collect();
    let mut co2_indexes: Vec<usize> = input.iter().enumerate().map(|(i,_)| i).collect();

    let mut oxygen_value = 0;
    let mut co2_value = 0;
    for i in 0..bit_depth {
        let (x,y) = step(input, &oxygen_indexes, &co2_indexes, i);
        oxygen_indexes = x;
        co2_indexes = y;
        if oxygen_indexes.len() == 1 {
            let oxygen_arr = &input[oxygen_indexes[0]];
            oxygen_value = from_binary_c(&oxygen_arr, bit_depth);
            println!("oxygen {}", oxygen_value);
        }
        if co2_indexes.len() == 1 {
            let arr = &input[co2_indexes[0]];
            co2_value = from_binary_c(&arr, bit_depth);
            println!("co2 {}", co2_value);
        }
    }
    println!("result {}", oxygen_value as u32 * co2_value as u32);
}

fn step(input: &Vec<Vec<char>>, indexes: &Vec<usize>,
    co2_indexes: &Vec<usize>, bit_position: u8)->(Vec<usize>, Vec<usize>) {
    let i = bit_position;

    let mut input_bits_1: Vec<u8> = Vec::new();
    let mut input_bits_2: Vec<u8> = Vec::new();
    for j in 0..input.len() {
        if indexes.iter().filter(|&x| *x == j).count() > 0 {
            let mut value = 0;
            if input[j][i as usize] == '1' {
                value = 1;
            }
            input_bits_1.push(value)
        } else {
            input_bits_1.push(2);
        }
        if co2_indexes.iter().filter(|&x| *x == j).count() > 0 {
            let mut value = 0;
            if input[j][i as usize] == '1' {
                value = 1;
            }
            input_bits_2.push(value)
        } else {
            input_bits_2.push(2);
        }
    }

    let one_count = input_bits_1.iter().filter(|&x| *x == 1).count();
    let zero_count = input_bits_1.iter().filter(|&x| *x == 0).count();
    let one_count_co2 = input_bits_2.iter().filter(|&x| *x == 1).count();
    let zero_count_co2 = input_bits_2.iter().filter(|&x| *x == 0).count();
    let mut new_indexes: Vec<usize> = Vec::new();
    let mut new_co2_indexes: Vec<usize> = Vec::new();
    for j in 0..input_bits_1.len() {
        if input_bits_1[j] == 1 && one_count >= zero_count || input_bits_1[j] == 0 && one_count < zero_count {
            new_indexes.push(j);
        }
    }
    for j in 0..input_bits_2.len() {
        if input_bits_2[j] == 0 && zero_count_co2 <= one_count_co2 || input_bits_2[j] == 1 && zero_count_co2 > one_count_co2 {
            new_co2_indexes.push(j);
        }
    }

    //println!("new indexes {:?} {:?} {:?} {:?}", input_bits_1, new_indexes, input_bits_2, new_co2_indexes);
    (new_indexes, new_co2_indexes)
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let version = &args[1];
    let filename = &args[2];

    (version, filename)
}
