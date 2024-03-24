use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Error, Read, Write};
use std::time::Duration;
use chrono::Utc;
use json::JsonValue;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    let mut buffer = [0; 512];
    let mut thread = true;
    let mut last_time = Utc::now().timestamp();
    
    stream.set_read_timeout(Some(Duration::from_secs(1)))?;
    while thread == true {
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                let received_data = &buffer[..n];
                let str_data = std::str::from_utf8(&received_data).unwrap();
                println!("Received data: {:?}", str_data);
            },
            Ok(_) => thread = false,
            Err(e) => (),
        }

        if Utc::now().timestamp() - last_time > 5 {
            last_time = Utc::now().timestamp();
            let mut data = json::JsonValue::new_object();
            data["working"] = false.into();
            data["frequency"] = 60.into();
            data["direction"] = 0.into();
            stream.write(data.dump().as_bytes());
        }
    }

    println!("Closed connection!");

    Ok(())
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");
    println!("EqPi server aguardando na porta 8888!");
    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("failed: {}", e),
            Ok(stream) => {
            	println!("Nova conexão: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}