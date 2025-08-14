use std::io::{self, Write};

pub fn read_usize(prompt: &str) -> usize {
    loop {
        print!("{prompt}");
        let _ = io::stdout().flush();

        let mut s = String::new();
        if io::stdin().read_line(&mut s).is_ok() {
            match s.trim().parse::<usize>() {
                Ok(n) => return n,
                Err(e) => eprintln!("Please enter a non-negative integer: {e}"),
            }
        }
    }
}

pub fn read_string_default(prompt: &str, default: &str) -> String {
    loop {
        let val = read_string(prompt);

        if val.trim().is_empty() {
            return default.to_string();
        }

        return val;
    }
}

pub fn read_string(prompt: &str) -> String {
    loop {
        println!("{prompt}");
        let _ = io::stdout().flush();

        let mut s = String::new();
        if io::stdin().read_line(&mut s).is_ok() {
            return s;
        } else {
            continue;
        }
    }
}

pub fn wait_for_user_input() {
    println!("Press any key to continue...");

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
}