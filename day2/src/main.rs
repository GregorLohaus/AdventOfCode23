use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() -> io::Result<()> {
    let mut rules: HashMap<&str, i32> = HashMap::new();
    rules.insert("red", 12);
    rules.insert("blue", 14);
    rules.insert("green", 13);
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut sum = 0;
    for line in handle.lines() {
        let line_str = line.ok().unwrap().to_string();
        let mut line_iter = line_str.split(':').into_iter();
        let game_str = line_iter.next().unwrap().to_string();
        let id_str = game_str.split(' ').last().unwrap().to_string();
        let data_sets_str = line_iter.next().unwrap().to_string();
        let data_iter = data_sets_str.split(&[',', ';']).map(|p| p.trim());
        let invalid = data_iter.filter(|d| {
            let p = d.trim().split(' ').collect::<Vec<&str>>();
            &(p[0].parse::<i32>().unwrap()) > rules.get(p[1]).unwrap()
        });
        let valid = invalid.collect::<Vec<&str>>().len() == 0;
        if valid {
            sum += id_str.parse::<i32>().unwrap();
        }
    }
    dbg!(sum);
    Ok(())
}
