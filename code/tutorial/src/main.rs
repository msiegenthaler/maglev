use std::time::{Duration, SystemTime};
use tutorial::aggregator::MedianOverWindow;
use tutorial::devices::distance_sensor::DistanceSensor;
use tutorial::distance::Distance;
use tutorial::measurer::Measurer;
use tutorial::visualize_status::VisualizeStatus;
use tutorial::devices::zero_borg::ZeroBorg;
use linux_embedded_hal::I2cdev;
use std::thread;

fn main() {
    println!("Starting...");

    let mut i2c = I2cdev::new("/dev/i2c-1").unwrap();
    let zero_borgs = ZeroBorg::scan(&mut i2c);
    println!("{:?}", zero_borgs);
    if zero_borgs.is_empty() {
        panic!("No ZeroBorg found");
    }

    let borg0_addr = *zero_borgs.get(0).unwrap();
    let mut borg = ZeroBorg::new(i2c, borg0_addr);

    loop {
        println!("Turning on");
        borg.set_led_value(true).unwrap();
        println!("value read from borg {}", borg.get_led_value().unwrap());

        thread::sleep(Duration::from_millis(500));

        println!("Turning off");
        borg.set_led_value(false).unwrap();
        println!("value read from borg {}", borg.get_led_value().unwrap());

        thread::sleep(Duration::from_millis(500));
    }
}

#[allow(dead_code)]
fn main2() {
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
