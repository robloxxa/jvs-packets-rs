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

// TODO: Complete writing tests
#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // #[should_panic]
    // fn test_request_packet_new_panic() {
    //     let _ = RequestPacket::<1>::new();
    // }

    #[test]
    fn test_request_packet_from_slice() {
        let data = [0xE0, 6, 3, 1, 2, 1, 2, 14];
        let packet = RequestPacket::<256>::from_slice(&data);
        assert_eq!(&data, packet.as_slice());
    }

    // #[test]
    // #[should_panic]
    // fn test_request_packet_from_slice_panic() {
    //     let data = [0, 1, 2];
    //     RequestPacket::<256>::from_slice(&data);
    // }

    #[test]
    fn test_request_packet_access_methods() {
        let data = [0xE0, 6, 3, 1, 2, 1, 2, 14];
        let packet = dbg!(RequestPacket::<256>::from_slice(&data));

        assert_eq!(packet.sync(), data[0]);
        assert_eq!(packet.size(), data[1]);
        assert_eq!(packet.dest(), data[2]);
        assert_eq!(packet.sequence(), data[3]);
        assert_eq!(packet.cmd(), data[4]);
        assert_eq!(packet.data(), &[data[5], data[6]]);
        assert_eq!(packet.checksum(), data[7]);
    }

    #[test]
    fn test_request_packet_setter_methods() {
        let mut packet = RequestPacket::<256>::new();
        packet
            .set_sync()
            .set_dest(0xFF)
            .set_sequence(0x01)
            .set_cmd(0x02)
            .set_data(&[0x01, 0x02])
            
            .set_checksum(0x05);
        assert_eq!(packet.as_slice(), [0xE0, 0xFF, 0x03, 0x01, 0x02, 0x05]);
        packet.calculate_checksum();
        dbg!(&packet.as_slice());
        assert_eq!(packet.checksum(), 0x05)
    }

    #[test]
    fn test_request_packet_read() {
        use crate::ReadPacket;
        let mut data = std::io::Cursor::new([0xE0u8, 0, 3, 1, 2, 5]);
        let mut packet = RequestPacket::<256>::new();
        data.read_packet(&mut packet).unwrap();

        assert_eq!(&data.into_inner(), packet.as_slice())
    }
}

