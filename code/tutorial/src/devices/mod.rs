use std::fmt;

pub mod distance_sensor;
pub mod status_display;
pub mod zero_borg;
pub mod hall_sensor;

#[derive(PartialEq, Copy, Clone)]
pub struct Voltage(f64);

impl Voltage {
    pub fn from_volts(value: f64) -> Voltage { Voltage(value) }
    pub fn in_volts(&self) -> f64 { self.0 }
}

impl fmt::Display for Voltage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}V", self.0)
    }
}
