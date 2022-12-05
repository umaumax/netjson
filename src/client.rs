use netjson::Command;

use anyhow::Result;
use log::{error, info};

use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;

fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    app()?;
    Ok(())
}

fn app() -> Result<()> {
    let mut sock = TcpStream::connect("127.0.0.1:4200").expect("failed to connect server");

    for i in 0..3 {
        let command = Command {
            name: "hoge".to_string(),
            content: format!("CONTENT:{}", i).to_string(),
        };
        let body = serde_json::to_string(&command)?;
        info!("[send] {:?}", body);
        sock.write_all(&body.as_bytes())?;
    }
    let command = Command {
        name: "disconnect".to_string(),
        content: "".to_string(),
    };
    let body = serde_json::to_string(&command)?;
    info!("[send] {:?}", body);
    sock.write_all(&body.as_bytes())?;
    sock.flush()?;

    let mut reader = BufReader::new(sock);
    loop {
        let mut buf = String::new();
        let result = reader.read_line(&mut buf);
        match result {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                println!("[{}] {:?}", n, buf);
            }
            Err(e) => {
                println!("error reading: {}", e);
                break;
            }
        }
    }

    Ok(())
}
