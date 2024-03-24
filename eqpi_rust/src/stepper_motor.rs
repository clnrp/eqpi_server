use std::collections::HashMap;
use json::{self, JsonValue};

pub struct StepperMotor {
    pinout: HashMap<String, i32>,
    dutycycle: i32,
    frequency: i32,
    direction: i32,
    working: bool,
}

impl StepperMotor {

    pub fn new() -> Self {
        let mut pinout = HashMap::new();
        pinout.insert(String::from("pwm"), 0);
        pinout.insert(String::from("direction"), 0);
        pinout.insert(String::from("enable"), 0);
        StepperMotor { pinout: pinout, dutycycle: 50, frequency: 60, direction: 0, working: false }
    }

    pub fn start(&mut self) {
    
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

    pub fn get_frequency(self) -> i32{
        self.frequency
    }

    // get current status
    pub fn get_status(&self) -> JsonValue {
        let mut data = json::JsonValue::new_object();
        data["working"] = self.working.into();
        data["frequency"] = self.frequency.into();
        data["direction"] = self.direction.into();
        return data;
    }
}
