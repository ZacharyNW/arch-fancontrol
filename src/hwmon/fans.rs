use std::path::PathBuf;

use crate::{hwmon::pwm::Pwm, path_helpers};

#[derive(Clone)]
pub struct Fan {
    file_path: PathBuf,
    pub index: i32,
    pub label: String,
    pub min_speed_rpm: i32,
    pub max_speed_rpm: i32,
    pub current_speed: i32,
    pub paired_pwm: Option<Pwm>,
}

impl Fan {
    pub fn new(path: PathBuf) -> Self {
        let p = path.clone();
        Self {file_path: path, index: 0, label: "".into(), max_speed_rpm: 0, min_speed_rpm: 0, current_speed: get_speed(p), paired_pwm: None }
    }

    pub fn with_label(mut self, s: String) -> Self {
        self.label = s;
        return self;
    }

    pub fn with_index(mut self, i: String) -> Self {
        self.index = i.parse::<i32>().unwrap_or(0);
        return self;
    }

    pub fn with_rpm(mut self, min: i32, max: i32) -> Self {
        self.min_speed_rpm = min;
        self.max_speed_rpm = max;
        return self;
    }

    pub fn with_current_speed(mut self, speed: i32) -> Self {
        self.current_speed = speed;
        return self;
    }

    // pub fn edit_label(self, new_value: String) {
    //     self.with_label(new_value);
    //     //TODO: write to file.
    // }

    pub fn get_speed(&self) -> i32{
        get_speed(self.get_input_path())
    }

    pub fn get_formatted_speed(&self) -> String {
        return format!("{} RPM", &self.get_speed());
    }

    pub fn get_formatted_cached_speed(&self) -> String {
        return format!("{} RPM", &self.current_speed);
    }

    pub fn update_speed(&mut self){
        self.current_speed = path_helpers::read_from_path(&self.get_input_path()).parse::<i32>().unwrap_or(0);
    }

    fn get_input_path(&self) -> PathBuf {
        self.file_path.join(format!("fan{}_input", self.index))
    }
}

fn get_speed(path: PathBuf) -> i32 {
    path_helpers::read_from_path(&path).parse::<i32>().unwrap_or(0)
}