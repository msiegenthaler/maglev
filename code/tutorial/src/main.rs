use std::thread;
use std::time::Duration;
use tutorial::devices::distance_sensor::DistanceSensor;
use tutorial::visualize_status::VisualizeStatus;

fn main() {
    println!("initializing...");
    let distance_sensor = DistanceSensor::new(20, 21).unwrap();
    let status = VisualizeStatus::start().unwrap();
    println!("initialized.");

    loop {
        match distance_sensor.measure_range() {
            Ok(distance) => status.set_distance(Some(distance)),
            Err(e) => {
                println!("Measurement failed {:?}", e);
                status.set_distance(None)
            }
        }

        thread::sleep(Duration::from_millis(10));
    }
}
