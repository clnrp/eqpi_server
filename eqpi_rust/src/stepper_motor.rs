use std::collections::HashMap;
use json::{self, JsonValue};
use std::thread;
use std::time::Duration;
use rppal::gpio::{Gpio, Level, Mode};
use rppal::pwm::{Channel, Polarity, Pwm};

pub struct StepperMotor {
    pinout: HashMap<String, u8>,
    dutycycle: f64,
    frequency: f64,
    direction: u8,
    working: bool,
    pwm: Pwm,
}

impl StepperMotor {

    pub fn new(pinout: HashMap<String, u8>) -> Self {
        let mut channel: Channel = Channel::Pwm0; 
        if let Some(pin) = pinout.get("DEC_STEP") {
            if *pin == 18 {
                channel = Channel::Pwm1;
            }
        }
        StepperMotor { pinout: pinout, dutycycle: 0.5, frequency: 60.0, direction: 0, working: false, pwm: Pwm::new(channel).unwrap() }
    }

    pub fn start(&mut self, frequency: f64, direction: u8) {
        if let Some(pin) = self.pinout.get("RA_EN") {
            let mut pin_en = Gpio::new().unwrap().get(*pin).unwrap().into_output();
            pin_en.set_high();
        }
        self.frequency = frequency;
        self.direction = direction;
        self.set_direction(direction);
        self.pwm.set_frequency(self.frequency, self.dutycycle);
        self.pwm.set_polarity(Polarity::Normal);
        self.pwm.enable();
        self.working = true;
    }

    pub fn stop(&mut self) {
        if let Some(pin) = self.pinout.get("RA_EN") {
            let mut pin_en = Gpio::new().unwrap().get(*pin).unwrap().into_output();
            pin_en.set_low();
        }
        self.pwm.disable();
        self.working = false;
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

    pub fn set_frequency(&mut self, frequency: f64) {
        self.frequency = frequency;
        self.pwm.set_frequency(self.frequency, self.dutycycle);
    }

    pub fn get_frequency(&self) -> f64{
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
