use gacm::command::Cli;
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();

    println!("{:?}", args)
}
