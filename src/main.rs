use color_eyre::eyre::Result;
use clap::Parser;
use tracing_subscriber;

mod cli;
mod mouse_mover;

use crate::cli::Cli;
use crate::mouse_mover::MouseMover;

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    print_banner(cli.delay);

    let mut mouse_mover = MouseMover::new();
    mouse_mover.run(cli.delay)
}

fn print_banner(delay: u64) {
    println!(r#"
______ _____ _____ ______ _   _ 
|  ___|  _  |  __ \| ___ \ \ | |
| |_  | | | | |  \/| |_/ /  \| |
|  _| | | | | | __ |    /| . ` |
| |   \ \_/ / |_\ \| |\ \| |\  |
\_|    \___/ \____/\_| \_\_| \_/
:: Forever Green 0.1 RELEASE ::  
Delay: {} seconds
"#, delay);
}
