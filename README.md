# xwl-notifier-rs
Tiny program to alert you of new X11 apps opening, rewritten in Rust because I'm bad at C

## Why?
In the past, I've had many programs I installed on my Computer run really badly, only to figure out every single time that it's because they run under Xwayland, and in turn X11, by default. So I made [xwl-notifier](https://github.com/itsyunaya/xwl-notifier) (short for Xwayland-notifier) to send a desktop notification every time a new X11 opens, which kind of worked for a while. 

Unfortunately it had some issues that I couldn't be bothered to fix, because I'm neither very good at programming in C, nor good at trying to make sense of Xlib, so I decided to rewrite it in Rust (because it's something I'm decent at).

## Installation
### From source
Note: requires nightly toolchain
```bash
cargo install --git https://github.com/itsyunaya/xwl-notifier-rs.git
# or
git clone https://github.com/itsyunaya/xwl-notifier-rs.git && cd xwl-notifier-rs
cargo install --path .
```
### From compiled binary
Download the latest release from [here](https://github.com/itsyunaya/xwl-notifier-rs/releases) and run the following commands on it:
```bash
chmod +x xwl-notifier-rs
sudo mv xwl-notifier-rs /usr/local/bin/
```

## Usage
Use any method to make it run on computer startup and enjoy! It should send a desktop notification every time a new X11 window opens (obviously requires a notification daemon to be installed)
### Examples
For Hyprland
```bash
# in hyprland.conf
exec-once = /path/to/xwl-notifier-rs
```
For KDE

`System Settings > System > Autostart > Add new`

## Credits and afterword
For querying open X11 windows, I used [this library](https://github.com/HiruNya/x11_get_windows) by HiruNya

Incase of problems with the program or improvement suggestions, please open a Github Issue
