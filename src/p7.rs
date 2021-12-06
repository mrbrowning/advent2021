use std::collections::HashMap;
use std::convert::{identity, TryInto};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub type BoardCoordinate = (usize, usize);

#[derive(Debug)]
pub struct Board {
    numbers: [[u32; 5]; 5],
    marked: [[bool; 5]; 5],
    indexes: HashMap<u32, BoardCoordinate>,
    last_called: Option<u32>,
}

impl Board {
    pub fn new(numbers: [[u32; 5]; 5]) -> Board {
        let marked: [[bool; 5]; 5] = (0..5)
            .map(|_| {
                (0..5)
                    .map(|_| false)
                    .collect::<Vec<bool>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[bool; 5]>>()
            .try_into()
            .unwrap();

        Board {
            numbers,
            marked,
            indexes: Board::indexes_from_board(numbers),
            last_called: None,
        }
    }

    fn indexes_from_board(numbers: [[u32; 5]; 5]) -> HashMap<u32, BoardCoordinate> {
        let mut map: HashMap<u32, BoardCoordinate> = HashMap::new();
        for i in 0..5 {
            for j in 0..5 {
                map.insert(numbers[i][j], (i, j));
            }
        }

        map
    }

    fn check_row(&self, row: usize) -> bool {
        self.marked[row].iter().all(|b| *b)
    }

    fn check_col(&self, col: usize) -> bool {
        self.marked.iter().map(|r| r[col]).all(identity)
    }

    fn check_diagonals(&self) -> bool {
        let top_left = (0..5).map(|i| self.marked[i][i]).all(identity);
        let top_right = (0..5).map(|i| self.marked[i][4 - i]).all(identity);

        top_left && top_right
    }

    pub fn play(&mut self, number: u32) {
        let index = self.indexes.get(&number);

        match index {
            Some((i, j)) => {
                self.marked[*i][*j] = true;
                self.last_called = Some(number);
            }
            None => (),
        };
    }

    pub fn is_winning(&self) -> bool {
        let any_winning_row = (0..5).map(|row| self.check_row(row)).any(identity);
        let any_winning_col = (0..5).map(|col| self.check_col(col)).any(identity);
        let any_winning_diag = self.check_diagonals();

        any_winning_row || any_winning_col || any_winning_diag
    }

    pub fn sum(&self) -> Option<u32> {
        let mut sum: u32 = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.marked[i][j] {
                    sum += self.numbers[i][j];
                }
            }
        }

        match self.last_called {
            Some(x) => Some(x * sum),
            None => None,
        }
    }
}

fn parse_picks(line: &str) -> Vec<u32> {
    line.split(",").map(|s| s.parse::<u32>().unwrap()).collect()
}

fn parse_board(lines: &[String; 5]) -> Board {
    let numbers: [[u32; 5]; 5] = (0..5)
        .map(|i| {
            lines[i]
                .split(" ")
                .filter(|s| s.len() > 0)
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[u32; 5]>>()
        .try_into()
        .unwrap();

    Board::new(numbers)
}

pub fn parse_game(file: BufReader<File>) -> (Vec<Board>, Vec<u32>) {
    let mut lines = file.lines().peekable();
    let picks = parse_picks(&lines.next().unwrap().unwrap());

    let mut boards: Vec<Board> = vec![];
    loop {
        if lines.peek().is_none() {
            break;
        }

        let numbers: [String; 5] = (0..6)
            .map(|_| lines.next().unwrap().unwrap())
            .skip(1)
            .collect::<Vec<String>>()
            .try_into()
            .unwrap();

        boards.push(parse_board(&numbers));
    }

    (boards, picks)
}

pub fn run(args: &[String]) {
    let filename = match args.len() {
        0 => "raw_data/input3.txt",
        1 => args[0].as_str(),
        _ => args[0].as_str(),
    };

    let raw_file = File::open(filename);
    if raw_file.is_err() {
        println!("failed to load file: {:?}", raw_file);
        return;
    }

    let file = BufReader::new(raw_file.unwrap());
    let (mut boards, picks) = parse_game(file);

    let mut answer: Option<u32> = None;
    for (i, call) in picks.iter().enumerate() {
        let mut found = false;
        for board in boards.iter_mut() {
            board.play(*call);
            if i >= 5 && board.is_winning() {
                answer = board.sum();
                found = true;
            }
        }

        if found {
            break;
        }
    }

    println!("Answer is {}", answer.unwrap());
}
