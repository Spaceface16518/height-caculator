#![deny(clippy::pedantic)]
use std::{
    thread,
    time::{Duration, Instant},
};

use dialoguer::Input;
use indicatif::{ProgressBar, ProgressStyle};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    println!(
        "--- Height Caculator tool --- v{} --- ",
        env!("CARGO_PKG_VERSION")
    );

    let height: u32 = Input::new()
        .with_prompt("Input your height")
        .interact_text()?;

    let pb = ProgressBar::new(2000).with_prefix("Caculating...");
    pb.set_style(ProgressStyle::default_bar().template("{prefix} {wide_bar}"));
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(2) {
        pb.inc(1);
        thread::sleep(Duration::from_millis(1));
    }
    pb.finish_and_clear();

    println!("Your height is {height}cm!");

    Ok(())
}
