use embedded_hal::blocking::i2c;
use crate::devices::zero_borg::command::{ReadCommand, WriteCommand};
use crate::devices::Voltage;

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

pub enum Led {
    MainLed,
    IRLed,
}

pub enum Motor {
    Motor1,
    Motor2,
    Motor3,
    Motor4,
}

pub enum AnalogSource {
    Analog1,
    Analog2,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AnalogValue(u16);

impl AnalogValue {
    /** voltage based on the 3.3V reference pin (pin 1) */
    pub fn voltage(&self) -> Voltage {
        Voltage(self.fraction() * 3.3)
    }

    /** value between 0 and 1 */
    pub fn fraction(&self) -> f64 {
        (self.0 as f64) / (0x3FF as f64)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MotorPower(u8, bool);

impl MotorPower {
    pub fn off() -> MotorPower { MotorPower(0, true) }
    pub fn full_forward() -> MotorPower { MotorPower::forward(255) }
    pub fn full_backward() -> MotorPower { MotorPower::backward(255) }
    pub fn forward(value: u8) -> MotorPower { MotorPower(value, true) }
    pub fn backward(value: u8) -> MotorPower { MotorPower(value, false) }

    pub fn is_forward(&self) -> bool { self.1 }
    pub fn is_backward(&self) -> bool { !self.1 }
}

const I2C_ID_ZEROBORG: u8 = 0x40;

impl<I2C, E> ZeroBorg<I2C>
    where I2C: i2c::Write<Error=E> + i2c::Read<Error=E>,
{
    pub fn new(i2c: I2C, address: Address) -> Self {
        ZeroBorg { i2c, address }
    }

    /** Scans the i2c bus for zeroborg devices */
    pub fn scan(i2c: &mut I2C) -> Vec<Address> {
        let mut result: Vec<Address> = Vec::new();
        for a in 0x03_u8..0x78_u8 {
            let address = Address(a);
            match ReadCommand::get_id().execute(i2c, &address) {
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
        let resp = self.ask(ReadCommand::get_id())?;
        if resp[1] == I2C_ID_ZEROBORG {
            Ok(())
        } else {
            Err(Error::NotAZeroBorg {
                device_id: self.address.0,
            })
        }
    }

    pub fn get_led_value(&mut self, led: Led) -> Result<bool, Error<E>> {
        let resp = self.ask(ReadCommand::get_led(led))?;
        return Ok(resp[1] != 0);
    }

    pub fn set_led_value(&mut self, led: Led, value: bool) -> Result<(), Error<E>> {
        self.send(WriteCommand::set_led(led, if value { 1 } else { 0 }))
    }

    pub fn get_motor(&mut self, motor: Motor) -> Result<MotorPower, Error<E>> {
        let resp = self.ask(ReadCommand::get_motor(motor))?;
        let forward = resp[1] != 2;
        return Ok(MotorPower(resp[2], forward));
    }

    pub fn set_motor(&mut self, motor: Motor, power: MotorPower) -> Result<(), Error<E>> {
        self.send(WriteCommand::set_motor(motor, power))
    }

    pub fn turn_everything_off(&mut self) -> Result<(), Error<E>> {
        self.send(WriteCommand::all_off())
    }

    pub fn get_analog(&mut self, analog: AnalogSource) -> Result<AnalogValue, Error<E>> {
        let resp = self.ask(ReadCommand::get_analog(analog))?;
        let raw = ((resp[1] as u16) << 8) + (resp[2] as u16);
        Ok(AnalogValue(raw))
    }

    fn ask(&mut self, command: ReadCommand) -> Result<[u8; 4], Error<E>> {
        command.execute(&mut self.i2c, &self.address)
    }

    fn send(&mut self, command: WriteCommand) -> Result<(), Error<E>> {
        command.execute(&mut self.i2c, &self.address)
    }
}


pub(crate) mod command {
    use crate::devices::zero_borg::{Address, Error, Motor, MotorPower, AnalogSource, Led};
    use embedded_hal::blocking::i2c;

    fn motor_offset(motor: Motor) -> u8 {
        match motor {
            Motor::Motor1 => 3,
            Motor::Motor2 => 6,
            Motor::Motor3 => 9,
            Motor::Motor4 => 12,
        }
    }

    pub struct ReadCommand {
        pub code: u8,
    }

    impl ReadCommand {
        pub fn get_id() -> ReadCommand { ReadCommand { code: 0x99 } }

        pub fn get_led(led: Led) -> ReadCommand {
            ReadCommand {
                code: match led {
                    Led::MainLed => 2,
                    Led::IRLed => 27,
                }
            }
        }

        pub fn get_motor(motor: Motor) -> ReadCommand {
            ReadCommand { code: motor_offset(motor) + 2 }
        }

        pub fn get_analog(analog: AnalogSource) -> ReadCommand {
            ReadCommand {
                code: match analog {
                    AnalogSource::Analog1 => 28,
                    AnalogSource::Analog2 => 29,
                }
            }
        }

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
        pub fn all_off() -> WriteCommand {
            WriteCommand {
                code: 15,
                data: vec![0],
            }
        }

        pub fn set_led(led: Led, value: u8) -> WriteCommand {
            WriteCommand {
                code: match led {
                    Led::MainLed => 1,
                    Led::IRLed => 26,
                },
                data: vec![value],
            }
        }

        pub fn set_motor(motor: Motor, power: MotorPower) -> WriteCommand {
            WriteCommand {
                code: motor_offset(motor) + if power.is_backward() { 1 } else { 0 },
                data: vec![power.0],
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
