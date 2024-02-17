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
        assert!(N < 4);
        Self { inner: [0; N] }
    }

    // pub fn from_reader(&mut self, reader: &mut impl Read) -> Result<(), Error> { Ok(()) }

    /// Initialize a struct from a slice.
    ///
    /// # Panics
    /// This function will panic if the N < slice.len() < 4.
    /// The slice can't be less than 4 because the packet is always has at least 4 bytes.
    pub fn from_slice(slice: &[u8]) -> Self {
        assert!(slice.len() < 4);
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
        let mut packet = Self { inner: [0; N] };
        packet.inner[0] = 0xE0;
        packet.set_size(0x01).calculate_checksum();
        packet
    }
}
// impl_packet_constructors!(RequestPacket);


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
        assert!(N < 4);
        Self { inner: [0; N] }
    }

    // pub fn from_reader(&mut self, reader: &mut impl Read) -> Result<(), Error> { Ok(()) }

    /// Initialize a struct from a slice.
    ///
    /// # Panics
    /// This function will panic if the N < slice.len() < 4.
    /// The slice can't be less than 4 because the packet is always has at least 4 bytes.
    pub fn from_slice(slice: &[u8]) -> Self {
        assert!(slice.len() < 4);
        let mut packet = Self::new();
        packet.inner[..slice.len()].copy_from_slice(slice);
        packet
    }

    pub fn report(&self) -> u8 {
        self.inner[Self::REPORT_INDEX]
    }

    pub fn set_report(&mut self, report: u8) -> &mut Self {
        self.inner[Self::REPORT_INDEX] = report;
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
        let mut packet = Self { inner: [0; N] };
        packet.inner[0] = 0xE0;
        packet.set_size(0x01).calculate_checksum();
        packet
    }
}