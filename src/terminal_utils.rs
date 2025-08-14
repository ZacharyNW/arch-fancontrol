use std::{io::{self, stdout, Write}, sync::{atomic::{AtomicBool, Ordering}, Arc}, thread, time::Duration};
use crate::hwmon::fans::Fan;

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

pub fn spawn_live_fan_speed_thread(fans: Arc<Vec<Fan>>, stop_flag: &Arc<AtomicBool>){
    let fans_clone = Arc::clone(&fans);
    let stop_flag_clone = Arc::clone(&stop_flag);

    let _ = thread::spawn(move || -> std::io::Result<()> {
        loop {
            if stop_flag_clone.load(Ordering::Relaxed) {
                break;
            }

            std::process::Command::new("clear").status().unwrap();
            println!("Fan Speeds (refresh: 100ms)\n");

            for fan in fans_clone.iter() {
                println!("{}: {}", fan.label, fan.get_formatted_speed());
            }

            stdout().flush().ok();
            thread::sleep(Duration::from_millis(100));
        }

        Ok(())
    });
}