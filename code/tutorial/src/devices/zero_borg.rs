use embedded_hal::blocking::i2c;
use crate::devices::zero_borg::command::{ReadCommand, WriteCommand};

#[derive(Debug, Copy, Clone)]
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

const I2C_ID_ZEROBORG: u8 = 0x40;

impl<I2C, E> ZeroBorg<I2C>
    where I2C: i2c::Write<Error=E> + i2c::Read<Error=E>,
{
    pub fn new(i2c: I2C, address: Address) -> Self {
        ZeroBorg { i2c, address }
    }

    /** Scans the i2c bus for zeroborgs */
    pub fn scan(i2c: &mut I2C) -> Vec<Address> {
        let mut result: Vec<Address> = Vec::new();
        for a in 0x03_u8..0x78_u8 {
            let address = Address(a);
            match ReadCommand::GET_ID.execute(i2c, &address) {
                Ok(data) =>
                    if data[1] == I2C_ID_ZEROBORG {
                        result.push(address);
                    },
                Err(_) => {}
            }
        }
        result
    }

    pub fn init(&mut self) -> Result<(), Error<E>> {
        let resp = self.ask(ReadCommand::GET_ID)?;
        if resp[1] == I2C_ID_ZEROBORG {
            Ok(())
        } else {
            Err(Error::NotAZeroBorg {
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
        command.execute(&mut self.i2c, &self.address)
    }

    fn send(&mut self, command: WriteCommand) -> Result<(), Error<E>> {
        command.execute(&mut self.i2c, &self.address)
    }
}


pub(crate) mod command {
    use crate::devices::zero_borg::{Address, Error};
    use embedded_hal::blocking::i2c;

    pub struct ReadCommand {
        pub code: u8,
    }

    impl ReadCommand {
        pub const GET_ID: ReadCommand = ReadCommand { code: 0x99 };
        pub const GET_LED: ReadCommand = ReadCommand { code: 2 };

        pub fn execute<I2C, E>(&self, i2c: &mut I2C, address: &Address) -> Result<[u8; 4], Error<E>>
            where I2C: i2c::Write<Error=E> + i2c::Read<Error=E> {
            i2c
                .write(address.0, &[self.code])
                .map_err(Error::I2CFailure)?;
            let mut buffer = [0u8; 4];
            i2c
                .read(address.0, &mut buffer)
                .map_err(Error::I2CFailure)?;
            return Ok(buffer);
        }
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

        pub fn execute<I2C, E>(&self, i2c: &mut I2C, address: &Address) -> Result<(), Error<E>>
            where I2C: i2c::Write<Error=E> + i2c::Read<Error=E> {
            if self.data.len() == 1 {
                let data = [self.code, self.data[0]];
                i2c.write(address.0, &data).map_err(Error::I2CFailure)
            } else {
                panic!("Unsupported command length: {}", self.data.len());
            }
        }
    }
}
