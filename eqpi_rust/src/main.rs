mod stepper_motor;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use std::io::{Error, Read, Write};
use std::time::Duration;
use chrono::Utc;
use std::collections::HashMap;
use json;
use stepper_motor::StepperMotor;

const RA_DIR: u8 = 13;
const RA_STEP: u8 = 19;
const RA_EN: u8 = 12;
const RA_M1: u8 = 16;
const RA_M2: u8 = 17;
const RA_M3: u8 = 20;
const DEC_DIR: u8 = 24;
const DEC_STEP: u8 = 18;
const DEC_EN: u8 = 4;
const DEC_M1: u8 = 21;
const DEC_M2: u8 = 22;
const DEC_M3: u8 = 27;

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
                let parsed = json::parse(str_data).unwrap();
                println!("Received data: {:?}", str_data);

                if let Some(value) = parsed["frequency"].as_u32() {
                    lo_ra.set_frequency(value);
                }
            },
            Ok(_) => thread = false,
            Err(e) => (),
        }

        // send current status
        if Utc::now().timestamp() - last_time > 2 {
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
    let mut ra_pinout = HashMap::new();
    ra_pinout.insert(String::from("pwm"), RA_STEP);
    ra_pinout.insert(String::from("direction"), RA_DIR);
    ra_pinout.insert(String::from("enable"), RA_EN);

    let mut dec_pinout = HashMap::new();
    dec_pinout.insert(String::from("pwm"), DEC_STEP);
    dec_pinout.insert(String::from("direction"), DEC_DIR);
    dec_pinout.insert(String::from("enable"), DEC_EN);

    let RA = Arc::new(Mutex::new(StepperMotor::new(ra_pinout)));
    let DEC = Arc::new(Mutex::new(StepperMotor::new(dec_pinout)));

    let listener = TcpListener::bind("0.0.0.0:2020").expect("Could not bind"); // 0.0.0.0 allow remote access
    println!("EqPi server aguardando na porta 2020!");

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