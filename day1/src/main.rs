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
            let re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
            let mut digits = re.find_iter(l.as_str());
            let mut num = String::new();
            match digits.next() {
                Some(d1) => {
                    let d1_str: &str;
                    if d1.len() > 1 {
                        match str_num_to_num_str().get(d1.as_str()) {
                            Some(str) => d1_str = str,
                            None => {
                                dbg!("str_num_to_num_str did not return anythinh");
                                panic!()
                            }
                        }
                    } else {
                        d1_str = d1.as_str();
                    }
                    match digits.last() {
                        Some(d2) => {
                            let d2_str: &str;
                            if d2.len() > 1 {
                                match str_num_to_num_str().get(d2.as_str()) {
                                    Some(str) => d2_str = str,
                                    None => {
                                        dbg!("str_num_to_num_str did not return anythinh");
                                        panic!()
                                    }
                                }
                            } else {
                                d2_str = d2.as_str();
                            }
                            num = num + d1_str + d2_str;
                            dbg!(l.clone(), num.clone(), d1_str, d2_str);
                        }
                        None => {
                            dbg!(l.clone(), num.clone(), d1_str, d1_str);
                            num = num + d1_str + d1_str;
                        }
                    }
                }
                None => panic!(),
            }
            if let Ok(int) = num.parse::<i32>() {
                *locked += int;
                dbg!(*locked);
            } else {
                dbg!("could not parse", num);
                panic!()
            }
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
