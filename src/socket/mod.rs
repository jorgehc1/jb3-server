extern crate ini;

use std::thread;
use std::io;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, Error};
use std::path::Path;
use std::time::Duration;
use ini::Ini;

//interface
pub trait ServerInterface {
    fn read_config_file(name: String) -> Result<ServerConfig, Error>;
    fn handle_client(stream: TcpStream) -> Result<(), Error>;
    fn connect() -> Result<(), Error>;
}

//constructor
pub struct Server();

#[derive(Debug)]
pub struct ServerConfig{
    host: String,
    port: u32
}

impl ServerInterface for Server {
    fn read_config_file(name: String) -> Result<ServerConfig, Error> {
        let c = Path::new("config.ini");
        let is_file = c.is_file();
        let mut sc: ServerConfig = ServerConfig{ host:"".to_string(), port:0};
        if is_file == true {
            let config = Ini::load_from_file(name).unwrap();
            let section = config.section(Some("network")).unwrap();
            let host = section.get("host").unwrap();
            let port = section.get("port").unwrap();
            sc = ServerConfig{host: String::from(host), port: port.parse::<u32>().unwrap()};
        }
        Ok(sc)
    }
    fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
        println!("Connection from {}", stream.peer_addr()?);
        let mut buffer = [0; 1024];
        loop {
            let nbytes = stream.read(&mut buffer)?;
            if nbytes == 0 {
                return Ok(());
            }
            stream.write(&buffer[0..nbytes])?;
            stream.flush()?;
        }
    }
    fn connect() -> Result<(), Error> {
        let config_name: String = String::from("config.ini");
        let param = Self::read_config_file(config_name);
        match param {
            Ok(serverconfig) => {
                if serverconfig.host != "" && serverconfig.port > 0 {
                    let host:String = serverconfig.host;
                    let port:u32 = serverconfig.port;
                    let address = format!("{}:{}", host, port);
                    let listener = TcpListener::bind(address)?;
                    listener.take_error().expect("No error was expected");
                    //listener.set_ttl(0).expect("Could not set TTL");
                    listener.set_nonblocking(false).expect("Cannot set non-blocking");
                    println!("Server listening on {}:{}", host, port);
                    for stream in listener.incoming() {
                        match stream {
                            Ok(stream) => {
                                //println!("New connection for client {}", stream.peer_addr().unwrap());
                                stream.take_error().expect("No error was expected...");
                                stream.set_write_timeout(Some(Duration::new(5, 0))).expect("set_write_timeout call failed");
                                stream.set_read_timeout(Some(Duration::new(5, 0))).expect("set_read_timeout call failed");
                                stream.set_nodelay(true).expect("set_nodelay call failed");
                                //stream.set_ttl(0).expect("set_ttl call failed");
                                stream.set_nonblocking(false).expect("set_nonblocking call failed");
                                thread::spawn(move|| {
                                    let handle = Self::handle_client(stream);
                                    match handle {
                                        Ok(v) => println!("OK handle {:?}", v),
                                        Err(e) => {
                                            panic!("Error handle: {:?}", e)
                                        },
                                    };
                                });
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                // wait until network socket is ready, typically implemented
                                // via platform-specific APIs such as epoll or IOCP
                                //wait_for_fd();
                                continue;
                            }
                            Err(e) => {
                                panic!("encountered IO error: {}", e)
                            }
                        }
                    }
                    drop(listener);
                }else{
                    panic!("");
                }
                //Ok()
            },
            Err(e) => { 
                println!("Error: {}", e) 
            }
        }
        Ok(())
    }
}