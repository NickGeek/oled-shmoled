# OLED Shmoled
OLED displays don't have backlights but it'd be nice if we could pretend they do on Linux!

This program aims to be very light weight and configurable. The easiest way to use it would be to make it run on startup.
The only external dependency is `xrandr`.

## Install
You can get an x86_64 Linux (ELF) binary from the [releases page](https://github.com/NickGeek/oled-shmoled/releases).

Alternatively, the program is [available on the AUR](https://aur.archlinux.org/packages/oled_shmoled/)
and on [crates.io](https://crates.io/crates/oled_shmoled) via `cargo install oled_shmoled`.

## Usage
```
$ oled_shmoled --help
OLED shmoled 0.1.0
Nick Webster <nick@nick.geek.nz>

USAGE:
    oled_shmoled [OPTIONS] <monitor-ident>

ARGS:
    <monitor-ident>    The xrandr identifier for the display

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --base-path <base-path>
            The base path for the backlight files [default: /sys/class/backlight/intel_backlight]

        --current-brightness-file <current-brightness-file>
            The name of the file containing the current backlight state [default: brightness]

        --max-brightness-file <max-brightness-file>
            The name of the file identifying the maximum brightness of the system [default:
            max_brightness]
```

```
$ xrandr
Screen 0: minimum 8 x 8, current 4608 x 4752, maximum 32767 x 32767
eDP1 connected primary 3840x2160+266+2592 (normal left inverted right x axis y axis) 340mm x 190mm
# ...etc...
```

```
$ oled_shmoled eDP1
Watching /sys/class/backlight/intel_backlight for backlight changes…
```

The easiest way to use OLED Shmoled is to run that last command at start-up. This can be done by running it in an
`.xinitrc` or a systemd job.

## Build instructions
```
$ git clone https://github.com/NickGeek/oled-shmoled oled_shmoled
$ cd oled_shmoled
$ cargo build --release
```
