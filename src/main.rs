#![allow(clippy::needless_return)]

use std::error::Error;
use std::time::Duration;
use notify_rust::Notification;

const KNOWN_DUPLICATES: [&str; 1] = [
    "steamwebhelper",
];

fn main() -> Result<(), Box<dyn Error>> {
    let mut active_windows: Vec<String> = Vec::new();

    loop {
        let windows = xwl_notifier::get_x11_windows();

        active_windows.retain(|x| windows.iter().any(|y| y.command == *x));

        for entry in windows {
            let cmd = entry.command;
            if !active_windows.contains(&cmd) && !KNOWN_DUPLICATES.contains(&cmd.as_str()) {
                notify(&cmd)?;
                active_windows.push(cmd);
            }
        }

        std::thread::sleep(Duration::from_secs(3));
    }
}

fn notify(title: &str) -> Result<(), Box<dyn Error>> {
    Notification::new()
        .summary("xwl-notifier-rs")
        .body(&format!("New X11 Window '{}'", title))
        .show()?;

    return Ok(());
}
