use linux_embedded_hal::I2cdev;
use std::thread;
use std::time::{Duration, SystemTime};
use tutorial::aggregator::MedianOverWindow;
use tutorial::devices::distance_sensor::DistanceSensor;
use tutorial::devices::hall_sensor::{FluxDensity, HallSensor};
use tutorial::devices::zero_borg::{AnalogSource, Led, Motor, MotorPower, ZeroBorg};
use tutorial::devices::Voltage;
use tutorial::distance::Distance;
use tutorial::measurer::Measurer;
use tutorial::visualize_status::VisualizeStatus;

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

    let magnetic = HallSensor::new(FluxDensity::from_gauss(670.), Voltage::from_volts(3.3));

    borg.set_led_value(Led::MainLed, true).unwrap();
    borg.set_led_value(Led::IRLed, true).unwrap();
    borg.reset_emergency_power_off().unwrap();
    borg.set_motor(Motor::Motor1, MotorPower::off()).unwrap();
    borg.set_motor(Motor::Motor2, MotorPower::off()).unwrap();
    // borg.set_motor(Motor::Motor1, MotorPower::full_forward()).unwrap();
    borg.set_motor(Motor::Motor2, MotorPower::full_forward()).unwrap();
    // borg.set_motor(Motor::Motor2, MotorPower::full_forward()).unwrap();


    balance(borg, magnetic);

    // let mut i = 0_u32;
    // let interval = 5;
    // loop {
    //     if (i / interval) % 2 == 0 {
    //         borg.set_motor(Motor::Motor2, MotorPower::full_forward()).unwrap();
    //     } else if (i / interval) % 2 == 2 {
    //         borg.set_motor(Motor::Motor2, MotorPower::off()).unwrap();
    //     } else {
    //         borg.set_motor(Motor::Motor2, MotorPower::full_backward()).unwrap();
    //     }
    //     i += 1;
    //
    //     let analog1 = borg.get_analog(AnalogSource::Analog1).unwrap().voltage();
    //
    //     println!("values m={:>4.0} Gs    m2={:>5?} led={:>5} ir_led={:>5} epo={:>5}",
    //              magnetic.value_as_flux_density(analog1).in_gauss(),
    //              borg.get_motor(Motor::Motor2).unwrap(),
    //              borg.get_led_value(Led::MainLed).unwrap(),
    //              borg.get_led_value(Led::IRLed).unwrap(),
    //              borg.get_emergency_power_off_state().unwrap(),
    //     );
    //
    //     thread::sleep(Duration::from_millis(100));
    //
    //     // let led_value = borg.get_led_value(Led::MainLed).unwrap();
    //     // borg.set_led_value(Led::MainLed, !led_value).unwrap();
    // }
}

fn balance(mut borg: ZeroBorg<I2cdev>, hall_sensor: HallSensor) {
    let target_flux = FluxDensity::from_gauss(45.);
    let flux_range = FluxDensity::from_gauss(5.);
    let motor_per_gauss = 3.;

    borg.set_motor(Motor::Motor2, MotorPower::off()).unwrap();
    loop {
        let flux_density = hall_sensor.value_as_flux_density(borg.get_analog(AnalogSource::Analog1).unwrap().voltage());
        let correction = ((flux_density - target_flux).in_gauss() * motor_per_gauss).abs().min(255.) as u8;

        // borg.set_motor(Motor::Motor2, MotorPower::full_forward()).unwrap();

        if flux_density > target_flux + flux_range {
            println!("Current value is {:>4.0} Gs => Correcting (backward by {})", flux_density.in_gauss(), correction);
            borg.set_motor(Motor::Motor2, MotorPower::backward(correction)).unwrap();
        } else if flux_density < target_flux - flux_range {
            println!("Current value is {:>4.0} Gs => Correcting (forward by {})", flux_density.in_gauss(), correction);
            borg.set_motor(Motor::Motor2, MotorPower::forward(correction)).unwrap();
        } else {
            borg.set_motor(Motor::Motor2, MotorPower::off()).unwrap();
        }

        thread::sleep(Duration::from_millis(5));
        // thread::sleep(Duration::from_millis(1000));
        // borg.set_motor(Motor::Motor2, MotorPower::off()).unwrap();
        // thread::sleep(Duration::from_millis(1000));
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
