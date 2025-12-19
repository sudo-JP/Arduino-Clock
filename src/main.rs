#![no_std]
#![no_main]

use arduino_hal::hal::spi;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    let (spi, _cs) = spi::Spi::new(
        dp.SPI, 
        pins.d13.into_output(), 
        pins.d11.into_output(), 
        pins.d12.into_pull_up_input(), 
        pins.d10.into_output(), 
        spi::Settings::default(),
    );
    
    let dc = pins.d9.into_output();
    let rst = pins.d8.into_output();
    let mut delay = arduino_hal::Delay::new();
    
    let mut display = st7735_lcd::ST7735::new(spi, dc, rst, true, false, 160, 128);
    display.init(&mut delay).unwrap();
    display.set_orientation(&st7735_lcd::Orientation::Landscape).unwrap();
    
    // Try the hardware clear method
    display.hard_reset(&mut delay).unwrap();
    
    loop {
        arduino_hal::delay_ms(1000);
    }
}
