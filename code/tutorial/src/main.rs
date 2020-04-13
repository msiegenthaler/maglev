use std::thread;
use std::time::Duration;

use tutorial::status_display::StatusDisplay;

fn main() {
    println!("initializing...");
    let mut display = StatusDisplay::new().unwrap();
    println!("initialized.");

    display.set_name("Marcus").unwrap();
    let mut x = 1;
    loop {
        display.set_number(x).unwrap();
        println!("Number is {}", x);

        thread::sleep(Duration::from_millis(500));
        x = x + 1;
    }
}
