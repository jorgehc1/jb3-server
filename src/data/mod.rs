extern crate ini;

use std::path::Path;
use std::ffi::OsStr;
use std::io::{Error, Read, ErrorKind};
use ini::Ini;
use regex::Regex;

//interface
pub trait DataInterface {
    fn is_valid_host(host: String) -> bool;
    fn is_valid_port(port: String) -> bool;
    fn is_valid_data_dir(data_dir: String) -> bool;
    fn read_config_file(name: String) -> Result<ServerConfig, Error>;
    fn read_data_core()-> bool;
}

//constructor
pub struct Data();

#[derive(Debug)]
pub struct ServerConfig{
    host: String,
    port: u32,
    ttl: u32,
    buffer_size: u32,
    timeout_read: u32,
    timeout_write: u32,
    encoding: String,
    data_dir: String
}

impl DataInterface for Data {

    fn is_valid_host(host: String) -> bool {
        let r1 = Regex::new(r"^((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
        let r2 = Regex::new(r"/^[a-zA-Z0-9][a-zA-Z0-9-]{1,61}[a-zA-Z0-9]\.[a-zA-Z]{2,}$/").unwrap();
        r1.is_match(&host) || r2.is_match(&host)
    }

    fn is_valid_port(port: String) -> bool {
        let r1 = Regex::new(r"^((6553[0-5])|(655[0-2][0-9])|(65[0-4][0-9]{2})|(6[0-4][0-9]{3})|([1-5][0-9]{4})|([0-5]{0,5})|([0-9]{1,4}))$").unwrap();
        r1.is_match(&port)
    }

    fn is_valid_data_dir(data_dir: String) -> bool {
        Path::new(&data_dir).is_dir()
    }

    fn read_config_file(name: String) -> Result<ServerConfig, Error> {
        let c = Path::new(&name);
        let is_file = c.is_file();
        let sc;
        if is_file == true {
            let config = Ini::load_from_file(name).unwrap();

            let section1 = config.section(Some("network")).unwrap();
            let host = section1.get("host").unwrap();
            let port = section1.get("port").unwrap();
            let ttl = section1.get("ttl").unwrap();
            let buffer_size = section1.get("buffer_size").unwrap();
            let timeout_read = section1.get("timeout_read").unwrap();
            let timeout_write = section1.get("timeout_write").unwrap();

            let section2 = config.section(Some("engine")).unwrap();
            let encoding = section2.get("encoding").unwrap();
            let data_dir = section2.get("data_dir").unwrap();

            if Self::is_valid_host(host.to_string()) == false {
                return Err(Error::new(ErrorKind::Other, format!("host param: \'{}\' is invalid.", host.to_string())));
            }

            if Self::is_valid_port(port.to_string()) == false {
                return Err(Error::new(ErrorKind::Other, format!("port param: \'{}\' is invalid.", port.to_string())));
            }

            if Self::is_valid_data_dir(data_dir.to_string()) == false {
                return Err(Error::new(ErrorKind::Other, format!("data_dir param: \'{}\' is invalid or directory doesn't exists", data_dir.to_string())));
            }

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
        }else{
            return Err(Error::new(ErrorKind::Other, "Can't opening file config.ini..."));
        }
        Ok(sc)
    }

    fn read_data_core() -> bool{
        let config = Self::read_config_file("config.ini".to_string());
        match config {
            Ok(sc) => {
                let encoding = sc.encoding;
                let data_dir = sc.data_dir;

                //println!("OK server {:?}", sc)
            }
            Err(e) => {
                panic!("Error server: {:?}", e)
            }
        };
        
        
        let path = Path::new("./data/bar.txt");



        let parent = path.parent();
        assert_eq!(parent, Some(Path::new("./foo")));

        let file_stem = path.file_stem();
        assert_eq!(file_stem, Some(OsStr::new("bar")));

        let extension = path.extension();
        assert_eq!(extension, Some(OsStr::new("txt")));

        true
    }

}