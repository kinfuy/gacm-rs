use clap::Parser;
use gacm_rs::actions::gacm::{self, Action};

#[derive(Parser, Debug, Clone)]
#[clap(name = "gacm", version, author)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

fn main() {
    let args = Args::parse();
    match args.action {
        Action::Use(args) => gacm::use_account(args),
        Action::Ls(_) => gacm::ls_account(),
    }
}
