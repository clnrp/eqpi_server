mod stepper_motor;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use std::io::{Error, Read, Write};
use std::time::Duration;
use chrono::Utc;
use json;
use stepper_motor::StepperMotor;

fn handle_client(mut stream: TcpStream, mut ra: Arc<Mutex<StepperMotor>>, mut dec: Arc<Mutex<StepperMotor>>) -> Result<(), Error> {
    let mut buffer = [0; 512];
    let mut thread = true;
    let mut last_time = Utc::now().timestamp();
    
    stream.set_read_timeout(Some(Duration::from_secs(1)))?;
    while thread == true {
        let mut lo_ra = ra.lock().unwrap();
        
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                let received_data = &buffer[..n];
                let str_data = std::str::from_utf8(&received_data).unwrap();
                println!("Received data: {:?}", str_data);
                lo_ra.set_frequency(90);
            },
            Ok(_) => thread = false,
            Err(e) => (),
        }

        // send current status
        if Utc::now().timestamp() - last_time > 5 {
            last_time = Utc::now().timestamp();
            let status = lo_ra.get_status();
            stream.write(status.dump().as_bytes());
        }
        std::mem::drop(lo_ra);
    }

    println!("Closed connection!");

    Ok(())
}

fn main() {
    let RA = Arc::new(Mutex::new(StepperMotor::new()));
    let DEC = Arc::new(Mutex::new(StepperMotor::new()));

    let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind"); // 0.0.0.0 allow remote access
    println!("EqPi server aguardando na porta 8888!");

    // waiting for new connection
    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("failed: {}", e),
            Ok(stream) => {
            	println!("Nova conexão: {}", stream.peer_addr().unwrap());
                let ra = Arc::clone(&RA);
                let dec = Arc::clone(&DEC);
                thread::spawn(move || {
                    handle_client(stream, ra, dec).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}