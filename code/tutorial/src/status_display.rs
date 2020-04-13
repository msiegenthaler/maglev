use embedded_graphics::fonts::{Font6x8, Font8x16, Text};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::style::{PrimitiveStyleBuilder, TextStyleBuilder};
use linux_embedded_hal::spidev::{SpiModeFlags, SpidevOptions};
use linux_embedded_hal::sysfs_gpio::Direction;
use linux_embedded_hal::Pin;
use linux_embedded_hal::{Delay, Spidev};
use st7735_lcd::{Orientation, ST7735};

pub struct StatusDisplay {
    display: ST7735<Spidev, Pin, Pin>,
}

#[derive(Debug)]
pub enum Error {
    InitializationFailed(String),
    CommunicationError(String),
    DrawingError(String),
}

impl StatusDisplay {
    pub fn new() -> Result<StatusDisplay, Error> {
        let spi = create_spi()?;
        let dc = open_pin(25)?;
        let rst = open_pin(27)?;
        let mut delay = Delay;

        let mut display = st7735_lcd::ST7735::new(spi, dc, rst, false, false, 128, 128);
        display.init(&mut delay).map_err(|_| {
            Error::InitializationFailed("Could not initialize the display".to_string())
        })?;
        display
            .set_orientation(&Orientation::Landscape)
            .map_err(|_| Error::InitializationFailed("Could not set orientation".to_string()))?;
        let mut r = StatusDisplay { display };
        r.clear()?;
        Ok(r)
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::BLACK)
            .build();
        let black_backdrop =
            Rectangle::new(Point::new(0, 0), Point::new(128, 128)).into_styled(style);
        self.display.set_offset(0, 1);
        black_backdrop
            .draw(&mut self.display)
            .map_err(|_| Error::DrawingError("could not clear the screen".to_string()))
    }

    pub fn set_number(&mut self, num: i32) -> Result<(), Error> {
        let style = TextStyleBuilder::new(Font8x16)
            .text_color(Rgb565::RED)
            .background_color(Rgb565::BLACK)
            .build();
        let text = format!("# {}", num);
        Text::new(&text, Point::new(10, 10))
            .into_styled(style)
            .draw(&mut self.display)
            .map_err(|_| Error::DrawingError("failed to draw text (number)".to_string()))?;
        Ok(())
    }

    pub fn set_name(&mut self, name: &str) -> Result<(), Error> {
        let style = TextStyleBuilder::new(Font6x8)
            .text_color(Rgb565::GREEN)
            .background_color(Rgb565::BLACK)
            .build();
        let text = format!("Hi {}", name);
        Text::new(&text, Point::new(10, 30))
            .into_styled(style)
            .draw(&mut self.display)
            .map_err(|_| Error::DrawingError("failed to draw text (name)".to_string()))?;
        Ok(())
    }
}

fn open_pin(num: u64) -> Result<Pin, Error> {
    let pin = Pin::new(num);
    pin.export()
        .map_err(|e| Error::InitializationFailed(e.to_string()))?;
    pin.set_direction(Direction::Out)
        .map_err(|e| Error::InitializationFailed(e.to_string()))?;
    Ok(pin)
}

fn create_spi() -> Result<Spidev, Error> {
    let mut spi = Spidev::open("/dev/spidev0.0")
        .map_err(|_| Error::InitializationFailed("error initializing SPI".to_string()))?;
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(20000000)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options)
        .map_err(|_| Error::InitializationFailed("error configuring SPI".to_string()))?;
    Ok(spi)
}
