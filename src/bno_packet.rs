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
pub struct SensorData<const EL_COUNT: usize, T: Sized + std::clone::Clone=f32> {
    pub id: u8,
    pub seq_number: u8,
    pub status: u8,
    pub values: [T; EL_COUNT]
}

impl<const EL_COUNT: usize, T: Sized + std::clone::Clone> SensorData<EL_COUNT, T> {
    pub fn new(id: u8, seq_number: u8, status: u8, values: [T; EL_COUNT]) -> Self {
        return SensorData { id, seq_number, status, values }
    }

    pub fn get_vec(&self) -> Vec<T> {
        return self.values.to_vec();
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

