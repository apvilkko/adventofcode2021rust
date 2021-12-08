use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;
use std::fmt;

type Segment = [bool; 7];

#[derive(Debug, Copy, Clone)]
struct DisplayEntry {
    patterns: [Segment; 10],
    output: [Segment; 4]
}

static empty_segment: Segment = [false, false,false, false, false, false,false];

fn number_of_segments(s: &Segment)->u8 {
    s.iter().filter(|&x| *x).count() as u8
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

fn eq(s1: &Segment, s2: &Segment)->bool {
    for (i, x) in s1.iter().enumerate() {
        if *x != s2[i] {
            return false
        }
    }
    return true
}

fn or(s1: &Segment, s2: &Segment)->Segment {
    let mut out: Segment = empty_segment;
    for (i, x) in s1.iter().enumerate() {
        out[i] = x | s2[i]
    }
    out
}

fn and(s1: &Segment, s2: &Segment)->Segment {
    let mut out: Segment = empty_segment;
    for (i, x) in s1.iter().enumerate() {
        out[i] = x & s2[i]
    }
    out
}

fn not(s1: &Segment)->Segment {
    let mut out: Segment = empty_segment;
    for (i, x) in s1.iter().enumerate() {
        out[i] = !x
    }
    out
}

fn v1(input: &Vec<DisplayEntry>) {
    let mut count: [u16; 10] = [0,0,0,0,0,0,0,0,0,0];
    let mut sums: Vec<u32> = Vec::new();
    for entry in input {
        let mut five_segments = HashSet::new();
        let mut six_segments = HashSet::new();
        let mut zero = empty_segment;
        let mut one = empty_segment;
        let mut two = empty_segment;
        let mut three = empty_segment;
        let mut four = empty_segment;
        let mut five = empty_segment;
        let mut six = empty_segment;
        let mut seven = empty_segment;
        let mut eight = empty_segment;
        let mut nine = empty_segment;
        let mut e_segment = empty_segment;
        for segment in entry.patterns {
            let num = number_of_segments(&segment);
            match num {
                2 => {
                    one = segment;
                }
                3 => {
                    seven = segment;
                }
                4 => {
                    four = segment;
                }
                5 => {
                    five_segments.insert(segment);
                }
                6 => {
                    six_segments.insert(segment);
                }
                7 => {
                    eight = segment;
                }
                _ => {}
            }
        }
        for segment in entry.output {
            let num = number_of_segments(&segment);
            match num {
                2 => {
                    count[0] += 1;
                    one = segment;
                }
                3 => {
                    count[6] += 1;
                    seven = segment;
                }
                4 => {
                    count[3] += 1;
                    four = segment;
                }
                5 => {
                    five_segments.insert(segment);
                }
                6 => {
                    six_segments.insert(segment);
                }
                7 => {
                    count[7] += 1;
                    eight = segment;
                }
                _ => {}
            }
        }
        println!("one {:?}", one);
        println!("four {:?}", four);
        println!("seven {:?}", seven);
        let four_or_seven = or(&four,&seven);
        //println!("f|s {:?}", four_or_seven);
        let mut bailout = 100;
        loop {
            let three_found = !eq(&three, &empty_segment);
            let six_found = !eq(&six, &empty_segment);
            let nine_found = !eq(&nine, &empty_segment);
            let five_found = !eq(&five, &empty_segment);
            let e_segment_found = !eq(&e_segment, &empty_segment);
            for x in &five_segments {
                //println!("[5] {:?}", x);

                let with_1 = and(&x, &one);
                if number_of_segments(&with_1) == 2 {
                    three = *x;
                }
                if three_found && e_segment_found {
                    let three_e = or(&three, &e_segment);
                    let with_three_e = and(&x, &three_e);
                    //println!("{:?} {:?}", &x, with_three_e);
                    if number_of_segments(&with_three_e) == 4 {
                        five = *x;
                    }
                }
                if three_found && five_found && !eq(&x, &five) && !eq(&x, &three) {
                    two = *x;
                }
            }
            for x in &six_segments {
                //println!("[6] {:?}", x);
                let with_fs = and(&x, &four_or_seven);
                if number_of_segments(&with_fs) == 5 {
                    //println!(" nine {:?}", with_fs);
                    nine = *x;
                    e_segment = not(&nine);
                }
                //println!("    {:?}", with_fs);
                if five_found && e_segment_found {
                    let five_e = or(&five, &e_segment);
                    if eq(&x, &five_e) {
                        six = *x;
                    }
                }
                if six_found && nine_found && !eq(&x, &six) && !eq(&x, &nine) {
                    zero = *x;
                }
            }
            bailout -= 1;
            if bailout == 0 {
                break;
            }
        }
        println!("zero {:?}", zero);
        println!("two {:?}", two);
        println!("three {:?}", three);
        println!("five {:?}", five);
        println!("six {:?}", six);
        println!("nine {:?}", nine);
        println!("");
        let segments = [zero, one, two, three, four, five, six, seven, eight, nine];
        let pos = [1000, 100, 10, 1];
        let mut sum: u32 = 0;
        for (i, segment) in entry.output.iter().enumerate() {
            let val = as_number(&segment, &segments);
            sum += (pos[i] * (val as u32));
        }
        println!("sum of values {}", sum);
        sums.push(sum);
    }
    let total: u16 = count.iter().sum();
    println!("part 1 counts {:?} {}", count, total);
    let total_sums: u32 = sums.iter().sum();
    println!("part 2 sums {}", total_sums);
}

fn as_number(s: &Segment, segments: &[Segment; 10])->u8 {
    for (i, x) in segments.iter().enumerate() {
        if eq(&s, &x) {
            return i as u8
        }
    }
    return 0
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let version = &args[1];
    let filename = &args[2];

    (version, filename)
}

fn to_segment(x: &str) -> Segment {
    [x.contains('a'),
        x.contains('b'),
        x.contains('c'),
        x.contains('d'),
        x.contains('e'),
        x.contains('f'),
        x.contains('g')]
}

fn read_input(filename: &str) -> Vec<DisplayEntry> {
    let contents: Vec<DisplayEntry> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .trim()
        .lines()
        .map(|line| {
            let parts : Vec<&str> = line.split("|").collect();
            DisplayEntry {
                patterns: parts[0].split_whitespace()
                .map(to_segment).collect::<Vec<Segment>>().try_into().unwrap(),
                output: parts[1].split_whitespace()
                .map(to_segment).collect::<Vec<Segment>>().try_into().unwrap(),
            }
        })
        .collect();

    contents
}

/*

 0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

6, 0 d
2,  1
5, 2 bf
5, 3 be
4,  4
5, 5 ce
6, 6 c
3,  7
7,  8
6  9 e

*/