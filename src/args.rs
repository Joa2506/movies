use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version)]
pub struct RustMoviesArgs {
    ///First argument
    pub first_arg: String,
}
