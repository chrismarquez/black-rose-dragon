use esp_idf_svc::hal::peripherals::Peripherals;

pub struct Board {

}

impl Board {
    pub fn new() -> Self {
        // It is necessary to call this function once. Otherwise some patches to the runtime
        // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
        esp_idf_svc::sys::link_patches();

        // Bind the log crate to the ESP Logging facilities
        esp_idf_svc::log::EspLogger::initialize_default();
        Self {}
    }

    pub fn get_peripherals(&self) -> Peripherals {
        let Ok(peripherals) = Peripherals::take() else {
            let error = "Failed to take peripherals";
            log::error!("{}", error);
            panic!("{}", error);
        };
        peripherals
    }

}