# rsblocks
A minimal multi threaded fast status bar for dwm window manager written in **Rust** 🦀
<p>
<img align="center" src="./screenshots/1.png"/>
</p><br/>

## Features
* Multi Threads
* Time/Date
* Used Memory
* Used Disk space
* Sound volume _reads from `alsa-utils` for now_
* Easy to configure with `rsblocks.yml` file


## Notes
* This tool is still in development stage.
* currently supports only linux.

## Cargo Installation
You can install the binary crate directly
```sh
cargo install rsblocks
```

## Manual Installation
You can clone the repo and build from the source code
```sh
git clone https://github.com/mustafasalih1993/rsblocks
```
build with **cargo**
```sh
cargo build --release
```
move the executable somewhere in your **PATH** (assuming you are in the root dir of the project)
```sh
mv ./target/release/rsblocks /usr/local/bin
```

you good to go now and can run `rsblocks` from your terminal or put that in your `.xinitrc`

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

## License
[MIT](./LICENSE)
