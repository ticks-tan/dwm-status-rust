# rsblocks
A minimal fast dwm status bar written in **Rust** 🦀
<p>
<img align="center" src="./screenshots/1.png"/>
</p><br/>

## Features
* Time/Date
* Used Memory
* Used Disk space
* Sound volume _reads from `amixer` for now_
* Easy to configure
* Minimal

## Note
This tool is still in development stage.

## Install
clone the repo
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
**rsblocks** will try to read the file `~/.config/rsblocks/rsblocks.yml`, if it does not exist, it will load the defaults.

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
