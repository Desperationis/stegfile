use clap::{Parser, Subcommand, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SplitModeEnum {
    Scrambled,
    Full,
}

#[derive(Subcommand)]
pub enum Commands {
    Extract {
        image_dir: String,
        passphrase: String,
        output_file: String,
    },
    Embed {
        image_dir: String,
        passphrase: String,
        input_file: String,
    },
    Capacity {
        image_dir: String,
    },
}

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(
        long,
        short = 'm',
        default_value = "full",
        long_help = "How data is / was split between images"
    )]
    pub split_mode: SplitModeEnum,
}
