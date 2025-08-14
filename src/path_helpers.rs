use std::{fs, io, path::Path};

pub(crate) trait ReadTrimmed {
    fn read_trimmed(&self) -> io::Result<String>;
}

impl ReadTrimmed for Path {
    fn read_trimmed(&self) -> io::Result<String> {
        read_trimmed(self)
    }
}

fn read_trimmed<P: AsRef<Path>>(p: P) -> io::Result<String> {
    let contents = fs::read_to_string(p)?;
    return Ok(contents.trim().to_string());
}

pub fn read_from_path(path: &Path) -> String {
    path.read_trimmed().unwrap_or_else(|_| "".into())
}

pub fn read_from_file(base_path: &Path, file_name: String) -> String {
    read_from_path(base_path.join(file_name).as_path())
}
