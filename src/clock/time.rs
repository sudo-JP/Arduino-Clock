pub enum Meridiem {
    AM, 
    PM
}

pub struct Time {
    pub hours: u8,
    pub minutes: u8, 
    pub seconds: u8, 
    pub millis: u16, 
}

impl Time {
    pub fn get_meridiem(&self) -> Meridiem {
        match self.hours < 12 {
            true => Meridiem::AM,
            false => Meridiem::PM,
        }
    }
}
