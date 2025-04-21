use esp_idf_svc::hal::gpio::{Output, OutputPin, PinDriver};
use esp_idf_svc::hal::peripherals::Peripherals;
use log::error;

pub struct Led<'a, P: OutputPin> {
    pin: PinDriver<'a, P, Output>,
}

impl<'a, P: OutputPin> Led<'a, P> {
    pub fn new(pin: P) -> Self {
        let Ok(led) = PinDriver::output(pin) else {
            let error = "Failed to get led pin";
            log::error!("{}", error);
            panic!("{}", error);
        };
        Led { pin: led }
    }

    pub fn turn_on(&mut self) { 
        if let Err(error) = self.pin.set_high() {
            log::error!("Failed to turn on led: {}", error);
            panic!("{}", error);
        };
        log::info!("Led is on!");
    }

    pub fn turn_off(&mut self) {
        if let Err(error) = self.pin.set_low() {
            log::error!("Failed to turn on led: {}", error);
            panic!("{}", error);   
        }
        log::info!("Led is off!");
    }
}

