# dwm-status-rust

**!!! This project clone [rsblocks](https://github.com/MustafaSalih1993/rsblocks) ，add some new education, update irregular !**

A status bar for dwm window manager written in **Rust** 🦀
<p>
<img align="center" src="./screenshots/2.png"/>
</p><br/>

## Features
* Async
* Battery Percentage
* Bitcoin Price
* Brightness
* Cpu Temperature
* Disk Usage
* Load Average
* Local Ip Address
* Memory Usage
* Mpd Current Song
* Net Usage
* Public Ip Address
* Sound Volume
* Spotify Current Song
* Time/Date
* Uptime
* Weather Temperature
* Easy to configure with `rsblocks.yml` file
* Restart after config file modified [New]


## Notes
* This tool is still in development stage.
* supports only linux.


## Manual Installation
You can clone the repo and build from the source code
```sh
git clone https://github.com/ticks-tan/dwm-status-rust.git
```
build with **cargo**
```sh
cargo check
cargo build --release
```
move the executable somewhere in your **PATH** (assuming you are in the root dir of the project)
```sh
# a simple example
mv ./target/release/rsblocks /usr/local/bin
```

then you good to go now and can run `rsblocks` from your terminal or put that in your `.xinitrc`

## Configuration
#### Notes:
* **rsblocks** will try to read the file `$HOME/.config/rsblocks/rsblocks.yml`, if it does not exist, it will load the defaults.
* **rsblocks** will read the configuration file **only** on startup, which means you have to kill it and start it again if you updated your `rsblocks.yml` file.

create the directory
```sh
mkdir ~/.config/rsblocks
```

copy the [template](./rsblocks.yml) to the new config directory (assuming you are in the root dir of the repo)
```sh
cp ./rsblocks.yml ~/.config/rsblocks/rsblocks.yml
```


## Contributions
All Contributions are welcome.

## Credits
* [wttr.in](https://github.com/chubin/wttr.in) for using their weather API

## License
[MIT](./LICENSE)
