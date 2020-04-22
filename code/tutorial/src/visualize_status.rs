use crate::devices::status_display::{Error, StatusDisplay};
use crate::distance::Distance;
use concread::CowCell;
use std::sync::Arc;
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;

pub struct VisualizeStatus {
    distance: Arc<CowCell<Option<Distance>>>,
    join_handle: JoinHandle<()>,
}

impl VisualizeStatus {
    pub fn start() -> Result<VisualizeStatus, Error> {
        let mut display = StatusDisplay::new()?;
        display.set_name("MagLev Prototype")?;
        let distance = Arc::new(CowCell::new(None));
        let distance_reader = distance.clone();
        let join_handle = thread::spawn(move || {
            let mut last_distance = None;
            loop {
                let distance = *distance_reader.read();
                if distance != last_distance {
                    display.set_distance(&distance).unwrap();
                    last_distance = distance;
                }

                sleep(Duration::from_millis(100));
            }
        });
        Ok(VisualizeStatus {
            join_handle,
            distance,
        })
    }

    pub fn join(self) -> thread::Result<()> {
        self.join_handle.join()
    }

    pub fn set_distance(&self, distance: Option<Distance>) {
        let mut write = self.distance.write();
        *write = distance;
        write.commit();
    }
}
