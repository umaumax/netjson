use netjson::Command;

use anyhow::Result;
use log::{error, info};
use serde::de::Deserialize;

use std::env;
use std::io::BufReader;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) -> Result<()> {
    info!("client start {:?}", stream);

    // NOTE: accept json stream only
    let mut buf = vec![];
    'outer: loop {
        let mut read = [0; 16];
        info!("[read] start");
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                buf.append(&mut read[0..n].to_vec());

                loop {
                    let mut reader = BufReader::new(Cursor::new(&buf));

                    let mut de = serde_json::Deserializer::from_reader(&mut reader);
                    let u = Command::deserialize(&mut de);
                    match u {
                        Ok(v) => {
                            let current_pos = reader.seek(SeekFrom::Current(0))? as usize;

                            info!("[recv] {:?}", v);
                            if v.name.as_str() == "disconnect" {
                                break 'outer;
                            }

                            // echo content
                            stream.write_all(&buf[0..current_pos])?;
                            stream.write_all("\n".as_bytes())?;
                            stream.flush()?;

                            // set remaining data to buffer
                            buf.drain(0..current_pos);
                            continue;
                        }
                        Err(_) => {
                            // NOTE: receiving json data in progress
                            info!("[receiving...] {:?}", String::from_utf8_lossy(&buf));
                            break;
                        }
                    }
                }
            }
            Err(err) => {
                error!("[ERROR] server read failed {}", err);
            }
        }
    }
    info!("client end {:?}", stream);
    std::thread::sleep(std::time::Duration::from_millis(1000));
    Ok(())
}

fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:4200").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || -> Result<()> {
                    handle_client(stream)?;
                    Ok(())
                })
                .join()
                .unwrap()?;
            }
            Err(err) => {
                error!("[ERROR] server connection failed {}", err);
            }
        }
    }

    Ok(())
}
