use core::fmt;
use std::{fmt::{Display, Formatter}, fs, path::{Path, PathBuf}, thread, time::Duration};

use crate::{hwmon::{fans::Fan, pwm::Pwm, temp::Temp}, path_helpers::{self, ReadTrimmed}, terminal_utils};

pub struct Hwmon {
    path: PathBuf,
    pub name: String,
    pub fans: Vec<Fan>,
    pub temps: Vec<Temp>,
    pub pwms: Vec<Pwm>,
}

impl Hwmon {
    pub fn new(path: PathBuf, name: String) -> Self {
        Self {path: path, name: name, fans: Vec::new(), temps: Vec::new(), pwms: Vec::new()}
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn set_all_pwm(&self, pwm_value: i32) {
        for pwm in self.pwms.iter() {
            pwm.write_speed(pwm_value);
        }
    }

    pub fn try_pair_fans_to_pwm(&mut self) {
        for fan in self.fans.iter_mut() {
            fan.update_speed();
        }

        for pwm in self.pwms.iter() {
            std::process::Command::new("clear").status().unwrap();
            println!("Setting {} to max speed...", pwm.name);
            pwm.write_speed(255);

            thread::sleep(Duration::from_secs(5));

            let mut diff_requirement = 400;
            loop {
                println!("Searching for paired fan...");
                let mut possible_fans = self.fans
                .iter()
                .enumerate()
                .filter(|(_, fan)| fan.get_speed().abs_diff(fan.current_speed) > diff_requirement);

               let unique_index = match (possible_fans.next(), possible_fans.next()) {
                (Some((i, _)), None) => Some(i),
                _ => None
               };

               if let Some(i) = unique_index {
                    let fan: &mut Fan = &mut self.fans[i];
                    fan.paired_pwm = Some(pwm.clone());
                    println!("pwm {} matched to fan {}", pwm.name, fan.label);
                    terminal_utils::wait_for_user_input();
                    pwm.write_speed(150);
                    break;
               } else if diff_requirement > 1000 {
                    println!("Unable to match {}", pwm.name);
                    terminal_utils::wait_for_user_input();
                    pwm.write_speed(150);
                    break;
               } else {
                    diff_requirement += 100;
               }
            }
        }
    }

    pub fn initialize(&mut self) {
        self.initialize_fans();
        self.initialize_pwms();
        self.initialize_temps();
    }

    pub fn initialize_fans(&mut self) {
        let base_path = self.path();

        if let Ok(directory) = fs::read_dir(base_path){
            let mut list = Vec::new();
            for dir_entry in directory.flatten() {
                let path = dir_entry.path();
                if !is_file(&path) {continue;}

                let name = basename(&path);

                if let Some(index) = extract_index(name, "fan", "_input") {
                    let current_speed = &path.read_trimmed().unwrap_or_else(|_| "?".into());
                    let label = path_helpers::read_from_file(base_path, format!("fan{}_label", index));
                    let max = path_helpers::read_from_file(base_path, format!("fan{}_max", index));
                    let min = path_helpers::read_from_file(base_path, format!("fan{}_min", index));

                    list.push(Fan::new(base_path.to_path_buf())
                                    .with_current_speed(current_speed.parse::<i32>().unwrap_or(0))
                                    .with_index(index)
                                    .with_label(label)
                                    .with_rpm(min.parse().unwrap_or(0), max.parse().unwrap_or(0)));
                }
            }

            self.fans = list;
        }
    }

    pub fn initialize_temps(&mut self) {
        let base_path = self.path();

        if let Ok(directory) = fs::read_dir(base_path) {
            let mut list = Vec::new();
            for dir_entry in directory.flatten() {
                let path = dir_entry.path();
                if !is_file(&path) {continue;}

                let name = basename(&path);

                if let Some(index) = extract_index(name, "temp", "_input") {
                    let label = path_helpers::read_from_file(base_path, format!("temp{}_label", index));
                    
                    list.push(Temp::new(base_path.to_path_buf())
                                    .with_index(index)
                                    .with_label(label));
                }
            }

            self.temps = list;
        }
    }

    pub fn initialize_pwms(&mut self) {
        let base_path = self.path();
        
        if let Ok(directory) = fs::read_dir(base_path) {
            let mut list = Vec::new();
            for dir_entry in directory.flatten() {
                let path = dir_entry.path();
                if !is_file(&path) {continue;}

                let name = basename(&path);
                if name.ends_with("_enable") {continue;}


                if let Some(index) = extract_index(name, "pwm", "") {
                    list.push(Pwm::new(base_path.to_path_buf())
                                .with_index(index)
                                .with_name(name.to_string()));
                }
            }

            self.pwms = list;
        }
    }

    pub fn print_temps(&self) {
       println!("-- temps --"); 
       
       for temp in self.temps.iter(){
            println!("{}: {}", temp.label, temp.get_temp())
       }
    }

    pub fn print_fans(&self) {
       println!("-- fans --"); 
       
       for fan in self.fans.iter(){
            println!("{}: {}", fan.label, fan.get_speed())
       }
    }

    pub fn print_pwms(&self) {
        println!("-- pwms --");

        for pwm in self.pwms.iter(){
            println!("{}: {}", pwm.name, pwm.get_speed())
        }
    }
}

impl Display for Hwmon {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.path.display())
    }
}

fn basename<'a>(p: &'a Path) -> &'a str {
    p.file_name().and_then(|s| s.to_str()).unwrap_or("")
}

fn is_file(p: &Path) -> bool {
    fs::metadata(p).map(|m| m.is_file()).unwrap_or(false)
}

fn extract_index(name: &str, prefix: &str, suffix: &str) -> Option<String> {
    if !name.starts_with(prefix) || !name.ends_with(suffix) { return None; }
    let mid = &name[prefix.len()..name.len()-suffix.len()];
    if mid.chars().all(|c| c.is_ascii_digit()) { Some(mid.to_string()) } else { None }
}