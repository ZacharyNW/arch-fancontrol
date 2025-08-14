use std::{env, process::{self, Command}};

use crate::hwmon_service::HwmonService;

mod hwmon_service;
mod path_helpers; 
mod terminal_utils;
mod program;
mod hwmon;

fn main() {
    if !is_root() {
        restart_as_root();
    }

    terminal_utils::clear_terminal();

    let mut hwmon_service = HwmonService::new();
    hwmon_service.initialize_hwmons();

    let hwmon_index = match program::print_initial_select(&hwmon_service.hwmons) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("module selection failed: {e}");
            std::process::exit(1);
        }
    };

    terminal_utils::clear_terminal();

    let hwmon = match hwmon_service.hwmons.get_mut(hwmon_index) {
        Some(h) => h,
        None => {
            eprintln!("Error finding hwmon");
            process::exit(1);
        }
    };

    hwmon.initialize();
    hwmon.set_all_pwm(100);

    hwmon.print_temps();
    hwmon.print_fans();
    hwmon.print_pwms();

    if terminal_utils::get_yes_no_selection_default_yes("Attempt auto pairing?") {
        hwmon.try_pair_fans_to_pwm();
    } else {
        hwmon.manual_pair_fan_to_pwm();
    }

    terminal_utils::clear_terminal();
}

#[cfg(unix)]
fn is_root() -> bool {
    // SAFETY: libc::geteuid has no side effects
    unsafe { libc::geteuid() == 0 }
}

fn restart_as_root() {
    let exe = env::current_exe().unwrap_or_default();
    let args = env::args().skip(1);
    let status = Command::new("sudo").arg(exe).args(args).status().unwrap_or_default();
    process::exit(status.code().unwrap_or(1));
}