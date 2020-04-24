use std::time::{Duration, SystemTime};
use tutorial::aggregator::MedianOverWindow;
use tutorial::devices::distance_sensor::DistanceSensor;
use tutorial::distance::Distance;
use tutorial::measurer::Measurer;
use tutorial::visualize_status::VisualizeStatus;
use tutorial::devices::zero_borg::{ZeroBorg, Led, AnalogSource, Motor};
use linux_embedded_hal::I2cdev;
use std::thread;
use tutorial::devices::hall_sensor::{HallSensor, FluxDensity};
use tutorial::devices::Voltage;

fn main() {
    println!("Starting...");

    let mut i2c = I2cdev::new("/dev/i2c-1").unwrap();
    let zero_borgs = ZeroBorg::scan(&mut i2c);
    println!("-found {} zero borgs", zero_borgs.len());
    if zero_borgs.is_empty() {
        panic!("No ZeroBorg found");
    }
    println!("Started");

    let borg0_address = *zero_borgs.get(0).unwrap();
    let mut borg = ZeroBorg::new(i2c, borg0_address);

    let mut magnetic = HallSensor::new(FluxDensity::from_gauss(670.), Voltage::from_volts(3.3));

    borg.set_led_value(Led::MainLed, true).unwrap();
    borg.set_led_value(Led::IRLed, true).unwrap();

    loop {
        let analog1 = borg.get_analog(AnalogSource::Analog1).unwrap().voltage();

        println!("values m={:.0} Gauss m1={:?} led={:?} ir_led={:?}",
                 magnetic.value_as_flux_density(analog1).in_gauss(),
                 borg.get_motor(Motor::Motor1).unwrap(),
                 borg.get_led_value(Led::MainLed).unwrap(),
                 borg.get_led_value(Led::IRLed).unwrap(),
        );

        thread::sleep(Duration::from_millis(100));
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
