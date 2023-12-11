use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines().peekable();
    let mut prev_line: Vec<char> = ".".repeat(140).chars().collect();
    let mut num: i32;
    let mut sum: i32 = 0;
    while let Some(Ok(line)) = lines.next() {
        let current_line: Vec<char> = line.chars().collect::<Vec<char>>();
        let mut next_line: Vec<char> = ".".repeat(140).chars().collect();
        match lines.peek() {
            Some(Ok(l)) => next_line = l.chars().collect(),
            Some(Err(_)) => (),
            None => (),
        }
        if next_line.len() == 0 {
            next_line = ".".repeat(140).chars().collect();
        }
        if current_line.len() > 0 {
            let len = current_line.len() - 1;
            let e: usize = len - 1;
            let mut skips: i32 = 0;
            for i in 0..len {
                if skips > 0 {
                    skips -= 1;
                    continue;
                }
                let char = (*current_line)[i];
                if char.is_digit(10) {
                    match i {
                        0 => {
                            if is_special_char(next_line[i])
                                || is_special_char(next_line[i + 1])
                                || is_special_char(prev_line[i])
                                || is_special_char(prev_line[i + 1])
                                || is_special_char(current_line[i + 1])
                            {
                                (num, skips) = collect_num_and_skips_from_index(&current_line, i);
                                sum += num;
                            }
                        }
                        _e if _e == e => {
                            if is_special_char(next_line[i])
                                || is_special_char(next_line[i - 1])
                                || is_special_char(prev_line[i])
                                || is_special_char(prev_line[i - 1])
                                || is_special_char(current_line[i - 1])
                            {
                                (num, skips) = collect_num_and_skips_from_index(&current_line, i);
                                sum += num;
                            }
                        }
                        _ => {
                            if is_special_char(next_line[i])
                                || is_special_char(next_line[i - 1])
                                || is_special_char(next_line[i + 1])
                                || is_special_char(prev_line[i])
                                || is_special_char(prev_line[i - 1])
                                || is_special_char(prev_line[i + 1])
                                || is_special_char(current_line[i - 1])
                                || is_special_char(current_line[i + 1])
                            {
                                (num, skips) = collect_num_and_skips_from_index(&current_line, i);
                                sum += num;
                            }
                        }
                    }
                };
            }
        } else {
            continue;
        }
        prev_line = current_line;
    }
    dbg!(sum);
}

fn is_special_char(char: char) -> bool {
    !char.is_digit(10) && char != '.'
}

fn collect_num_and_skips_from_index(vec: &Vec<char>, index: usize) -> (i32, i32) {
    let mut nums_forward: String = String::from("");
    let mut iter_forward = (*vec).split_at(index).1.iter().peekable();
    while let Some(char) = iter_forward.next_if(|char| char.is_digit(10)) {
        nums_forward.push(*char);
    }
    let mut nums_backwards: String = String::from("");
    let mut iter_backwards = (*vec).split_at(index).0.iter().rev().peekable();
    while let Some(char) = iter_backwards.next_if(|char| char.is_digit(10)) {
        nums_backwards.push(*char);
    }
    nums_backwards = nums_backwards.chars().rev().collect();
    nums_backwards.push_str(&nums_forward);
    (
        nums_backwards.parse::<i32>().unwrap(),
        (nums_forward.len() - 1).try_into().unwrap(),
    )
}

#[test]
fn collect_num_from_index_test() {
    let a = vec!['q', 'r', '1', 'r', '5', '6', 't', '#', '8', '0', '1', '?'];
    let a_num_2 = collect_num_and_skips_from_index(&a, 2);
    let a_num_4 = collect_num_and_skips_from_index(&a, 4);
    let a_num_5 = collect_num_and_skips_from_index(&a, 5);
    let a_num_8 = collect_num_and_skips_from_index(&a, 8);
    let a_num_9 = collect_num_and_skips_from_index(&a, 9);
    let a_num_10 = collect_num_and_skips_from_index(&a, 10);
    assert_eq!(a_num_2, (1, 0));
    assert_eq!(a_num_4, (56, 1));
    assert_eq!(a_num_5, (56, 0));
    assert_eq!(a_num_8, (801, 2));
    assert_eq!(a_num_9, (801, 1));
    assert_eq!(a_num_10, (801, 0));
}
