use std::time::{Duration, SystemTime};
use tutorial::aggregator::MedianOverWindow;
use tutorial::devices::distance_sensor::DistanceSensor;
use tutorial::distance::Distance;
use tutorial::measurer::Measurer;
use tutorial::visualize_status::VisualizeStatus;

fn main() {
    println!("initializing...");
    let distance_sensor = DistanceSensor::new(20, 21, Distance::from_mm(1000)).unwrap();
    let status = VisualizeStatus::start().unwrap();
    println!("initialized.");

    let log = false;

    let start_time = SystemTime::now();
    let distance = Measurer::start(
        move || match distance_sensor.measure_range() {
            Ok(d) => {
                if log {
                    println!(
                        "{:?} Measured: {:.1}",
                        start_time.elapsed().unwrap(),
                        d.as_centimetre()
                    );
                }
                Some(d.as_millimetre() as f64)
            }
            Err(e) => {
                if log {
                    println!(
                        "{:?} Distance failed: {:?}",
                        start_time.elapsed().unwrap(),
                        e
                    );
                }
                None
            }
        },
        MedianOverWindow::new(15),
        Duration::from_millis(15),
    );

    loop {
        let d = distance.value();
        status.set_distance(d.map(|x| x as u64).map(Distance::from_mm));
    }

    // status.join().unwrap();
    // distance.join().unwrap();
}
