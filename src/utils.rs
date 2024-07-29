use std::ffi::OsStr;
use std::path::Path;

use crate::handler::Status;
use colored::Colorize;

pub fn get_file_extension(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

pub fn c_r() -> Box<dyn Fn(bool) -> String> {
    Box::new(|_| "\r\n".to_owned())
}

pub fn content_length(length: usize) -> Box<dyn Fn(bool) -> String> {
    Box::new(move |verbose| {
        if verbose {
            let key = "Content-Length: ".bold().white();
            println!("{}{}", key, length);
        }
        format!("Content-Length: {}", length)
    })
}

pub fn content_type(ext: String) -> Box<dyn Fn(bool) -> String> {
    Box::new(move |verbose| {
        let mimetype = match ext.as_str() {
            "html" => "text/html",
            "css" => "text/css",
            "js" => "text/javascript",
            "json" => "application/json",
            _ => "text/plain",
        };
        if verbose {
            print!("{}", "Content-Type: ".bold().white());
            println!("{}; charset=utf-8", mimetype);
        }

        format!("Content-Type: {}; charset=utf-8", mimetype)
    })
}

pub fn status_line(status_code: u16) -> Box<dyn Fn(bool) -> String> {
    Box::new(move |verbose| {
        let status: Status = status_code.try_into().unwrap();
        let heading = format!("HTTP/1.1 {status_code} {}", status.to_string());
        if verbose {
            if status_code == 200 {
                println!("{}", heading.bright_green())
            } else {
                println!("{}", heading.bright_red())
            }
        }

        heading
    })
}

pub fn string_line(str: String) -> Box<dyn Fn(bool) -> String> {
    Box::new(move |verbose| {
        if verbose {
            print!("{}", str);
        }
        str.to_string()
    })
}

pub fn response_builder(args: Vec<Box<dyn Fn(bool) -> String>>, verbose: bool) -> String {
    if verbose {
        let heading = "Response:".bold().green();
        println!("{}", heading);
    }
    let plain = args
        .iter()
        .map(|func| {
            let s = func(verbose);
            return s;
        })
        .collect::<Vec<String>>()
        .join("");
    println!();

    return plain;
}
