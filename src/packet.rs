use std::io::{self, Read};

/// A trait for all packets structures
pub trait Packet: AsRef<[u8]> + AsMut<[u8]> {
    const SIZE_INDEX: usize;
    const DATA_BEGIN_INDEX: usize;
    const DESTINATION_INDEX: usize;

    fn len_of_packet(&self) -> usize {
        Self::SIZE_INDEX + self.as_ref()[Self::SIZE_INDEX] as usize + 1
    }

    /// Returns a slice of the packet until SUM byte.
    fn as_slice(&self) -> &[u8] {
        &self.as_ref()[..self.len_of_packet()]
    }

    /// Returns a mutable slice of the packet until SUM byte.
    fn as_mut_slice(&mut self) -> &mut [u8] {
        let len = self.len_of_packet();
        &mut self.as_mut()[..len]
    }

    /// Returns a SIZE byte at [`Packet::SIZE_INDEX`]
    fn size(&self) -> u8 {
        self.as_ref()[Self::SIZE_INDEX]
    }

    /// Sets a size byte at [`Packet::SIZE_INDEX`].
    ///
    /// Don't use this method unless you know what you're doing. Use [`Packet::set_data`] instead.
    fn set_size(&mut self, size: u8) -> &mut Self {
        self.as_mut()[Self::SIZE_INDEX] = size;
        self
    }


    /// Returns a destination byte at [`Packet::DESTINATION_INDEX`].
    fn dest(&self) -> u8 {
        self.as_ref()[Self::DESTINATION_INDEX]
    }

    /// Sets a destination byte at [`Packet::DESTINATION_INDEX`] and calculates a new checksum.
    fn set_dest(&mut self, dest: u8) -> &mut Self {
        self.as_mut()[Self::DESTINATION_INDEX] = dest;
        self
    }

    /// Returns a slice of the packet data.
    fn data(&self) -> &[u8] {
        &self.as_ref()[Self::DATA_BEGIN_INDEX..self.len_of_packet() - 1]
    }

    /// Sets the packet data.
    ///
    /// This method will also set the size byte and calculate a new checksum.
    fn set_data(&mut self, data: &[u8]) -> &mut Self {
        let size = data.len() + Self::DATA_BEGIN_INDEX;
        self.as_mut()[Self::DATA_BEGIN_INDEX..size].copy_from_slice(data);
        self.set_size((size - Self::SIZE_INDEX) as u8);
        self
    }

    /// Calculates checksum.
    ///
    /// The checksum is calculated by summing all bytes except the SYNC (first byte).
    fn calculate_checksum(&mut self) -> &mut Self {
        self.set_checksum(
            self.as_slice()
                .iter()
                .skip(1)
                .take(self.len_of_packet() - 2)
                .fold(0, |acc: u8, &x| acc.wrapping_add(x)),
        );
        self
    }

    /// Returns a checksum.
    fn checksum(&self) -> u8 {
        self.as_ref()[self.len_of_packet()]
    }

    /// Sets a checksum in the end of the packet.
    ///
    /// Don't use this method unless you know what you're doing. Use [`Packet::calculate_checksum`] instead.
    fn set_checksum(&mut self, checksum: u8) -> &mut Self {
        let len = self.len_of_packet();
        self.as_mut()[len - 1] = checksum;
        self
    }
}

// TODO:
pub trait ReadPacket: Read {
    fn read_packet(&mut self, packet: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }
}

impl <R: Read> ReadPacket for R {}
