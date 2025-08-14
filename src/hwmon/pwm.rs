use std::{env, fs, io, path::PathBuf, process::{self, Command}};

use crate::path_helpers;

#[derive(Clone)]
pub struct Pwm {
    file_path: PathBuf,
    pub index: String,
    pub name: String,
}

impl Pwm {
    pub fn new(path: PathBuf) -> Self {
        Self { file_path: path, index: "".into(), name: "".into() }
    }

    pub fn with_index(mut self, index: String) -> Self {
        self.index = index;
        return self;
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        return self;
    }

    pub fn write_speed(&self, new_speed: i32){
        if new_speed > 255 {
            return;
        }

        let write_res = fs::write(self.get_input_path(), format!("{new_speed}"));

        match write_res {
            Ok(_) => { return }
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied =>  {
                    eprintln!("Need root to write {}. Re-running with sudo…", self.file_path.display());
                    let exe = env::current_exe().unwrap_or_default();
                    let args = env::args().skip(1);
                    let status = Command::new("sudo").arg(exe).args(args).status().unwrap_or_default();
                    process::exit(status.code().unwrap_or(1));
            } 
            Err(e) => eprintln!("{e} \n Error writing PWM value to file for {}", self.name)
        }
    }

    pub fn get_speed(&self) -> String {
        let speed = path_helpers::read_from_path(&self.get_input_path());
        return format!("{speed}");
    }

    fn get_input_path(&self) -> PathBuf {
        self.file_path.join(format!("pwm{}", self.index))
    }
}