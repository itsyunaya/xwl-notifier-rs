#![allow(clippy::needless_return)]

mod blacklist;

use std::{error::Error, time::Duration, thread};

use notify_rust::Notification;
use x11_get_windows::Session;

use crate::blacklist::{init_blacklist_file, read_blacklist};

fn main() -> Result<(), Box<dyn Error>> {
    let blacklist: Vec<String>;

    if let Err(e) = init_blacklist_file() {
        eprintln!("Failed to initialize blacklist file: {} \nFalling back to none", e);
        blacklist = Vec::new();
    } else {
        blacklist = read_blacklist()?;
    }

    let mut session = Session::open().unwrap();
    let mut windows_vector: Vec<String> = Vec::new();

    loop {
        let current_titles: Vec<String> = session
            .get_windows()
            .expect("Could not get a list of windows.")
            .iter()
            .filter_map(|x| x.get_title(&session.display).ok())
            .map(|x| x.as_ref().to_str().unwrap().to_string())
            .collect();

        windows_vector.retain(|x| current_titles.contains(x));

        for title in current_titles {
            if !windows_vector.contains(&title) {
                if !is_subwindow(&title, &windows_vector, &blacklist) {
                    on_window_added(&title)?;
                }

                windows_vector.push(title);
            }
        }

        thread::sleep(Duration::from_secs(1));
    }
}

fn on_window_added(title: &str) -> Result<(), Box<dyn Error>> {
    Notification::new()
        .summary("xwl-notifier-rs")
        .body(&format!("New X11 Window '{}'", title))
        .show()?;

    return Ok(());
}

// this isn't perfect, as not every subwindow will have the original window's name in the title too,
// but it's the best I can do right now
fn is_subwindow(title: &str, win_vec: &Vec<String>, blacklist: &[String]) -> bool {
    for entry in win_vec {
        if title.contains(entry) || blacklist.contains(&title.to_string()) {
            return true;
        }
    }

    return false;
}
