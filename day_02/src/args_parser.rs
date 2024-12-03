// External imports
use clap::Parser;

/// Command line arguments for the processor of the Red Nosed Reactor System. This struct is used
/// to parse the command line arguments using the `clap` crate.
#[derive(Parser, Debug)]
#[command(author, version, about = "Process Report levels from a file")]
pub struct Args {
    /// Input file containing reports where each number is a level.
    #[arg(short, long)]
    pub file: String,
}
