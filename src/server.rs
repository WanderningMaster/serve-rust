use crate::handler::handle_connection;
use std::{fs, net::TcpListener, path::PathBuf};

use anyhow::{anyhow, Result as ResultAnyhow};
use colored::Colorize;

pub struct Config {
    pub verbose: bool,
    pub port: u32,
    pub static_path: String,
}

pub fn start_server(conf: Config) -> ResultAnyhow<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", conf.port));
    if listener.is_err() {
        panic!("Failed to start server: {}", listener.unwrap_err());
    }
    let resource_path = PathBuf::from(conf.static_path.clone());
    let resource_path = fs::canonicalize(&resource_path);
    if resource_path.is_err() {
        return Err(anyhow!("Failed to resolve resource path"));
    }

    let listener = listener.unwrap();

    println!(
        "Static is being served on port {}",
        conf.port.to_string().bright_blue()
    );
    println!(
        "{}",
        format!("Resource path: {:?}", resource_path.unwrap()).bright_blue()
    );

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream, &conf)?;
            }
            Err(err) => {
                eprintln!("{}", err.to_string().red());
                continue;
            }
        }
    }

    Ok(())
}
