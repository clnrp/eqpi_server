use std::collections::HashMap;
use json::{self, JsonValue};
use std::thread;
use std::time::Duration;
use rppal::gpio::{Gpio, Level, Mode};

pub struct StepperMotor {
    pinout: HashMap<String, u8>,
    dutycycle: u8,
    frequency: u32,
    direction: u8,
    working: bool,
}

impl StepperMotor {

    pub fn new(pinout: HashMap<String, u8>) -> Self {
        StepperMotor { pinout: pinout, dutycycle: 50, frequency: 60, direction: 0, working: false }
    }

    pub fn start(&mut self) {
        if let Some(pin) = self.pinout.get("RA_EN") {
            let mut pin_en = Gpio::new().unwrap().get(*pin).unwrap().into_output();
            pin_en.set_high();
        }
    }

    pub fn stop(&mut self) {
        if let Some(pin) = self.pinout.get("RA_EN") {
            let mut pin_en = Gpio::new().unwrap().get(*pin).unwrap().into_output();
            pin_en.set_low();
        }
    }

    // direction of rotation of the engine
    pub fn set_direction(&mut self, direction: u8) {
        self.direction = direction;
        if let Some(pin) = self.pinout.get("RA_DIR") {
            let mut pin_dir = Gpio::new().unwrap().get(*pin).unwrap().into_output();
            if direction == 1 {
                pin_dir.set_high();
            } else {
                pin_dir.set_low();
            }
        }
    }

    pub fn set_frequency(&mut self, frequency: u32) {
        self.frequency = frequency;
    }

    pub fn get_frequency(&self) -> u32{
        self.frequency
    }

    // get current status
    pub fn get_status(&self) -> JsonValue {
        let mut data: JsonValue = json::JsonValue::new_object();
        data["working"] = self.working.into();
        data["frequency"] = self.frequency.into();
        data["direction"] = self.direction.into();
        return data;
    }
}
