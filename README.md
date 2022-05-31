# Pancake Browser Launcher

Pancake is a simple browser launcher that allows users to select from various browser and browser profiles when opening a link. Profile selections can be saved for a domain or exact match URL. Pancake has been tested on various Linux distributions and Windows 10.

## Build

`cargo build --release`

## Setup

* Set `pancake` binary as your system's default browser.
  
  * This requires some registry entries on Windows. There are example reg files in the assets folder. Be sure to replace the `pancake` binary location with the one you selected.

* Create configuration file defining browser profiles that can be launched.
  
  * Windows config path: `C:\Users\Alice\AppData\Roaming\pancake\pancake.toml`
  
  * Linux config path: `/home/alice/.config/pancake/pancake.toml`
    
    

### Sample Config

```toml
theme = "default"

[[browsers]]
key = "brave-personal"
display = "Brave - Personal"
exec = ["/usr/bin/brave", "--profile-directory=Default"]

[[browsers]]
key = "chrome-personal"
display = "Google Chrome - Personal"
exec = ["/usr/bin/google-chrome-stable", "--profile-directory=Default"]

[[browsers]]
key = "chrome-work"
display = "Google Chrome - Work"
exec = ["/usr/bin/google-chrome-stable", "--profile-directory=Profile 1"]

[[browsers]]
key = "firefox-burp-proxy"
display = "Firefox - BurpSuite Proxy"
exec = ["/usr/bin/firefox", "-P", "BurpProxy"]

[[browsers]]
key = "firefox-private"
display = "Firefox - Private"
exec = ["/usr/bin/firefox", "--private-window"]
```

## Todos

* Automatically build the configuration file by finding browser profiles on the system.

* Setup automated package builds: pacman and Windows (msi or exe based installer).




