mod led;
mod board;
mod wifi;

use embedded_svc::http::client::Client as HttpClient;

use std::net::SocketAddr;
use esp_idf_svc::eventloop::{BackgroundLoopConfiguration, EspEventLoop, EspSystemEventLoop};
use esp_idf_svc::http::client;
use log;
use led::Led;
use board::Board;
use esp_idf_svc::hal::delay::{FreeRtos};
use esp_idf_svc::http::client::{Configuration, EspHttpConnection};
use crate::wifi::Wifi;

fn main() {
    log::info!("Hello, world!");
    let board = unsafe { Board::new() };
    let peripherals = board.get_peripherals();
    let mut led = Led::new(peripherals.pins.gpio12);
    
    let Ok(sys_loop) = EspSystemEventLoop::take() else {
        panic!("Failed to take system loop");
    };
    let Ok(mut wifi) = Wifi::new(peripherals.modem, sys_loop) else {
        panic!("Failed to take wifi");
    };
    wifi.connect("Khala-Alta", "Netm@ralv1")
        .expect("Failed to connect to wifi");

    let connection = EspHttpConnection::new(&Configuration {
        ..Default::default()
    }).expect("Failed to create http connection");
    
    let mut client = HttpClient::wrap(connection);
    
    let response = client
        .get("http://www.google.com")
        .expect("Failed to get http")
        .submit()
        .expect("Failed to submit http");
    
    log::info!("Response: {:?}", response.status());
    
    
    log::info!("Connected to wifi");
    loop {
        led.turn_on();
        FreeRtos::delay_ms(1000);
        led.turn_off();
        FreeRtos::delay_ms(1000);
    }
}
