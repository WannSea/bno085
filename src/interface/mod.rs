use async_trait::async_trait;

pub mod i2c;

#[async_trait]
pub trait ComInterface {
    type ComError;
    async fn setup(&self) -> Result<(), Self::ComError>;
    async fn read_packet(&self, recv_buf: &mut[u8]) -> Result<usize, Self::ComError>;
    async fn write_packet(&self, packet: &[u8]) -> Result<(), Self::ComError>;
}
