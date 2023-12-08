use std::{
    io::{self, BufRead},
    str::Chars,
};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines().peekable();
    let mut prev_line: Vec<char> = ".".repeat(140).chars().collect();
    let mut current_line: Vec<char> = ".".repeat(140).chars().collect();
    while let Some(Ok(line)) = lines.next() {
        current_line = line.chars().collect();
        let mut next_line: Vec<char> = ".".repeat(140).chars().collect();
        match lines.peek() {
            Some(Ok(l)) => next_line = l.chars().collect(),
            Some(Err(_)) => (),
            None => (),
        }
        for i in 0..current_line.len() {
            let char = (*current_line)[i];
            // dbg!(char);
            if char.is_digit(10) {};
        }
        prev_line = current_line;
    }
}
