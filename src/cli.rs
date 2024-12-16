use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = 300)]
    pub delay: u64,

    #[arg(short, long)]
    pub verbose: bool,
}
