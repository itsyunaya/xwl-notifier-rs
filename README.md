# xwl-notifier-rs
xwl-notifier-rs (short for Xwayland-notifier) is a tiny program to alert you of new X11 apps opening, rewritten in Rust because I'm bad at C.

## Why?
In the past, I've had many programs I installed on my Computer run really badly, 
only to figure out every single time that it's because they run under Xwayland, 
and in turn X11, by default. So I created the original xwl-notifier in C, to send a 
desktop notification every time a new X11 window opens, which kind of worked for a while. 

Unfortunately it had some issues that I couldn't be bothered to fix, because I'm 
neither very good at programming in C, nor good at trying to make sense of Xlib, 
so I decided to rewrite it in Rust.

## Installation
### From source
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

### From Nix Flake
Add to your Flake inputs:
```nix
inputs = {
    xwl-notifier = {
        url = "github:itsyunaya/xwl-notifier-rs";
        inputs.nixpkgs.follows = "nixpkgs";
    };
};
```

Place in your package list:
```nix
environment.systemPackages = [
    inputs.xwl-notifier.packages.${pkgs.stdenv.hostPlatform.system}.default
];
```

And rebuild.

## Usage
Use any method to make it run on startup and enjoy! It should send a desktop 
notification every time a new X11 window opens (obviously requires a notification 
daemon to be installed)

## Afterword
In case of problems with the program or improvement suggestions, 
please open a GitHub Issue
