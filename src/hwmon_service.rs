use std::{fs, io, path::Path};
use crate::{hwmon::hwmon::Hwmon, path_helpers::ReadTrimmed};

pub struct HwmonService {
    pub hwmons: Vec<Hwmon>
}

impl HwmonService {
    pub fn new() -> Self {
        Self {hwmons: get_hwmons()}
    }

    pub fn initialize_hwmons(&mut self) {
        for hwmon in self.hwmons.iter_mut() {
            hwmon.initialize();
        }
    }
}


fn get_hwmons() -> Vec<Hwmon> {
    let hwmons = match collect_hwmon() {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Error: {e}");
            return Vec::new();
        }
    };

    if hwmons.is_empty() {
            eprintln!("No hwmon");
            return Vec::new();
    } 

    return hwmons
}

fn collect_hwmon() -> io::Result<Vec<Hwmon>> {
    let root = Path::new("/sys/class/hwmon");
    let mut list = Vec::new();

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();

        if !path.file_name()
            .and_then(|f| f.to_str())
            .map(|s| s.starts_with("hwmon"))
            .unwrap_or(false)
        {
            continue;
        }

        if let Ok(name) = path.join("name").read_trimmed(){
            list.push(Hwmon::new(path, name))
        }
    }

    list.sort_by(|a, b| a.path().cmp(&b.path()));
    return Ok(list);
}