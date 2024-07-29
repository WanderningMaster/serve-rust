use clap::Parser;
use serve_rust::server::{start_server, Config};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(required = false, default_value_t = 8080)]
    port: u32,

    #[arg(required = false, default_value_t = String::from("./"))]
    static_path: String,

    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    match start_server(Config {
        port: args.port,
        verbose: args.verbose,
        static_path: args.static_path,
    }) {
        Err(err) => eprintln!("{}", err),
        _ => (),
    }
}
