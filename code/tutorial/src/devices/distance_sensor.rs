use crate::devices::distance_sensor::Error::{InitializationFailed, MeasurementFailed};
use crate::distance::Distance;
use crate::utils::ExportableWait;
use linux_embedded_hal::sysfs_gpio::Pin;
use linux_embedded_hal::sysfs_gpio::{Direction, Edge};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

pub struct DistanceSensor {
    trigger_pin: Pin,
    echo_pin: Pin,
    max_distance: Distance,
}

#[derive(Debug)]
pub enum Error {
    InitializationFailed(String),
    MeasurementFailed(String),
}

const SPEED_OF_SOUND: f64 = 343.5; // metres per second (at 20 degrees celsius)

impl DistanceSensor {
    /**
    * max_distance: use the max value that is interesting in your use case (and supported by
     the hardware). Setting it to a small value will provide speed ups in case of errors, since
      we don't waste time for echos that will never come.
    */
    pub fn new(
        trigger_pin: u64,
        echo_pin: u64,
        max_distance: Distance,
    ) -> Result<DistanceSensor, Error> {
        return Ok(DistanceSensor {
            trigger_pin: create_trigger_pin(trigger_pin)?,
            echo_pin: create_echo_pin(echo_pin)?,
            max_distance,
        });
    }

    pub fn measure_range(&self) -> Result<Distance, Error> {
        // Trigger the sound impulse
        self.trigger()?;

        // Wait until it has been send and note that time as t0
        let value0 = self
            .echo_pin
            .get_value()
            .map_err(|e| MeasurementFailed(format!("Error reading pin value {}", e)))?;
        if value0 == 0 {
            self.await_edge(Edge::RisingEdge, 5)?
                .ok_or(MeasurementFailed(
                    "Timeout while waiting for echo initialization".to_string(),
                ))?;
        }
        let t0 = SystemTime::now();

        //Wait until the echo has been received
        let max_wait = self.max_distance.as_metre() * 2_f64 / SPEED_OF_SOUND * 1000_f64 * 1.5_f64;
        self.await_edge(Edge::FallingEdge, max_wait as isize)?
            .ok_or(MeasurementFailed(
                "Timeout while waiting for echo reception".to_string(),
            ))?;

        // Measure how long it took to receive the echo
        let duration = t0.elapsed().map_err(|e| {
            MeasurementFailed(format!("failed to measure the time interval: {}", e))
        })?;

        let distance =
            Distance::from_mm((SPEED_OF_SOUND * duration.as_secs_f64() / 2_f64 * 1000_f64) as u64);
        if distance > self.max_distance {
            return Err(MeasurementFailed(
                "too far to measure or no echo received".to_string(),
            ));
        }
        Ok(distance)
    }

    fn trigger(&self) -> Result<(), Error> {
        self.trigger_pin
            .set_value(1)
            .map_err(|e| MeasurementFailed(format!("failed to trigger: {}", e)))?;
        sleep(Duration::from_micros(10));
        self.trigger_pin
            .set_value(0)
            .map_err(|e| MeasurementFailed(format!("failed to reset trigger: {}", e)))?;
        Ok(())
    }

    fn await_edge(&self, edge: Edge, max_wait_ms: isize) -> Result<Option<u8>, Error> {
        self.echo_pin
            .set_edge(edge)
            .map_err(|e| MeasurementFailed(format!("failed set polling edge: {}", e)))?;
        self.echo_pin
            .get_poller()
            .map_err(|e| MeasurementFailed(format!("failed to setup echo polling: {}", e)))?
            .poll(max_wait_ms)
            .map_err(|e| MeasurementFailed(format!("failed to poll for echo: {}", e)))
    }
}

fn create_trigger_pin(nr: u64) -> Result<Pin, Error> {
    let trigger = Pin::new(nr);
    trigger
        .ensure_exported()
        .map_err(|e| InitializationFailed(e.to_string()))?;
    trigger
        .set_direction(Direction::Out)
        .map_err(|e| InitializationFailed(e.to_string()))?;
    trigger
        .set_value(0)
        .map_err(|e| InitializationFailed(e.to_string()))?;
    Ok(trigger)
}

fn create_echo_pin(nr: u64) -> Result<Pin, Error> {
    let echo = Pin::new(nr);
    echo.ensure_exported()
        .map_err(|e| InitializationFailed(e.to_string()))?;
    echo.set_direction(Direction::In)
        .map_err(|e| InitializationFailed(e.to_string()))?;
    echo.set_active_low(false)
        .map_err(|e| InitializationFailed(e.to_string()))?;
    Ok(echo)
}
