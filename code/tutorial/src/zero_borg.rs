use crate::zero_borg::command::{ReadCommand, WriteCommand};
use crate::zero_borg::Error::{I2CFailure, NotAZeroBorg};
use embedded_hal::blocking::i2c;

pub struct Address(pub u8);

#[derive(Debug)]
pub enum Error<E> {
    I2CFailure(E),
    NotAZeroBorg { device_id: u8 },
}

pub struct ZeroBorg<I2C> {
    i2c: I2C,
    address: Address,
}

impl<I2C, E> ZeroBorg<I2C>
where
    I2C: i2c::Write<Error = E> + i2c::Read<Error = E>,
{
    pub fn new(i2c: I2C, address: Address) -> Self {
        ZeroBorg { i2c, address }
    }

    pub fn init(&mut self) -> Result<(), Error<E>> {
        const I2C_ID_ZEROBORG: u8 = 0x40;
        let resp = self.ask(ReadCommand::GET_ID)?;
        if resp[1] == I2C_ID_ZEROBORG {
            Ok(())
        } else {
            Err(NotAZeroBorg {
                device_id: self.address.0,
            })
        }
    }

    pub fn get_led_value(&mut self) -> Result<bool, Error<E>> {
        let resp = self.ask(ReadCommand::GET_LED)?;
        return Ok(resp[1] != 0);
    }

    pub fn set_led_value(&mut self, value: bool) -> Result<(), Error<E>> {
        self.send(WriteCommand::set_led(if value { 1 } else { 0 }))
    }

    fn ask(&mut self, command: ReadCommand) -> Result<[u8; 4], Error<E>> {
        self.i2c
            .write(self.address.0, &[command.code])
            .map_err(I2CFailure)?;
        let mut buffer = [0u8; 4];
        self.i2c
            .read(self.address.0, &mut buffer)
            .map_err(I2CFailure)?;
        return Ok(buffer);
    }

    fn send(&mut self, command: WriteCommand) -> Result<(), Error<E>> {
        if command.data.len() == 1 {
            let data = [command.code, command.data[0]];
            self.i2c.write(self.address.0, &data).map_err(I2CFailure)
        } else {
            panic!("Unsupported command length: {}", command.data.len());
        }
    }
}

pub(crate) mod command {
    pub struct ReadCommand {
        pub code: u8,
    }

    impl ReadCommand {
        pub const GET_ID: ReadCommand = ReadCommand { code: 0x99 };
        pub const GET_LED: ReadCommand = ReadCommand { code: 2 };
    }

    pub struct WriteCommand {
        pub code: u8,
        pub data: Vec<u8>,
    }

    impl WriteCommand {
        pub fn set_led(value: u8) -> WriteCommand {
            WriteCommand {
                code: 1,
                data: vec![value],
            }
        }
    }
}
