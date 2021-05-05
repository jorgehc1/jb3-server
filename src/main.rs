#[warn(unused_imports)]
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate regex;
extern crate ini;

mod socket;
mod data;

use std::env;
use std::io::{self, Write};

//use regex::Regex;
use console::style;

use data::Data;
use data::DataInterface;
use socket::Server;
use socket::ServerInterface;

/*
mod data;

use std::mem;
use data::Configuration;
*/

fn main() -> io::Result<()> {
    env_logger::init();
    debug!("Starting JB3 Database Server...");

    let arg1: String = env::args().nth(1).unwrap();
    let cmd = String::from(arg1);

    if let "start" = &*cmd {
        let data = Data::read_config_file("config.ini".to_string());
        match data {
            Ok(config) => {
                println!("OK server {:?}", config)
            }
            Err(e) => { 
                panic!("Error: {:?}", e)
            }
        };

        /*
        let server = Server::connect();
        match server {
            Ok(v) => println!("OK server {:?}", v),
            Err(e) => panic!("Error server: {:?}", e),
        };*/

    }else if let "stop" = &*cmd {
        println!("stop");
    }else if let "reload" = &*cmd {
        println!("reload");
    }else if let "restart" = &*cmd {
        println!("Equal");
    }if let "cmd" = &*cmd {
        loop{
            let mut buffer = String::new();
            print!("{}>> ", style("jb3").cyan());
            io::stdout().flush().expect("some error message");
            io::stdin().read_line(&mut buffer)?;

            match buffer.trim_end() {
                "" => print!(""),
                "exit" => break,
                "quit" => break,
                line => { 
                    println!("processing...");
                    if let "hhh" = &*line.to_lowercase() { 
                    }

                    println!("finished...");
                }
            };
        }
    }

    Ok(())
}
