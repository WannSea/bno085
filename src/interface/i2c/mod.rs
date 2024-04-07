mod ids;

use async_trait::async_trait;
use embedded_hal_async::i2c::I2c;
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

pub enum I2CError {
    Default
}

#[async_trait]
impl<I2C> ComInterface for I2CInterface<I2C> where I2C : std::marker::Sync {
    type ComError = I2CError;

    async fn setup(&self) -> Result<(), Self::ComError> {
        Ok(())
    }
    async fn read_packet(&self, recv_buf: &mut[u8]) -> Result<usize, Self::ComError> {
        Ok(0)
    }
    async fn write_packet(&self, packet: &[u8]) -> Result<(), Self::ComError> {
        Ok(())
    }
}
