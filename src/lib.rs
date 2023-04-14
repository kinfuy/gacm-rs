pub mod command {
    use structopt::StructOpt;

    #[derive(StructOpt, Debug)]
    #[structopt(name = "gacm")]
    pub struct Cli {
        pub pattern: String,
        #[structopt(parse(from_os_str))]
        pub command: std::path::PathBuf,
    }
}
