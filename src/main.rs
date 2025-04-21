mod led;
mod board;

use log;
use led::Led;
use board::Board;
use esp_idf_svc::hal::delay::{FreeRtos};

fn main() {
    log::info!("Hello, world!");
    let board = Board::new();
    let peripherals = board.get_peripherals();
    let mut led = Led::new(peripherals.pins.gpio12);
    loop {
        led.turn_on();
        FreeRtos::delay_ms(1000);
        led.turn_off();
        FreeRtos::delay_ms(1000);
    }
}
