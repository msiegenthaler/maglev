use crate::devices::Voltage;

/** Linear Hall Effect Sensor, e.g. SS495ASP */
pub struct HallSensor {
    sensor_max: FluxDensity,
    reference_voltage: Voltage,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct FluxDensity(f64);

impl FluxDensity {
    pub fn from_tesla(value: f64) -> FluxDensity { FluxDensity(value) }
    pub fn from_gauss(value: f64) -> FluxDensity { FluxDensity(value / 10000.) }
    pub fn in_tesla(&self) -> f64 { self.0 }
    pub fn in_gauss(&self) -> f64 { self.0 * 10000. }
}

/** value between -1 and 1, relative to the max strength supported by the sensor. */
pub struct Fraction(f64);

impl HallSensor {
    pub fn new(sensor_max: FluxDensity, reference_voltage: Voltage) -> HallSensor {
        HallSensor { sensor_max, reference_voltage }
    }

    pub fn value_as_flux_density(&mut self, voltage: Voltage) -> FluxDensity {
        let relative = self.value_as_relative(voltage);
        FluxDensity::from_tesla(relative.0 * self.sensor_max.in_tesla())
    }

    pub fn value_as_relative(&mut self, voltage: Voltage) -> Fraction {
        Fraction((voltage.in_volts() / self.reference_voltage.in_volts() - 0.5) * 2.)
    }
}