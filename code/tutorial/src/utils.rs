use linux_embedded_hal::sysfs_gpio::Error;
use linux_embedded_hal::sysfs_gpio::Pin;
use std::thread::sleep;
use std::time::Duration;

pub trait ExportableWait {
    fn ensure_exported(&self) -> Result<(), Error>;
}

impl ExportableWait for Pin {
    fn ensure_exported(&self) -> Result<(), Error> {
        if self.is_exported() {
            return Ok(());
        }
        self.export()?;
        for _ in 1..100 {
            if self.is_exported() {
                return Ok(());
            }
            sleep(Duration::from_millis(10));
        }
        return Err(Error::Unexpected("timeout while exporting".to_string()));
    }
}
