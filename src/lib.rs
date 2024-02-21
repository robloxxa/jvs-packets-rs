//! A packet structures for [JAMMA Video Standart] protocols.
//! 
//! This crate provides a wrapper around `[u8]` array with getter and setter methods for easily changing/writing/reading data.
//! 
//! # Example
//! ```
//! use jvs_packets::{jvs::{RequestPacket}, ReadPacket, Packet};
//! 
//! # fn main() -> std::io::Result<()> {
//!     // This is only for example. You can use any structure, that implements std::io::Read. 
//!     let mut reader = std::io::Cursor::new([0xE0, 0xFF, 0x03, 0x01, 0x02, 0x05]);
//!     let mut req_packet: RequestPacket = RequestPacket::new();
//!     reader.read_packet(&mut req_packet);
//!     
//!     assert_eq!(req_packet.size(), 0x03);
//!     Ok(())
//! # }
//! ```
//! 
//! [JAMMA Video Standart]: https://en.wikipedia.org/wiki/Japan_Amusement_Machine_and_Marketing_Association#Video

mod packet;
pub use packet::{
    Packet, ReadByteExt, ReadPacket, Report, ReportField, WriteByteExt, WritePacket, MARK_BYTE, SYNC_BYTE,
};

#[cfg(feature = "jvs")]
pub mod jvs;

#[cfg(feature = "jvs_modified")]
pub mod jvs_modified;

#[cfg(any(feature = "jvs", feature = "jvs_modified"))]
#[macro_export]
macro_rules! impl_required_packet_blocks {
    ($t:tt) => {
        impl<const N: usize> $t<N> {
            pub const fn new() -> Self {
                Self { inner: [0; N] }
            }

            pub fn from_reader(reader: &mut impl crate::ReadPacket) -> std::io::Result<Self> {
                let mut packet = $t::new();
                reader.read_packet(&mut packet)?;

                Ok(packet)
            }

            /// Initialize a struct from a slice.
            ///
            /// # Panics
            /// If the slice length is less than 4 and more than N.
            /// The slice can't be less than 4 because the packet is always has at least 4 bytes.
            pub fn from_slice(slice: &[u8]) -> Self {
                let mut packet = Self::new();
                packet.inner[..slice.len()].copy_from_slice(slice);
                packet
            }
        }

        impl<const N: usize> AsRef<[u8]> for $t<N> {
            fn as_ref(&self) -> &[u8] {
                &self.inner
            }
        }

        impl<const N: usize> AsMut<[u8]> for $t<N> {
            fn as_mut(&mut self) -> &mut [u8] {
                &mut self.inner
            }
        }

        impl<const N: usize> Default for $t<N> {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}
