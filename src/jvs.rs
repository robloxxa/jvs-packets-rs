//! A packet structures used for communication with JAMMA Video Standart.
use std::convert::{AsMut, AsRef};

use super::Packet;

#[derive(Debug, Clone)]
pub struct RequestPacket<const N: usize = 256> {
    inner: [u8; N],
}

impl<const N: usize> Packet for RequestPacket<N> {
    const DATA_BEGIN_INDEX: usize = 3;
    const SIZE_INDEX: usize = 2;
    const DESTINATION_INDEX: usize = 1;
}

impl<const N: usize> RequestPacket<N> {
    pub const fn new() -> Self {
        assert!(N > 4);
        Self { inner: [0; N] }
    }

    // pub fn from_reader(&mut self, reader: &mut impl Read) -> Result<(), Error> { Ok(()) }

    /// Initialize a struct from a slice.
    ///
    /// # Panics
    /// If the slice length is less than 4 and more than N.
    /// The slice can't be less than 4 because the packet is always has at least 4 bytes.
    pub fn from_slice(slice: &[u8]) -> Self {
        assert!(slice.len() > 4);
        let mut packet = Self::new();
        packet.inner[..slice.len()].copy_from_slice(slice);
        packet
    }
}

impl<const N: usize> AsRef<[u8]> for RequestPacket<N> {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

impl<const N: usize> AsMut<[u8]> for RequestPacket<N> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.inner
    }
}

impl<const N: usize> Default for RequestPacket<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// jvs response report codes.
#[derive(Debug, Clone)]
pub enum Report {
    /// Request was processed successfully.
    Normal = 1,
    /// Incorrect number of parameters were sent.
    IncorrectDataSize,
    /// Incorrect data was sent
    InvalidData,
    /// The device I/O is busy.
    Busy,
    /// Unknown report code.
    Unknown,
}

impl From<u8> for Report {
    fn from(value: u8) -> Self {
        match value {
            1 => Report::Normal,
            2 => Report::IncorrectDataSize,
            3 => Report::InvalidData,
            4 => Report::Busy,
            _ => Report::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResponsePacket<const N: usize = 256> {
    inner: [u8; N],
}

impl<const N: usize> Packet for ResponsePacket<N> {
    const DATA_BEGIN_INDEX: usize = 4;
    const SIZE_INDEX: usize = 2;
    const DESTINATION_INDEX: usize = 1;
}

impl<const N: usize> ResponsePacket<N> {
    const REPORT_INDEX: usize = 3;

    pub const fn new() -> Self {
        assert!(N > 4);
        Self { inner: [0; N] }
    }

    // pub fn from_reader(&mut self, reader: &mut impl Read) -> Result<(), Error> { Ok(()) }

    /// Initialize a struct from a slice.
    ///
    /// # Panics
    /// If the slice length is less than 4 and more than N.
    /// The slice can't be less than 4 because the packet is always has at least 4 bytes.
    pub fn from_slice(slice: &[u8]) -> Self {
        assert!(slice.len() > 4);
        let mut packet = Self::new();
        packet.inner[..slice.len()].copy_from_slice(slice);
        packet
    }

    pub fn report(&self) -> Report {
        self.inner[Self::REPORT_INDEX].into()
    }

    pub fn set_report(&mut self, report: impl Into<u8>) -> &mut Self {
        self.inner[Self::REPORT_INDEX] = report.into();
        self
    }
}

impl<const N: usize> AsRef<[u8]> for ResponsePacket<N> {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

impl<const N: usize> AsMut<[u8]> for ResponsePacket<N> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.inner
    }
}

impl<const N: usize> Default for ResponsePacket<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_request_packet_new_panic() {
        let _ = RequestPacket::<1>::new();
    }

    #[test]
    fn test_request_packet_from_slice() {
        let data = [0xE0, 0, 3, 1, 2, 6];
        let packet = RequestPacket::<256>::from_slice(&data);
        assert_eq!(&data, packet.as_slice());
    }

    #[test]
    #[should_panic]
    fn test_request_packet_from_slice_panic() {
        let data = [0, 1, 2];
        RequestPacket::<256>::from_slice(&data);
    }

    #[test]
    fn test_request_packet_access_methods() {
        let data = [0xE0, 0xFF, 3, 1, 2, 5];
        let packet = dbg!(RequestPacket::<256>::from_slice(&data));

        assert_eq!(packet.sync(), 0xE0);
        assert_eq!(packet.dest(), 0xFF);
        assert_eq!(packet.size(), 0x03);
        assert_eq!(packet.data(), &[0x01, 0x02]);
        assert_eq!(packet.checksum(), 0x05);
    }

    #[test]
    fn test_request_packet_setter_methods() {
        let mut packet = RequestPacket::<256>::new();
        packet
            .set_sync()
            .set_dest(0xFF)
            .set_size(0x03)
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
