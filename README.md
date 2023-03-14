<div align="center">

# ff2mpv-rust

</div>

Native messaging host for ff2mpv written in Rust.

This is a native compiled binary, so it runs much faster and doesn't require external dependencies by itself, unlike Python and Ruby scripts.

# What is this?
This is a script for hooking mpv and [ff2mpv](https://github.com/woodruffw/ff2mpv), that allows opening any video link in mpv straight from the browser.

# Installation
First, install ff2mpv extension from [AMO](https://addons.mozilla.org/en-US/firefox/addon/ff2mpv) or [Chrome Store](https://chrome.google.com/webstore/detail/ff2mpv/ephjcajbkgplkjmelpglennepbpmdpjg).

After that get native messasing host manifest:
```
ff2mpv-rust manifest
```
Install it following manual installation instructions on [ff2mpv wiki](https://github.com/woodruffw/ff2mpv/wiki).

# Configuration
On Linux configuration file is searched in such order:

1. $XDG_CONFIG_HOME/ff2mpv-rust.json
2. $HOME/.config/ff2mpv-rust.json
3. /etc/ff2mpv-rust.json

On Windows configuration file should be placed at: %APPDATA%/ff2mpv-rust.json.

See [example configuration](ff2mpv-rust.json).

# Command line interface
ff2mpv-rust provides command line interface with following commands:
```
help: prints help message
manifest: prints manifest for browser configuration
validate: checks configration file for validity
```
Note that it won't fail on invalid commands, but instead assume it is called from browser, blocking the input.

# Contributing

All issues and pull requests are welcome! Feel free to open an issue if you've got an idea or a problem. You can open a pull request if you are able to implement it yourself.

---
<p align="center">
<sub><strong>
    Made with ponies and love!
    <br/>
    GNU GPL © Ryze 2023
</strong></sub>
</p>
