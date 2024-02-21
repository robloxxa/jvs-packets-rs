//! A packet structures used for communication with JAMMA Video Standart.
//! 
//! # Request Packet (master -> slave)
//!  00     | 01     | 02  | 03       | 04        | ...          | N + 2 
//! :------:|:------:|:---:|:--------:|:---------:|:------------:|:-----:   
//!  [SYNC] | `DEST` | `N` | `DATA_0` | `DATA_1 ` | `DATA_(N-1)` | `SUM` 
//!  
//! # Response Packet (slave -> master)
//!  00     | 01     | 02  | 03       | 04        | 05       | ...          | N + 2 
//! :------:|:------:|:---:|:--------:|:---------:|:--------:|:------------:|:-----:   
//!  [SYNC] | `DEST` | `N` | [REPORT] | `DATA_0`  | `DATA_1` | `DATA_(N-1)` | `SUM` 
//! 
//! [SYNC]: crate::SYNC_BYTE
//! [REPORT]: crate::Report
use std::convert::{AsMut, AsRef};

use crate::{impl_required_packet_blocks, Packet, ReportField};

#[derive(Debug, Clone)]
pub struct RequestPacket<const N: usize = 256> {
    inner: [u8; N],
}

impl<const N: usize> Packet for RequestPacket<N> {
    const DATA_BEGIN_INDEX: usize = 3;
    const SIZE_INDEX: usize = 2;
    const DESTINATION_INDEX: usize = 1;
}

impl_required_packet_blocks!(RequestPacket);

#[derive(Debug, Clone)]
pub struct ResponsePacket<const N: usize = 256> {
    inner: [u8; N],
}

impl<const N: usize> Packet for ResponsePacket<N> {
    const DATA_BEGIN_INDEX: usize = 4;
    const SIZE_INDEX: usize = 2;
    const DESTINATION_INDEX: usize = 1;
}

impl<const N: usize> ReportField for ResponsePacket<N> {
    const REPORT_INDEX: usize = 3;
}

impl_required_packet_blocks!(ResponsePacket);


#[cfg(test)]
mod tests {
    use super::*;

    const REQUEST_DATA: [u8; 6] = [0xE0, 0xFF, 0x03, 0x01, 0x02, 0x05];
    const RESPONSE_DATA: [u8; 7] = [0xE0, 0xFF, 0x04, 0x01, 0x01, 0x02, 0x07];

    // Request Packet tests
    #[test]
    fn test_request_packet_from_slice() {
        let packet = RequestPacket::<256>::from_slice(&REQUEST_DATA);
        assert_eq!(REQUEST_DATA, packet.as_slice());
    }

    #[test]
    fn test_request_packet_access_methods() {
        let packet = RequestPacket::<256>::from_slice(&REQUEST_DATA);

        assert_eq!(packet.sync(), REQUEST_DATA[0]);
        assert_eq!(packet.dest(), REQUEST_DATA[1]);
        assert_eq!(packet.size(), REQUEST_DATA[2]);
        assert_eq!(packet.data(), &[REQUEST_DATA[3], REQUEST_DATA[4]]);
        assert_eq!(packet.checksum(), REQUEST_DATA[5]);
    }

    #[test]
    fn test_request_packet_setter_methods() {
        let mut packet = RequestPacket::<256>::new();
        packet
            .set_sync()
            .set_dest(REQUEST_DATA[1])
            .set_data(&[REQUEST_DATA[3], REQUEST_DATA[4]])
            .set_checksum(REQUEST_DATA[5])
            .set_size(REQUEST_DATA[2]);

        assert_eq!(packet.as_slice(), REQUEST_DATA);
        packet.calculate_checksum();
        assert_eq!(packet.checksum(), REQUEST_DATA[5]);
        packet.set_data(&[0x01]);
        assert_eq!(packet.size(), REQUEST_DATA[2] - 1);
    }

    #[test]
    fn test_request_packet_read() {
        use crate::ReadPacket;
        let mut cursor = std::io::Cursor::new(REQUEST_DATA);
        let mut packet = RequestPacket::<256>::new();
        cursor.read_packet(&mut packet).unwrap();

        assert_eq!(cursor.into_inner(), packet.as_slice())
    }

    #[test]
    fn test_request_packet_write() {
        use crate::WritePacket;
        let mut writer = std::io::Cursor::new(vec![]);
        let packet = RequestPacket::<256>::from_slice(&REQUEST_DATA);
        writer.write_packet_with_checksum(&packet).unwrap();

        assert_eq!(writer.into_inner(), packet.as_slice())
    }


    // Response Packet tests
    #[test]
    fn test_response_packet_from_slice() {
        let packet = ResponsePacket::<256>::from_slice(&REQUEST_DATA);
        assert_eq!(REQUEST_DATA, packet.as_slice());
    }

    // #[test]
    // #[should_panic]
    // fn test_response_packet_from_slice_panic() {
    //     let data = [0, 1, 2];
    //     ResponsePacket::<256>::from_slice(&data);
    // }

    #[test]
    fn test_response_packet_access_methods() {
        let packet = dbg!(ResponsePacket::<256>::from_slice(&RESPONSE_DATA));

        assert_eq!(packet.sync(), RESPONSE_DATA[0]);
        assert_eq!(packet.dest(), RESPONSE_DATA[1]);
        assert_eq!(packet.size(), RESPONSE_DATA[2]);
        assert_eq!(packet.report_raw(), RESPONSE_DATA[3]);
        assert_eq!(packet.data(), &[RESPONSE_DATA[4], RESPONSE_DATA[5]]);
        assert_eq!(packet.checksum(), RESPONSE_DATA[6]);
    }

    #[test]
    fn test_response_packet_setter_methods() {
        let mut packet = ResponsePacket::<256>::new();
        packet
            .set_sync()
            .set_dest(RESPONSE_DATA[1])
            .set_report(RESPONSE_DATA[3])
            .set_data(&[RESPONSE_DATA[4], RESPONSE_DATA[5]])
            .set_checksum(RESPONSE_DATA[6])
            .set_size(RESPONSE_DATA[2]);

        assert_eq!(packet.as_slice(), RESPONSE_DATA);
        packet.calculate_checksum();
        assert_eq!(packet.checksum(), RESPONSE_DATA[6]);
        packet.set_data(&[0x01]);
        assert_eq!(packet.size(), RESPONSE_DATA[2] - 1);
    }

    #[test]
    fn test_response_packet_read() {
        use crate::ReadPacket;
        let mut reader = std::io::Cursor::new(RESPONSE_DATA);
        let mut packet = ResponsePacket::<256>::new();
        reader.read_packet(&mut packet).unwrap();

        assert_eq!(reader.into_inner(), packet.as_slice())
    }

    #[test]
    fn test_response_packet_write() {
        use crate::WritePacket;
        let mut writer = std::io::Cursor::new(vec![]);
        let packet = ResponsePacket::<256>::from_slice(&RESPONSE_DATA);
        writer.write_packet_with_checksum(&packet).unwrap();

        assert_eq!(writer.into_inner(), packet.as_slice())
    }
}


