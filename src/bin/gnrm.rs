use clap::Parser;
use gacm_rs::actions::gnrm::{self, Action};

#[derive(Parser, Debug, Clone)]
#[clap(name = "gnrm", version, author)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

fn main() {
    let args = Args::parse();
    match args.action {
        Action::Use(args) => gnrm::use_registry(args),
        Action::Ls(_) => gnrm::ls_registry(),
    }
}
