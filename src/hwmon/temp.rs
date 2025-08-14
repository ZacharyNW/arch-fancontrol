use std::path::PathBuf;

use crate::path_helpers;

pub struct Temp {
    file_path: PathBuf,
    pub index: String,
    pub label: String,
}

impl Temp {
    pub fn new(path: PathBuf) -> Self {
        Self {file_path: path, index: "".into(), label: "".into()}
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = label;
        return self;
    }

    pub fn with_index(mut self, index: String) -> Self {
        self.index = index;
        return self;
    }

    pub fn get_temp(&self) -> String {
        let temp_milli_celcius = self.get_current_value().parse::<f32>().unwrap_or(0.0);
        let temp_celcius = temp_milli_celcius / 1000.0;

        return format!("{temp_celcius} °C")
    }

    // pub fn edit_label(self, label: String){
    //     self.with_label(label);
    //     //TODO: write to file
    // }

    fn get_current_value(&self) -> String {
        path_helpers::read_from_path(self.get_input_path().as_path())
    }

    fn get_input_path(&self) -> PathBuf {
        self.file_path.join(format!("temp{}_input", self.index))
    }
}

