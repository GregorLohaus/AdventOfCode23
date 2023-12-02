use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
fn str_num_to_num_str() -> &'static HashMap<&'static str, &'static str> {
    static HASHMAP: OnceLock<HashMap<&str, &str>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut h: HashMap<&str, &str> = HashMap::new();
        h.insert("one", "1");
        h.insert("two", "2");
        h.insert("three", "3");
        h.insert("four", "4");
        h.insert("five", "5");
        h.insert("six", "6");
        h.insert("seven", "7");
        h.insert("eight", "8");
        h.insert("nine", "9");
        h
    })
}
fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let sum = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for line in handle.lines() {
        let l = line.ok().unwrap();
        let cloned = sum.clone();
        let handle = thread::spawn(move || {
            let mut locked = cloned.lock().unwrap();
            let re = Regex::new(r"(\d|one|three|four|five|six|seven)").unwrap();
            let re2 = Regex::new(r"eight").unwrap();
            let re3 = Regex::new(r"(nine|two)").unwrap();
            let mut matches: Vec<regex::Match> = re.find_iter(l.as_str()).collect();
            let mut matches2: Vec<regex::Match> = re2.find_iter(l.as_str()).collect();
            let mut matches3: Vec<regex::Match> = re3.find_iter(l.as_str()).collect();
            matches.append(&mut matches2);
            matches.append(&mut matches3);
            matches.sort_by_cached_key(|k| k.start());
            let mut num = String::new();
            let d1_str: &str;
            let d2_str: &str;
            if matches[0].len() > 1 {
                d1_str = str_num_to_num_str().get(matches[0].as_str()).unwrap();
            } else {
                d1_str = matches[0].as_str()
            }
            if matches.len() < 2 {
                d2_str = d1_str;
            } else {
                if matches[matches.len() - 1].len() > 1 {
                    d2_str = str_num_to_num_str()
                        .get(matches[matches.len() - 1].as_str())
                        .unwrap();
                } else {
                    d2_str = matches[matches.len() - 1].as_str();
                }
            }
            num = num + d1_str + d2_str;
            // dbg!(l.as_str(), d1_str, d2_str, num.clone());
            *locked += num.parse::<i32>().unwrap();
        });
        handles.push(handle)
    }
    for handle in handles {
        handle.join().unwrap()
    }
    io::stdout()
        .write(sum.try_lock().unwrap().to_owned().to_string().as_bytes())
        .unwrap();
    Ok(())
}
