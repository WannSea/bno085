pub(crate) const PACKET_HEADER_LENGTH: usize = 4;
pub(crate) const MAX_CARGO_DATA_LENGTH: usize = 32766 - PACKET_HEADER_LENGTH;

// The BNO080 supports six communication channels:
pub const CHANNEL_COMMAND: u8 = 0;
/// the SHTP command channel
pub const CHANNEL_EXECUTABLE: u8 = 1;
/// executable channel
pub const CHANNEL_HUB_CONTROL: u8 = 2;
/// sensor hub control channel
pub const CHANNEL_SENSOR_REPORTS: u8 = 3;

pub const CMD_RESP_ADVERTISEMENT: u8 = 0;
pub const CMD_RESP_ERROR_LIST: u8 = 1;


pub const EXECUTABLE_DEVICE_RESP_RESET_COMPLETE: u8 = 1;


pub const SHUB_COMMAND_RESP: u8 = 0xF1;
pub const SHUB_PROD_ID_RESP: u8 = 0xF8;
pub const SHUB_GET_FEATURE_RESP: u8 = 0xFC;

// Write
pub const EXECUTABLE_DEVICE_CMD_RESET: u8 = 1;
pub const SHUB_REPORT_SET_FEATURE_CMD: u8 = 0xFD;

pub const SENSOR_REPORTID_TS_BASE: u8 = 0xFB;
pub const SENSOR_REPORTID_ACCEL: u8 = 0x01;
pub const SENSOR_REPORTID_GYRO_CALIBRATED: u8 = 0x02;
pub const SENSOR_REPORTID_MAG_CALIBRATED: u8 = 0x03;
pub const SENSOR_REPORTID_LINEAR_ACCEL: u8 = 0x04;
pub const SENSOR_REPORTID_ROTATION_VECTOR: u8 = 0x05;
pub const SENSOR_REPORTID_GRAVITY: u8 = 0x06;
pub const SENSOR_REPORTID_GAME_ROTATION_VECTOR: u8 = 0x08;
