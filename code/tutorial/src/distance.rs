#[derive(Clone, PartialEq, PartialOrd, Copy, Debug)]
pub struct Distance(u64);

impl Distance {
    pub fn from_m(m: f64) -> Distance {
        Distance((m * 1000_f64).round() as u64)
    }
    pub fn from_mm(mm: u64) -> Distance {
        Distance(mm)
    }

    pub fn as_millimetre(&self) -> u64 {
        self.0
    }

    pub fn as_centimetre(&self) -> f64 {
        (self.0 as f64) / 10_f64
    }

    pub fn as_metre(&self) -> f64 {
        (self.0 as f64) / 1000_f64
    }
}
