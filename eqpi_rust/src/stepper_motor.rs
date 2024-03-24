use std::collections::HashMap;
use json::{self, JsonValue};
use std::thread;
use std::time::Duration;
use rppal::gpio::{Gpio, Level, Mode};

pub struct StepperMotor {
    pinout: HashMap<String, i32>,
    dutycycle: i32,
    frequency: i32,
    direction: i32,
    working: bool,
}

impl StepperMotor {

    pub fn new(pinout: HashMap<String, i32>) -> Self {
        StepperMotor { pinout: pinout, dutycycle: 50, frequency: 60, direction: 0, working: false }
    }

    pub fn start(&mut self) {
        let mut pin = Gpio::new().unwrap().get(17).unwrap().into_output();
        pin.set_high();
    }

    pub fn stop(&mut self) {

    }

    // direction of rotation of the engine
    pub fn set_direction(&mut self, direction: i32) {
        self.direction = direction;
    }

    pub fn set_frequency(&mut self, frequency: i32) {
        self.frequency = frequency;
    }

    pub fn get_frequency(&self) -> i32{
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
