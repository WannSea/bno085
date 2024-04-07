mod ids;

use async_trait::async_trait;
use embedded_hal::i2c::{I2c, SevenBitAddress};
use self::ids::DEFAULT_ADDRESS;
use super::ComInterface;

pub struct I2CInterface<I2C> {
    interface: I2C,
    address: u8
}

impl<I2C: I2c> I2CInterface<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self { interface: i2c, address: DEFAULT_ADDRESS }
    }
}

#[derive(Debug)]
pub enum I2CError {
    Default
}

#[async_trait]
impl<I2C: I2c, CommE> ComInterface for I2CInterface<I2C> where
    I2C: embedded_hal::i2c::I2c<SevenBitAddress, Error = CommE> {
    type ComError = CommE;

    fn setup(&mut self) -> Result<(), Self::ComError> {
        Ok(())
    }
    fn read_bytes(&mut self, recv_buf: &mut[u8]) -> Result<(), Self::ComError> {
        self.interface.read(self.address, recv_buf)?;
        Ok(())
    }
    fn write_bytes(&mut self, packet: &[u8]) -> Result<(), Self::ComError> {
        self.interface.write(self.address, packet)?;
        Ok(())
    }
}
