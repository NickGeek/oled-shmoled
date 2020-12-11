use anyhow::Result;
use clap::Clap;
use notify::{raw_watcher, RecursiveMode, Watcher};
use std::fs;
use std::path::{Path};
use std::sync::mpsc::channel;
use std::process::Command;

#[macro_use] extern crate anyhow;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

#[derive(Clap, Debug)]
#[clap(name = "OLED shmoled", version = VERSION, author = AUTHORS)]
struct Opts {
    /// The xrandr identifier for the display
    #[clap()]
    monitor_ident: String,

    /// The base path for the backlight files
    #[clap(long, default_value = "/sys/class/backlight/intel_backlight")]
    base_path: String,

    /// The name of the file containing the current backlight state
    #[clap(long, default_value = "brightness")]
    current_brightness_file: String,

    /// The name of the file identifying the maximum brightness of the system
    #[clap(long, default_value = "max_brightness")]
    max_brightness_file: String,
}

fn update_brightness(opts: &Opts) -> Result<()> {
    let current_brightness = read_file(
        Path::new(&opts.base_path)
            .join(Path::new(&opts.current_brightness_file))
    )?;

    let max_brightness = read_file(
        Path::new(&opts.base_path)
            .join(Path::new(&opts.max_brightness_file))
    )?;

    let new_brightness = current_brightness / max_brightness;

    Command::new("sh")
        .args(&[
            "-c",
            format!("xrandr --output {} --brightness {}", opts.monitor_ident, new_brightness).as_str(),
        ])
        .spawn()
        .map_err(|err| anyhow!("Failed to set xrandr output: {}", err))?;

    Ok(())
}

fn read_file<P: AsRef<Path>>(path: P) -> Result<f64> {
    let contents = fs::read_to_string(path)
        .map_err(|e| anyhow!("Error reading the file: {}", e))?;

    Ok(
        contents
            .trim()
            .parse::<f64>()
            .map_err(|e| anyhow!("Error parsing the file: {}", e))?
    )
}

fn main() {
    let opts: Opts = Opts::parse();

    let current_brightness_path = Path::new(&opts.base_path)
        .join(Path::new(&opts.current_brightness_file));

    let (tx, brightness_changes) = channel();
    let mut watcher = raw_watcher(tx).unwrap();
    watcher
        .watch(&current_brightness_path, RecursiveMode::NonRecursive)
        .expect("Could not create a watcher.");

    println!("Watching {} for backlight changes…", opts.base_path);
    loop {
        match brightness_changes.recv() {
            Ok(_) => {
                if let Err(e) = update_brightness(&opts) {
                    eprintln!("Error updating brightness: {}", e);
                }
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
