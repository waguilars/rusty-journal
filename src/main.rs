use structopt::StructOpt;

use crate::cli::CommandLineArgs;

mod cli;

fn main() {
    let args = cli::CommandLineArgs::from_args();
    println!("{:#?}", &args);
}
