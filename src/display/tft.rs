use embedded_hal::digital;
use heapless::format;
use st7735_lcd::Orientation;
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle}, 
    pixelcolor::{Rgb565}, prelude::*, 
    text::Text
};
use core::option::Option;
use core::option::Option::Some;
use core::result::Result;
use core::result::Result::Ok;
use core::fmt::Write;
use core::write;


use crate::time::*;


pub struct TFTScreen<SPI: embedded_hal::spi::SpiDevice, 
    DC: embedded_hal::digital::OutputPin, 
    RST: embedded_hal::digital::OutputPin> 
{
    pub display: st7735_lcd::ST7735<SPI, DC, RST>,
}


impl <SPI, DC, RST> TFTScreen<SPI, DC, RST>
where SPI: embedded_hal::spi::SpiDevice, 
    DC: embedded_hal::digital::OutputPin, 
    RST: embedded_hal::digital::OutputPin
{
    pub fn new(spi: SPI, dc: DC, rst: RST, delay: &mut arduino_hal::Delay) -> Option<Self> {

        let mut display = st7735_lcd::ST7735::new(spi, dc, rst, true, false, 160, 128);

        // Init
        display.init(delay).ok()?;

        // Orientation
        display.set_orientation(&Orientation::Landscape).ok()?; 

        // Clear display 
        display.clear(Rgb565::BLACK).ok()?;

        Some(Self {
            display: display
        })
    }

    pub fn clear_screen(&mut self) -> Result<(), ()> {
        self.display.clear(Rgb565::BLACK)?;
        Ok(())
    }

    pub fn draw_time(&mut self, t: &Time) -> Result<(), ()>  {
        let style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE); 

        let mut buf = heapless::String::<16>::new();
        write!(&mut buf, "{}", t).ok(); 

        let s = buf.as_str();

        Text::new(s, Point::new(20, 30), style).draw(&mut self.display)?; 
        Ok(())
    }
}

