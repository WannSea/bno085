#[derive(Debug)]

pub enum ChannelCommandData {
    AdvertiseResponse,
    ErrorList,
    Unknown(u8)
}

#[derive(Debug)]

pub enum ChannelExecutableData {
    ResetComplete,
    Unknown(u8)
}

#[derive(Debug)]

pub enum ChannelHubControlData {
    CommandResponse,
    ProdIdResponse,
    GetFeatureResponse,
    Unknown(u8)
}

#[derive(Debug)]
pub struct SensorData<const EL_COUNT: usize, T: Sized=f32> {
    id: u8,
    seq_number: u8,
    status: u8,
    values: [T; EL_COUNT]
}

impl<const EL_COUNT: usize, T: Sized> SensorData<EL_COUNT, T> {
    pub fn new(id: u8, seq_number: u8, status: u8, values: [T; EL_COUNT]) -> Self {
        return SensorData { id, seq_number, status, values }
    }
}

#[derive(Debug)]
pub enum SensorReportData {
    Acceleration(SensorData<3>), // 0x01
    GyroCalibrated(SensorData<3>), // 0x02
    MagFieldCalibrated(SensorData<3>), // 0x03

    LinearAcceleration(SensorData<3>), // 0x04
    Rotation(SensorData<5>), // 0x05
    Gravity(SensorData<1>), // 0x06
    Unknown(u8)
}

#[derive(Debug)]
pub enum BnoPacket {
    ChannelCommand(ChannelCommandData),
    ChannelExec(ChannelExecutableData),
    ChannelHubControl(ChannelHubControlData),
    SensorReports(Vec<SensorReportData>)
}

#[derive(Debug)]
pub enum BnoPacketParseError {
    Unknown
}

