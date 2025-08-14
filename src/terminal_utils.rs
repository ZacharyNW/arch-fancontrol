use std::{io::{self, stdout, BufRead, Read, Write}, os::fd::AsRawFd, sync::{atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver}, Arc}, thread, time::Duration};
use libc::{self, termios as Termios};
use crate::{hwmon::fans::Fan, terminal_utils};

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
            return s.trim().to_string();
        } else {
            continue;
        }
    }
}

pub fn get_yes_no_selection<T>(prompt: &str, on_empty: T) -> bool
where T : Fn() -> bool {
    loop {
        let input = read_string(format!("{prompt}").as_str());

        if input.is_empty() {
            return on_empty();
        }

        if input == "Y" || input == "y" {
            return true;
        } else if input == "N" || input == "n"{
            return false;
        }
    }
}

pub fn get_yes_no_selection_default_yes(prompt: &str) -> bool {
    get_yes_no_selection(format!("{prompt} (Y)/n").as_str(), || true)
}


pub fn get_yes_no_selection_default_no(prompt: &str) -> bool {
   get_yes_no_selection(format!("{prompt} y/(N)").as_str(), || false)
}

pub fn wait_for_user_input() {
 let stdin = io::stdin();
    let fd = stdin.as_raw_fd();

    // Save original terminal settings
    let mut orig = unsafe { std::mem::zeroed::<Termios>() };
    unsafe {
        libc::tcgetattr(fd, &mut orig);
    }
    let mut raw = orig;

    // Disable canonical mode and echo
    raw.c_lflag &= !(libc::ICANON | libc::ECHO);
    raw.c_cc[libc::VMIN] = 1;  // return after 1 byte
    raw.c_cc[libc::VTIME] = 0; // no timeout

    unsafe {
        libc::tcsetattr(fd, libc::TCSANOW, &raw);
    }

    print!("Press any key to continue...");
    io::stdout().flush().ok();

    let _ = stdin.lock().bytes().next(); // read and discard one byte

    // Restore original settings
    unsafe {
        libc::tcsetattr(fd, libc::TCSANOW, &orig);
    }
    println!();
}

pub fn spawn_live_fan_speed_thread(fans: Arc<Vec<Fan>>, header: Arc<String>) -> Arc<AtomicBool>{
    let stop_flag = Arc::new(AtomicBool::new(false));
    let fans_clone = Arc::clone(&fans);
    let header_clone = Arc::clone(&header);
    let stop_flag_clone = Arc::clone(&stop_flag);

    let _ = thread::spawn(move || -> std::io::Result<()> {
        let mut buffer = String::new();
        let mut sorted_fans: Vec<_> = fans_clone.iter().collect();
        sorted_fans.sort_by(|f1, f2| f1.index.cmp(&f2.index));

        loop {
            if stop_flag_clone.load(Ordering::Relaxed) {
                break;
            }

            buffer.push_str(&header_clone);
            for fan in sorted_fans.iter() {
                buffer.push_str(format!("{}: {} \n", fan.label, fan.get_formatted_speed()).as_str());
            }

            terminal_utils::clear_terminal();
            print!("{}", buffer);
            buffer.clear();

            thread::sleep(Duration::from_millis(100));
        }

        Ok(())
    });

    return stop_flag;
}


pub fn spawn_live_fan_speed_thread_with_selection(fans: Arc<Vec<Fan>>, header: Arc<String>) -> (Arc<AtomicBool>, Receiver<usize>)
{
    let stop_flag = Arc::new(AtomicBool::new(false));
    let fans_clone = Arc::clone(&fans);
    let header_clone = Arc::clone(&header);
    let stop_flag_clone = Arc::clone(&stop_flag);

    let _ = thread::spawn(move || -> std::io::Result<()> {
        let mut buffer = String::new();
        let mut sorted_fans: Vec<_> = fans_clone.iter().collect();
        sorted_fans.sort_by(|f1, f2| f1.index.cmp(&f2.index));

        loop {
            if stop_flag_clone.load(Ordering::Relaxed) {
                break;
            }

   
            buffer.push_str(format!("{} \n", &header_clone).as_str());
            for fan in sorted_fans.iter() {
                if fan.get_speed().abs_diff(fan.current_speed) > 200 {
                    buffer.push_str(format!("\x1b[32m{}: {}\x1b[0m - {} (was {}) \n", fan.index, fan.label, fan.get_formatted_speed(), fan.get_formatted_cached_speed().as_str()).as_str());
                } else {
                    buffer.push_str(format!("{}: {} - {} (was {}) \n", fan.index, fan.label, fan.get_formatted_speed(), fan.get_formatted_cached_speed().as_str()).as_str());
                }
            }
            buffer.push_str("\nSelect fan that has changed speed, or {enter} if none\n");

            terminal_utils::clear_terminal();
            print!("{}", buffer);
            buffer.clear();

            thread::sleep(Duration::from_millis(100));
        }

        Ok(())
    });


    // TODO: This seems to have an issue releasing control of the lines lock.
    let (tx, rx) = mpsc::channel::<usize>();
    {
        let stop_flag_clone = Arc::clone(&stop_flag);

        thread::spawn(move || {
            let stdin = io::stdin();
            let mut lines = stdin.lock().lines();

            while let Some(Ok(line)) = lines.next() && !stop_flag_clone.load(Ordering::Relaxed) {
                let s = line.trim();
                if s.is_empty() {
                    stop_flag_clone.store(true, Ordering::Relaxed);
                    break;
                }
                match s.parse::<usize>() {
                    Ok(i) => {
                        let _ = tx.send(i);
                        stop_flag_clone.store(true, Ordering::Relaxed);
                    }
                    Err(_) => {
                        eprintln!("Invalid index: {}", s);
                        stop_flag_clone.store(true, Ordering::Relaxed);
                    }
                }
            }
        });
    }


    return (stop_flag, rx);
}


pub fn clear_terminal() {
    std::process::Command::new("clear").status().unwrap();
}