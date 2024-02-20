//! A packet structures for slightly modified JAMMA Video Standart protocol,
//! that mostly used for card readers.
//!
//! # Request Packet (master -> slave)
//! | 00     | 01  | 02     | 03    | 04    | 05       | 06       | ...           | N + 1                                                                                                                                                              |
//! |:------:|:---:|:------:|:-----:|:-----:|:--------:|:--------:|:------------:|:----:|
//! | `SYNC` | `N` | `DEST` | `SEQ` | `CMD` | `DATA_0` | `DATA_1` | `DATA_(N-4)` | `SUM` |
//!  
//! | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | ... |
//! |--------|----|----|----|----|----|----|----|----|
//! | `SYNC` | `n` | `dest` | `seq` | `cmd` | `data_0` | 
//!
//! # Response Packet (slave -> master)


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
