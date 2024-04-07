use crate::bno_constants::{MAX_CARGO_DATA_LENGTH, PACKET_HEADER_LENGTH};
use core::ops::Shl;
pub fn parse_packet_header(packet: &[u8]) -> usize {
    const CONTINUATION_FLAG_MASK: u16 = 0x80;
    const CONTINUATION_FLAG_CLEAR: u16 = !(CONTINUATION_FLAG_MASK);
    if packet.len() < PACKET_HEADER_LENGTH {
        return 0;
    }
    //Bits 14:0 are used to indicate the total number of bytes in the body plus header
    //maximum packet length is ... PACKET_HEADER_LENGTH
    let raw_pack_len: u16 = (packet[0] as u16)
        + ((packet[1] as u16) & CONTINUATION_FLAG_CLEAR).shl(8);

    let mut packet_len: usize = raw_pack_len as usize;
    if packet_len > MAX_CARGO_DATA_LENGTH {
        // we sometimes get garbage packets of [0xFF, 0xFF, 0xFF, 0xFF]
        packet_len = 0; //PACKET_HEADER_LENGTH;
    }

    if 0 == packet_len && 0 != raw_pack_len {
        #[cfg(feature = "rttdebug")]
        rprintln!(
            "pph: {:?} {} -> {}",
            &packet[..PACKET_HEADER_LENGTH],
            raw_pack_len,
            packet_len
        );
    } else {
        // hprintln!("pph: {:?} {} ", &packet[..PACKET_HEADER_LENGTH], packet_len).unwrap();
    }

    packet_len
}