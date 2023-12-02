use regex::Regex;
use std::io::{self, BufRead, Write};
use std::sync::{Arc, Mutex};
use std::thread;
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
            let re = Regex::new(r"\d").unwrap();
            let mut digits = re.find_iter(l.as_str());
            let mut num = String::new();
            match digits.next() {
                Some(d1) => match digits.last() {
                    Some(d2) => num = num + d1.as_str() + d2.as_str(),
                    None => num = num + d1.as_str() + d1.as_str(),
                },
                None => (),
            }
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
