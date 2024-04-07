use core::ops::{Shl,Shr};

use crate::{bno_common::parse_packet_header, bno_constants::{self, CHANNEL_EXECUTABLE,  PACKET_HEADER_LENGTH}, bno_packet::{ BnoPacket, BnoPacketParseError, ChannelCommandData, ChannelExecutableData, ChannelHubControlData}, bno_sensor_parsing::parse_sensor_reports, interface::ComInterface};

const NUM_CHANNELS: usize = 6;
const PACKET_RECV_BUF_LEN: usize = 32766;
const PACKET_SEND_BUF_LEN: usize = 256;

#[derive(Debug)]
pub enum DriverError<E> {
    ///Communications error
    CommError(E),
    /// Invalid chip ID was read
    InvalidChipId(u8),
    /// Unsupported sensor firmware version
    InvalidFWVersion(u8),
    /// We expected some data but didn't receive any
    NoDataAvailable,
    ParseError(BnoPacketParseError),
    UnknownData
}

pub struct BnoDriver<CI> {
    pub interface: CI,
    recv_buffer: [u8; PACKET_RECV_BUF_LEN],
    send_buf: [u8; PACKET_SEND_BUF_LEN],
    sequence_numbers: [u8; NUM_CHANNELS]
}

impl<CI, CE>  BnoDriver<CI> where CI: ComInterface<ComError = CE>, CE: core::fmt::Debug {
    pub fn new(interface: CI) -> Self {
        Self{ interface, recv_buffer: [0; PACKET_RECV_BUF_LEN], send_buf: [0; PACKET_SEND_BUF_LEN], sequence_numbers: [0; NUM_CHANNELS] }
    }

    pub fn setup(&mut self) {
        self.interface.setup().unwrap();
    }

   
    /// Enable a particular report
    pub fn enable_report(
        &mut self,
        report_id: u8,
        millis_between_reports: u16,
        millis_max_delay: u16
    ) -> Result<(), DriverError<CE>> {
        let micros_between_reports: u32 =
            (millis_between_reports as u32) * 1000;
        let micros_max_delay: u32 =
            (millis_max_delay as u32) * 1000;
        let cmd_body: [u8; 17] = [
            bno_constants::SHUB_REPORT_SET_FEATURE_CMD,
            report_id,
            0,                                        //feature flags
            0,                                        //LSB change sensitivity
            0,                                        //MSB change sensitivity
            (micros_between_reports & 0xFFu32) as u8, // LSB report interval, microseconds
            (micros_between_reports.shr(8) & 0xFFu32) as u8,
            (micros_between_reports.shr(16) & 0xFFu32) as u8,
            (micros_between_reports.shr(24) & 0xFFu32) as u8, // MSB report interval
            (micros_max_delay & 0xFFu32) as u8, // LSB batch interval, microseconds
            (micros_max_delay.shr(8) & 0xFFu32) as u8,
            (micros_max_delay.shr(16) & 0xFFu32) as u8,
            (micros_max_delay.shr(24) & 0xFFu32) as u8, // MSB batch interval
            0, // LSB sensor-specific config
            0,
            0,
            0, // MSB sensor-specific config
        ];

       
        //we simply blast out this configuration command and assume it'll succeed
        self.send_packet(bno_constants::CHANNEL_HUB_CONTROL, &cmd_body)?;
        // any error or success in configuration will arrive some time later

        Ok(())
    }

    fn read_packet_header(&mut self) -> Result<(), DriverError<CE>> {
        self.interface.read_bytes(&mut self.recv_buffer[..PACKET_HEADER_LENGTH]).map_err(|e| DriverError::CommError(e))?;
        Ok(())
    }

    pub fn soft_reset(&mut self) -> Result<(), DriverError<CE>> {
        let data: [u8; 1] = [bno_constants::EXECUTABLE_DEVICE_CMD_RESET];

        self.send_packet(CHANNEL_EXECUTABLE, &data)
    }

    pub fn send_packet(&mut self, channel: u8, body_data: &[u8]) -> Result<(), DriverError<CE>> {
        let body_len = body_data.len();

        let packet_length = body_len + PACKET_HEADER_LENGTH;
        let packet_header = [
            (packet_length & 0xFF) as u8, //LSB
            packet_length.shr(8) as u8,   //MSB
            channel,
            self.sequence_numbers[channel as usize],
        ];
        self.sequence_numbers[channel as usize] += 1;

        self.send_buf[..PACKET_HEADER_LENGTH]
            .copy_from_slice(packet_header.as_ref());
        self.send_buf[PACKET_HEADER_LENGTH..packet_length]
            .copy_from_slice(body_data);
        
        self.interface.write_bytes(&self.send_buf[..packet_length]).map_err(|e| DriverError::CommError(e))
    }


    pub fn process_packet(&mut self, len: usize) -> Result<BnoPacket, DriverError<CE>> {
        let msg = &self.recv_buffer[..len];
        // println!("{:X?}", msg);
        let channel = msg[2];
        let report_id: u8 = if len > PACKET_HEADER_LENGTH {
            msg[4]
        } else {
            0
        };
        match channel {
            bno_constants::CHANNEL_COMMAND => match report_id {
                bno_constants::CMD_RESP_ADVERTISEMENT => Ok(BnoPacket::ChannelCommand(ChannelCommandData::AdvertiseResponse)),
                bno_constants::CMD_RESP_ERROR_LIST => Ok(BnoPacket::ChannelCommand(ChannelCommandData::ErrorList)),
                _ => Ok(BnoPacket::ChannelCommand(ChannelCommandData::Unknown(report_id)))
            },
            bno_constants::CHANNEL_EXECUTABLE => match report_id {
                bno_constants::EXECUTABLE_DEVICE_RESP_RESET_COMPLETE => Ok(BnoPacket::ChannelExec(ChannelExecutableData::ResetComplete)),
                _ => Ok(BnoPacket::ChannelExec(ChannelExecutableData::Unknown(report_id)))
            },
            bno_constants::CHANNEL_HUB_CONTROL => match report_id {
                bno_constants::SHUB_COMMAND_RESP => Ok(BnoPacket::ChannelHubControl(ChannelHubControlData::CommandResponse)),
                bno_constants::SHUB_PROD_ID_RESP => Ok(BnoPacket::ChannelHubControl(ChannelHubControlData::ProdIdResponse)),
                bno_constants::SHUB_GET_FEATURE_RESP => Ok(BnoPacket::ChannelHubControl(ChannelHubControlData::GetFeatureResponse)),

                _ => Ok(BnoPacket::ChannelHubControl(ChannelHubControlData::Unknown(report_id)))
            },
            bno_constants::CHANNEL_SENSOR_REPORTS => parse_sensor_reports(&msg).map_err(|e| DriverError::ParseError(e)),
            _ => {
                println!("unh chan 0x{:X} - {:?}", channel, msg);
                Err(DriverError::UnknownData)
            }
        }
    }


    pub fn receive_packet(&mut self) -> Result<BnoPacket, DriverError<CE>> {
        self.read_packet_header()?;
        let packet_len = parse_packet_header(
            &self.recv_buffer[..PACKET_HEADER_LENGTH],
        );
        if packet_len == 0 {
            return Err(DriverError::NoDataAvailable);
        }

        self.interface.read_bytes(&mut self.recv_buffer[..packet_len]).map_err(|e| DriverError::CommError(e))?;

        return self.process_packet(packet_len as usize);
    }
}