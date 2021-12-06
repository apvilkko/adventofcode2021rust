use regex::Regex;
use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Board {
    data: Vec<u16>,
}
#[derive(Debug)]
struct Game {
    draw: Vec<u16>,
    boards: Vec<Board>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (version, filename) = parse_config(&args);
    println!("In file {}", filename);
    let input = read_input(filename);
    if version == "1" {
        return v1(&input, true);
    }
    return v1(&input, false);
}

fn v1(game: &Game, do_break: bool) {
    let mut t = 0;
    let mut has_winner = false;
    let mut winners: Vec<usize> = Vec::new();
    println!("number of boards {}", game.boards.len(),);
    loop {
        let draw_value = game.draw[t];
        for i in 0..game.boards.len() {
            if !winners.contains(&i) {
                let mut has_won = false;
                let mut score = 0;
                let (has_won1, score1) = wins(&game.boards[i], &game.draw[0..t + 1], false);
                has_won = has_won1;
                score = score1;
                if !has_won {
                    let (has_won2, score2) = wins(&game.boards[i], &game.draw[0..t + 1], true);
                    has_won = has_won2;
                    score = score2;
                }
                if has_won {
                    println!(
                        "round {} board {} won with score {}, drawn {}",
                        t, i, score, draw_value
                    );
                    winners.push(i);
                    has_winner = do_break;
                    if do_break {
                        break;
                    }
                }
            }
        }
        t += 1;
        if has_winner || t >= game.draw.len() {
            break;
        }
    }
}

fn get_score(board: &Board, draw: &[u16]) -> u32 {
    let last: u32 = *draw.last().unwrap() as u32;
    board
        .data
        .iter()
        .filter(|x| !draw.contains(x))
        .map(|x| *x as u32)
        .sum::<u32>()
        * last
}

fn wins(board: &Board, draw: &[u16], x: bool) -> (bool, u32) {
    let mut has_won = false;
    let mut score = 0;
    let dim = 5;
    if !x {
        for y in 0..dim {
            let bingo = board.data[y * dim..y * dim + dim]
                .iter()
                .filter(|x| draw.contains(x))
                .count()
                == dim;
            if bingo {
                has_won = true;
                score = get_score(board, draw);
                break;
            }
        }
    } else {
        for x in 0..dim {
            let mut column = Vec::new();
            for y in 0..dim {
                column.push(board.data[y * dim + x])
            }
            let bingo = column.iter().filter(|x| draw.contains(x)).count() == dim;
            if bingo {
                has_won = true;
                score = get_score(board, draw);
                break;
            }
        }
    }

    (has_won, score)
}

fn v2(input: &Game) {}

fn parse_config(args: &[String]) -> (&str, &str) {
    let version = &args[1];
    let filename = &args[2];

    (version, filename)
}

fn read_input(filename: &str) -> Game {
    let game_regex: Regex = Regex::new(r"(\d+(,\d+)*)((\s+\d+)+)").unwrap();
    let mut game = Game {
        draw: Vec::new(),
        boards: Vec::new(),
    };
    let file = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let filestr = file.trim();
    let caps: Vec<Game> = game_regex
        .captures_iter(&filestr)
        .filter_map(|cap| {
            let draw = cap.get(1).map_or("", |x| x.as_str()).trim();
            let boards = cap.get(3).map_or("", |x| x.as_str()).trim();
            let splitBoards: Vec<Board> = boards
                .split("\r\n\r\n")
                .map(|x| Board {
                    data: x
                        .trim()
                        .split_whitespace()
                        .map(|x| x.parse())
                        .flat_map(|x| x)
                        .collect(),
                })
                .collect();
            let drawVec: Vec<u16> = draw.split(",").map(|x| x.parse()).flat_map(|x| x).collect();
            //println!("draw {:?}\nboards {:?}", drawVec, splitBoards);
            game.draw = drawVec;
            game.boards = splitBoards;
            None
        })
        .collect();
    println!("{:?}", game);
    game
}
