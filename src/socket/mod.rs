extern crate ini;
extern crate nucleo;

//use std::thread;
//use std::io;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, Error};
use std::path::Path;
//use std::time::Duration;
use ini::Ini;
use nucleo::config::ServerConfig;

//interface
pub trait ServerInterface {
    fn read_config_file(name: String) -> Result<ServerConfig, Error>;
    fn handle_client(stream: TcpStream) -> Result<(), Error>;
    fn connect(sc: ServerConfig) -> Result<(), Error>;
}

//constructor
pub struct Server();

impl ServerInterface for Server {
    fn read_config_file(name: String) -> Result<ServerConfig, Error> {
        let c = Path::new("config.ini");
        let is_file = c.is_file();
        let mut sc: ServerConfig = ServerConfig::default();

        if is_file == true {
            let config = Ini::load_from_file(name).unwrap();
            let section1      = config.section(Some("network")).unwrap();
            let host          = section1.get("host").unwrap();
            let port          = section1.get("port").unwrap();
            let ttl           = section1.get("ttl").unwrap();
            let buffer_size   = section1.get("buffer_size").unwrap();
            let timeout_read  = section1.get("timeout_read").unwrap();
            let timeout_write = section1.get("timeout_write").unwrap();
            let section2      = config.section(Some("engine")).unwrap();
            let encoding      = section2.get("encoding").unwrap();
            let data_dir      = section2.get("data_dir").unwrap();

            sc = ServerConfig{
                host: String::from(host), 
                port: port.parse::<u32>().unwrap(),
                ttl: ttl.parse::<u32>().unwrap(),
                buffer_size: buffer_size.parse::<u32>().unwrap(),
                timeout_read: timeout_read.parse::<u32>().unwrap(),
                timeout_write: timeout_write.parse::<u32>().unwrap(),
                encoding: String::from(encoding),
                data_dir: String::from(data_dir)
            };
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
    fn connect(sc: ServerConfig) -> Result<(), Error> {

        let host:String = sc.host;
        let port:u32    = sc.port;
        let ttl:u32     = sc.ttl;
        let address     = format!("{}:{}", host, port);
        let listener    = TcpListener::bind(address)?;

        listener.take_error().expect("No error was expected");
        listener.set_ttl(ttl).expect("Could not set TTL");
        listener.set_nonblocking(false).expect("Cannot set non-blocking");
        println!("JB3 Database Server listening on {}:{}", host, port);
        for streamer in listener.incoming() {
            match streamer {
                Ok(stream) => {
                    println!("New connection for client {}", stream.peer_addr().unwrap());
                    stream.take_error().expect("No error was expected...");
                }
                Err(_) => {

                }
            }
        }
        
        /*
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
        */

        Ok(())
    }
}