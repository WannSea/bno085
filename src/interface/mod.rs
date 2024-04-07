
pub mod i2c;

pub trait ComInterface {
    type ComError;
    fn setup(&mut self) -> Result<(), Self::ComError>;
    fn read_bytes(&mut self, recv_buf: &mut[u8]) -> Result<(), Self::ComError>;
    fn write_bytes(&mut self, packet: &[u8]) -> Result<(), Self::ComError>;
}
