use crate::{server::Config, utils::*};
use std::{
    env, fs,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    path::Path,
};

use anyhow::{anyhow, Result as ResultAnyhow};
use colored::Colorize;

pub enum Status {
    Ok,
    NotFound,
}

impl TryFrom<u16> for Status {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            200 => Ok(Status::Ok),
            404 => Ok(Status::NotFound),
            _ => Err(()),
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Ok => "OK".to_string(),
            Status::NotFound => "Not Found".to_string(),
        }
    }
}

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string(),
    }
}

fn print_request(req: &Vec<String>) {
    let heading = "Request:".bold().green();
    println!("{}", heading);

    req.iter().for_each(|x| {
        let splitted = x.split_once(":");
        if splitted.is_none() {
            print!("{}\n", x.bold().bright_green());
        } else {
            let (key, value) = splitted.unwrap();
            print!("{}: {}\n", key.bold().white(), value);
        }
    });
    print!("\n");
}

// fn print_response(res: String) {
//     let heading = "Response:".bold().green();
//     println!("{}", heading);
//     let splitted = res.split_once("\r\n\r\n");
//     if splitted.is_none() {
//         println!("{:?}", res.bold().bright_green());
//     } else {
//         println!("{:?}", splitted);
//     }
//
//     res.lines()
// }

pub fn handle_connection(mut stream: TcpStream, conf: &Config) -> ResultAnyhow<()> {
    let buf_reader = BufReader::new(&mut stream);
    let http_req: Vec<_> = buf_reader
        .lines()
        .flat_map(|res| res)
        .take_while(|line| !line.is_empty())
        .collect();

    if conf.verbose {
        print_request(&http_req);
    }
    let res = get_file_buf(http_req, &conf)?;

    if conf.verbose {}
    stream.write_all(res.as_bytes())?;

    Ok(())
}

fn get_file_buf(http_req: Vec<String>, conf: &Config) -> ResultAnyhow<String> {
    let headers = http_req.iter().nth(0).unwrap();
    let mut path = &headers.split(" ").nth(1).unwrap()[1..];
    if path == "" {
        path = "index.html";
    }

    let filename = Path::new(&get_current_working_dir())
        .join(&conf.static_path)
        .join(path)
        .into_os_string()
        .into_string();

    if let Err(_) = filename {
        return Err(
            anyhow!("Failed to convert OsString to String").context("While constructing file path")
        );
    }

    let filename = filename.unwrap();
    let ext = get_file_extension(&filename).or(Some("")).unwrap();
    let contents = fs::read_to_string(filename.clone());

    if let Err(err) = contents {
        eprintln!("{}", err);
        return Ok(response_builder(
            vec![status_line(404), c_r()],
            conf.verbose,
        ));
    }

    let contents = contents.unwrap();

    Ok(response_builder(
        vec![
            status_line(200),
            c_r(),
            content_type(ext.to_string()),
            c_r(),
            content_length(contents.len()),
            c_r(),
            c_r(),
            string_line(contents),
        ],
        conf.verbose,
    ))
}
