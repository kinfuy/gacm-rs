use gacm::cli;

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("ls", sub_matches)) => {
            println!("gacm ls:,{:?}", sub_matches)
        }
        Some(("clone", sub_matches)) => {
            println!(
                "Cloning {}",
                sub_matches.get_one::<String>("REMOTE").expect("required")
            );
        }
        other => {
            println!("{:?}", other)
        }
    }
}
