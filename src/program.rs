use std::io;

use crate::{hwmon::hwmon::Hwmon, terminal_utils};

pub fn print_initial_select(hwmons: &[Hwmon]) -> Result<usize, SelectError> {
    if hwmons.is_empty() {
        eprintln!("No hwmon");
        return Err(SelectError::Io(io::Error::new(io::ErrorKind::NotSeekable, "no hwmons found")));
    }

    println!("Select a module: ");
    for (i, h) in hwmons.iter().enumerate() {
        println!("{i}: {}", h.name);
    }

    let module_index = terminal_utils::read_usize("");

    if module_index < hwmons.len() {
        return Ok(module_index);
    } else {
        handle_selection_error(module_index, hwmons)
    }
}

fn handle_selection_error(module_index: usize, hwmons: &[Hwmon]) -> Result<usize, SelectError> {
    let mut idx = module_index;

    for attempt in 0..10 {
        if idx < hwmons.len() {
            return Ok(idx);
        }

        if attempt == 9 {
            return Err(SelectError::TooMany(10))
        }

        idx = terminal_utils::read_usize(&format!("{idx} not a valid selection, range is 0 - {}", hwmons.len() - 1));
    }
    unreachable!()
}

#[derive(Debug)]
pub enum SelectError {
    Io(io::Error),
    TooMany(usize),
}
impl std::fmt::Display for SelectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "err")
    }
}
impl std::error::Error for SelectError {}
impl From<io::Error> for SelectError {
    fn from(e: io::Error) -> Self { SelectError::Io(e) }
}
