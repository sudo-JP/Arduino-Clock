use core::option::Option;
use core::option::Option::{Some, None}; 
use core::fmt; 

use core::write;
use core::result::Result::Ok;

pub enum Meridiem {
    AM, 
    PM
}

pub enum TimeChange {
    None, 
    Second, 
    Minute, 
    Hour
}

pub struct Time {
    pub hours: u8,
    pub minutes: u8, 
    pub seconds: u8, 
}

const SECONDS_PER_MINUTE: u8 = 60;
const MINUTES_PER_HOUR: u8 = 60;
const HOURS_PER_DAY: u8 = 24;

impl Time {
    pub fn new(hour: u8, min: u8, sec: u8) -> Option<Self> {
        if hour >= HOURS_PER_DAY || 
            sec >= SECONDS_PER_MINUTE || 
            min >= MINUTES_PER_HOUR 
        {
            return None;     
        }

        Some(Self {
            seconds: sec, 
            minutes: min, 
            hours: hour
        })
    }

    pub fn get_meridiem(&self) -> Meridiem {
        match self.hours < 12 {
            true => Meridiem::AM,
            false => Meridiem::PM,
        }
    }

    pub fn tick(&mut self) -> TimeChange {
        // Increment second 
        self.seconds += 1;
        if self.seconds < SECONDS_PER_MINUTE {
            return TimeChange::Second; 
        }

        // New second 
        self.seconds = 0; 
        self.minutes += 1; 

        if self.minutes < MINUTES_PER_HOUR {
            return TimeChange::Minute;
        }
        // New minutes 
        self.minutes = 0; 

        self.hours += 1; 
        if self.hours < HOURS_PER_DAY {
            return TimeChange::Hour; 
        }
        // New hour 
        self.hours = 0; 

        TimeChange::Hour
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hour = match self.hours % 12 {
            0 => 12, 
            h => h, 
        }; 
        let meridiem = match self.get_meridiem() {
            Meridiem::AM => "AM", 
            Meridiem::PM => "PM"
        };

        write!(f, "{}:{} {}", hour, self.minutes, meridiem)?;
        Ok(()) 
    }
}

