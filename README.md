# OLED Shmoled
OLED displays don't have backlights but it'd be nice if we could pretend they do on Linux!

This program aims to be very light weight and configurable. The easiest way to use it would be to make it run on startup.
The only external dependency is `xrandr`.

## Build instructions
```
$ git clone https://github.com/NickGeek/oled-shmoled oled_shmoled
$ cd oled_shmoled
$ cargo build --release
```
