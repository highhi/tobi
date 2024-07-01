pub use anyhow::Result;
use clap::Parser;

pub trait Runnable {
    fn run(&self) -> Result<()>;
}

pub use clap::Parser as Cli;
pub use clap::Subcommand;

#[macro_export]
macro_rules! run {
    ($cli:expr) => {
        fn main() -> $crate::Result<()> {
            let cli = $cli::parse();
            cli.run()
        }
    };
}
