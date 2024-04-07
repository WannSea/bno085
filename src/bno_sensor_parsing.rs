use crate::{
    bno_constants::{
        PACKET_HEADER_LENGTH, SENSOR_REPORTID_ACCEL, SENSOR_REPORTID_GRAVITY, SENSOR_REPORTID_GYRO_CALIBRATED, SENSOR_REPORTID_LINEAR_ACCEL, SENSOR_REPORTID_MAG_CALIBRATED, SENSOR_REPORTID_ROTATION_VECTOR
    },
    bno_packet::{BnoPacket, BnoPacketParseError, SensorData, SensorReportData},
};

const Q4_SCALE: f32 = 1.0 / ((1 << 4) as f32);
const Q8_SCALE: f32 = 1.0 / ((1 << 8) as f32);
const Q9_SCALE: f32 = 1.0 / ((1 << 9) as f32);
const Q12_SCALE: f32 = 1.0 / ((1 << 12) as f32);
const Q14_SCALE: f32 = 1.0 / ((1 << 14) as f32);

fn read_u8_at_cursor(msg: &[u8], cursor: &mut usize) -> u8 {
    let val = msg[*cursor];
    *cursor += 1;
    val
}

fn read_i16_at_cursor(msg: &[u8], cursor: &mut usize) -> i16 {
    let val = (msg[*cursor] as i16) | ((msg[*cursor + 1] as i16) << 8);
    *cursor += 2;
    val
}

fn q14_to_f32(msg: &[u8], cursor: &mut usize) -> f32 {
    let q_val = read_i16_at_cursor(msg, cursor);
    (q_val as f32) * Q14_SCALE
}

fn q12_to_f32(msg: &[u8], cursor: &mut usize) -> f32 {
    let q_val = read_i16_at_cursor(msg, cursor);
    (q_val as f32) * Q12_SCALE
}

fn q4_to_f32(msg: &[u8], cursor: &mut usize) -> f32 {
    let q_val = read_i16_at_cursor(msg, cursor);
    (q_val as f32) * Q4_SCALE
}

fn q8_to_f32(msg: &[u8], cursor: &mut usize) -> f32 {
    let q_val = read_i16_at_cursor(msg, cursor);
    (q_val as f32) * Q8_SCALE
}

fn q9_to_f32(msg: &[u8], cursor: &mut usize) -> f32 {
    let q_val = read_i16_at_cursor(msg, cursor);
    (q_val as f32) * Q9_SCALE
}

pub fn parse_sensor_reports(data: &[u8]) -> Result<BnoPacket, BnoPacketParseError> {
    let received_len = data.len();

    let mut cursor: usize = PACKET_HEADER_LENGTH + 5; //skip header, timestamp
                                                      //TODO need to skip more above for a payload-level timestamp??
    if received_len < cursor {
        println!("bad lens: {} < {}", received_len, cursor);
        return Err(BnoPacketParseError::Unknown);
    }

    let payload_len = received_len - cursor;
    if payload_len < 14 {
        println!(
            "bad report - payload len {:?} - received len {:?}",
            payload_len, received_len
        );
        return Err(BnoPacketParseError::Unknown);
    }

    let mut reports: Vec<SensorReportData> = Vec::new();
    // Batching queue
    while cursor < payload_len {
        let id = read_u8_at_cursor(data, &mut cursor);
        let seq_num = read_u8_at_cursor(data, &mut cursor);
        let status = read_u8_at_cursor(data, &mut cursor);
        let _delay = read_u8_at_cursor(data, &mut cursor);

        let data = match id {
            SENSOR_REPORTID_ACCEL => SensorReportData::Acceleration(SensorData::new(
                id,
                seq_num,
                status,
                [
                    q8_to_f32(data, &mut cursor),
                    q8_to_f32(data, &mut cursor),
                    q8_to_f32(data, &mut cursor),
                ],
            )),
            SENSOR_REPORTID_GYRO_CALIBRATED => SensorReportData::GyroCalibrated(SensorData::new(
                id,
                seq_num,
                status,
                [
                    q9_to_f32(data, &mut cursor),
                    q9_to_f32(data, &mut cursor),
                    q9_to_f32(data, &mut cursor),
                ],
            )),
            SENSOR_REPORTID_MAG_CALIBRATED => SensorReportData::MagFieldCalibrated(SensorData::new(
                id,
                seq_num,
                status,
                [
                    q4_to_f32(data, &mut cursor),
                    q4_to_f32(data, &mut cursor),
                    q4_to_f32(data, &mut cursor),
                ],
            )),
            SENSOR_REPORTID_LINEAR_ACCEL => SensorReportData::LinearAcceleration(SensorData::new(
                id,
                seq_num,
                status,
                [
                    q8_to_f32(data, &mut cursor),
                    q8_to_f32(data, &mut cursor),
                    q8_to_f32(data, &mut cursor),
                ],
            )),
            SENSOR_REPORTID_ROTATION_VECTOR => 
                SensorReportData::Rotation(SensorData::new(
                    id,
                    seq_num,
                    status,
                    [
                        q14_to_f32(data, &mut cursor),
                        q14_to_f32(data, &mut cursor),
                        q14_to_f32(data, &mut cursor),
                        q14_to_f32(data, &mut cursor),
                        q12_to_f32(data, &mut cursor),
                    ],
            )),
            SENSOR_REPORTID_GRAVITY => SensorReportData::Gravity(SensorData::new(
                id,
                seq_num,
                status,
                [
                    q8_to_f32(data, &mut cursor),
                ],
            )),
            _ => SensorReportData::Unknown(id),
        };
        reports.push(data);
    }

    return Ok(BnoPacket::SensorReports(reports));
}
