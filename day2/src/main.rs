use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut sum = 0;
    for line in handle.lines() {
        let line_str = line.ok().unwrap().to_string();
        let mut line_iter = line_str.split(':').into_iter();
        let _game_str = line_iter.next().unwrap().to_string();
        let data_sets_str = line_iter.next().unwrap().to_string();
        let data_iter = data_sets_str.split(&[',', ';']).map(|p| p.trim());
        let mut prev_min_red = 0;
        let mut prev_min_green = 0;
        let mut prev_min_blue = 0;
        for d in data_iter {
            let p = d.trim().split(' ').collect::<Vec<&str>>();
            let num = p[0].parse::<i32>().unwrap();
            match p[1] {
                "red" => {
                    if num > prev_min_red {
                        prev_min_red = num;
                    }
                }
                "blue" => {
                    if num > prev_min_blue {
                        prev_min_blue = num;
                    }
                }
                "green" => {
                    if num > prev_min_green {
                        prev_min_green = num;
                    }
                }
                _ => (),
            }
        }
        let pow = prev_min_blue * prev_min_green * prev_min_red;
        sum += pow;
    }
    dbg!(sum);
    Ok(())
}
