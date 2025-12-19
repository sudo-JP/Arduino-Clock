#![no_std]
#![no_main]

use arduino_hal::hal::spi;
use panic_halt as _;
use arduino_clock::{display::{self}, Time, TimeChange};
use core::option::Option;
use core::option::Option::{None};

// False -> failed
fn check_byte(b1: u8, b2: u8) -> bool {
    b1 >= b'0' && b1 <= b'9' && 
    b2 >= b'0' && b2 <= b'9'
}

fn calculate_byte(b1: u8, b2: u8) -> u8 {
    let tens = b1 - b'0';
    let ones = b2 - b'0';
    tens * 10 + ones
}

fn parse_arg(buffer: [u8; 8]) -> Option<Time> {
    if buffer[2] != b':' || buffer[5] != b':' {
        return None;
    }

    if !check_byte(buffer[0], buffer[1]) {
        return None; 
    }
    let hour = calculate_byte(buffer[0], buffer[1]); 

    if !check_byte(buffer[3], buffer[4]) {
        return None; 
    }
    let min = calculate_byte(buffer[3], buffer[4]);

    if !check_byte(buffer[6], buffer[7]) {
        return None; 
    }
    let sec = calculate_byte(buffer[6], buffer[7]);
    Time::new(hour, min, sec)
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    //let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Display setup 
    let (spi, _cs) = spi::Spi::new(
        dp.SPI, 
        pins.d13.into_output(), 
        pins.d11.into_output(), 
        pins.d12.into_pull_up_input(), 
        pins.d10.into_output(), 
        spi::Settings::default(),
    );

    //ufmt::uwriteln!(&mut serial, "Arduino Started!").ok();

    let dc = pins.d9.into_output();
    let rst = pins.d8.into_output();

    let mut delay = arduino_hal::Delay::new();
    let mut displ = display::TFTScreen::new(spi, dc, rst, &mut delay)
        .expect("Display Init failed");

    // Prompt for time 
    //ufmt::uwriteln!(&mut serial, "Enter time (HH:MM:SS): ").ok();

    let mut buffer: [u8; 8] = [0; 8];
    let mut i = 0; 

    loop {
        let byte = serial.read_byte(); 
        if byte == b'\n' {
            break; 
        }

        if i >= 8 {
            panic!("Could not parse input");
        }
        buffer[i] = byte; 
        i += 1; 
    }

    // For length less than 8 
    if i != 8 {
        panic!("Invalid format - must be HH:MM:SS");
    }

    let mut timer = parse_arg(buffer)
        .expect("Invalid time format");

    let mut timer = Time::new(10, 20, 00)
        .expect("Bad");
    displ.draw_time(&timer).unwrap();
    loop {
        arduino_hal::delay_ms(1000);
        match timer.tick() {
            TimeChange::Minute | TimeChange::Hour => displ.draw_time(&timer).unwrap(), 
            _ => {}
        }
    }
}
