//! A packet structures for slightly modified JAMMA Video Standart protocol,
//! that mostly used for NFS readers.
//!
//! # Request Packet (master -> slave)
//!  00     | 01  | 02     | 03    | 04    | 05       | 06       | ...          | N + 1                                                                                                                                                  |
//! :------:|:---:|:------:|:-----:|:-----:|:--------:|:--------:|:------------:|:-----:
//!  [SYNC] | `N` | `DEST` | `SEQ` | `CMD` | `DATA_0` | `DATA_1` | `DATA_(N-4)` | `SUM` 
//!  
//! # Response Packet (slave -> master)
//!  00     | 01  | 02     | 03    | 04       | 05    | 06       | 07       | 08       | ...          | N + 1                                                                                                                                                             |
//! :------:|:---:|:------:|:-----:|:--------:|:-----:|:--------:|:--------:|:--------:|:------------:|:-----:
//!  [SYNC] | `N` | `DEST` | `SEQ` | `STATUS` | `CMD` | [REPORT] | `DATA_0` | `DATA_1` | `DATA_(N-4)` | `SUM` 
//! 
//! [SYNC]: crate::SYNC_BYTE
//! [REPORT]: crate::Report


use crate::{impl_required_packet_blocks, Packet, ReportField};

pub trait ModifiedPacket: Packet {
    const CMD_INDEX: usize;
    const SEQUENCE_INDEX: usize;

    fn cmd(&self) -> u8 {
        self.as_ref()[Self::CMD_INDEX]
    }

    fn set_cmd(&mut self, cmd: u8) -> &mut Self {
        self.as_mut()[Self::CMD_INDEX] = cmd;
        self
    }

    fn sequence(&self) -> u8 {
        self.as_ref()[Self::SEQUENCE_INDEX]
    }

    fn set_sequence(&mut self, sequence: u8) -> &mut Self {
        self.as_mut()[Self::SEQUENCE_INDEX] = sequence;
        self
    }
}

#[derive(Debug, Clone)]
pub struct RequestPacket<const N: usize = 256> {
    inner: [u8; N],
}

impl<const N: usize> Packet for RequestPacket<N> {
    const DATA_BEGIN_INDEX: usize = 5;
    const SIZE_INDEX: usize = 1;
    const DESTINATION_INDEX: usize = 2;
}

impl<const N: usize> ModifiedPacket for RequestPacket<N> {
    const CMD_INDEX: usize = 4;
    const SEQUENCE_INDEX: usize = 3;
}

impl_required_packet_blocks!(RequestPacket);

#[derive(Debug, Clone)]
pub struct ResponsePacket<const N: usize = 256> {
    inner: [u8; N],
}

impl<const N: usize> Packet for ResponsePacket<N> {
    const DATA_BEGIN_INDEX: usize = 7;
    const SIZE_INDEX: usize = 1;
    const DESTINATION_INDEX: usize = 2;
}

impl<const N: usize> ModifiedPacket for ResponsePacket<N> {
    const CMD_INDEX: usize = 5;
    const SEQUENCE_INDEX: usize = 3;
}

impl<const N: usize> ReportField for ResponsePacket<N> {
    const REPORT_INDEX: usize = 6;
}

impl<const N: usize> ResponsePacket<N> {
    const STATUS_INDEX: usize = 4;

    pub fn status(&self) -> u8 {
        self.as_ref()[Self::STATUS_INDEX]
    }

    pub fn set_status(&mut self, status: u8) -> &mut Self {
        self.as_mut()[Self::STATUS_INDEX] = status;
        self
    }
}

impl_required_packet_blocks!(ResponsePacket);

#[cfg(test)]
mod tests {
    use super::*;

    const REQUEST_DATA: [u8; 8] = [0xE0, 0x06, 0xFF, 0x01, 0x02, 0x01, 0x02, 0x0B];
    const RESPONSE_DATA: [u8; 10] = [0xE0, 0x08, 0xFF, 0x01, 0x03, 0x02, 0x04, 0x01, 0x02, 0x14];

    // Request Packet tests
    #[test]
    fn test_request_packet_from_slice() {
        let packet = RequestPacket::<256>::from_slice(&REQUEST_DATA);
        assert_eq!(REQUEST_DATA, packet.as_slice());
    }

    #[test]
    fn test_request_packet_access_methods() {
        let packet = dbg!(RequestPacket::<256>::from_slice(&REQUEST_DATA));

        assert_eq!(packet.sync(), REQUEST_DATA[0]);
        assert_eq!(packet.size(), REQUEST_DATA[1]);
        assert_eq!(packet.dest(), REQUEST_DATA[2]);
        assert_eq!(packet.sequence(), REQUEST_DATA[3]);
        assert_eq!(packet.cmd(), REQUEST_DATA[4]);
        assert_eq!(packet.data(), &[REQUEST_DATA[5], REQUEST_DATA[6]]);
        assert_eq!(packet.checksum(), REQUEST_DATA[7]);
    }

    #[test]
    fn test_request_packet_setter_methods() {
        let mut packet = RequestPacket::<256>::new();
        packet
            .set_sync()
            .set_dest(REQUEST_DATA[2])
            .set_sequence(REQUEST_DATA[3])
            .set_cmd(REQUEST_DATA[4])
            .set_data(&[REQUEST_DATA[5], REQUEST_DATA[6]])
            .set_checksum(REQUEST_DATA[7])
            .set_size(REQUEST_DATA[1]);

        assert_eq!(packet.as_slice(), REQUEST_DATA);
        packet.calculate_checksum();
        assert_eq!(packet.checksum(), REQUEST_DATA[7]);
        packet.set_data(&[0x01]);
        assert_eq!(packet.size(), REQUEST_DATA[1] - 1);
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
        let packet = ResponsePacket::<256>::from_slice(&RESPONSE_DATA);

        assert_eq!(packet.sync(), RESPONSE_DATA[0]);
        assert_eq!(packet.size(), RESPONSE_DATA[1]);
        assert_eq!(packet.dest(), RESPONSE_DATA[2]);
        assert_eq!(packet.sequence(), RESPONSE_DATA[3]);
        assert_eq!(packet.status(), RESPONSE_DATA[4]);
        assert_eq!(packet.cmd(), RESPONSE_DATA[5]);
        assert_eq!(packet.report_raw(), RESPONSE_DATA[6]);
        assert_eq!(packet.data(), &[RESPONSE_DATA[7], RESPONSE_DATA[8]]);
        assert_eq!(packet.checksum(), RESPONSE_DATA[9]);
    }

    #[test]
    fn test_response_packet_setter_methods() {
        let mut packet = ResponsePacket::<256>::new();
        packet
            .set_sync()
            .set_dest(RESPONSE_DATA[2])
            .set_sequence(RESPONSE_DATA[3])
            .set_status(RESPONSE_DATA[4])
            .set_cmd(RESPONSE_DATA[5])
            .set_report(RESPONSE_DATA[6])
            .set_data(&[RESPONSE_DATA[7], RESPONSE_DATA[8]])
            .set_checksum(RESPONSE_DATA[9])
            .set_size(RESPONSE_DATA[1]);

        assert_eq!(packet.as_slice(), RESPONSE_DATA);
        packet.calculate_checksum();
        assert_eq!(packet.checksum(), RESPONSE_DATA[9]);
        packet.set_data(&[0x01]);
        assert_eq!(packet.size(), RESPONSE_DATA[1] - 1);
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

