use std::io::{self, Read, Write};
/// SYNC byte indicates the beginning of the packet.
///
/// Readers should skip bytes until the SYNC byte is found.
pub const SYNC_BYTE: u8 = 0xE0;

/// MARK byte is used for escaping the [`SYNC_BYTE`] and [`MARK_BYTE`] bytes.
///
/// Since [`SYNC_BYTE`] is reserved for indicating the beggining of the packet,
/// it is escaped in the actual data by prepending [`MARK_BYTE`] and substructing one from the byte's value.
///
/// [`SYNC_BYTE`] and [`MARK_BYTE`] bytes are escaped as `D0 DF` and `D0 CF` respectively. Altough any bytes can be escaped, only these 2 bytes requried escaping.
pub const MARK_BYTE: u8 = 0xD0;

/// JVS response report codes.
/// 
/// When slave sending response to master, it will always contain a report code, which is placed before first DATA byte.
/// 
/// The Report byte indicates whether a request was completed succesfully.
/// 
/// Check variants documentation if you need to know what which code does.
#[derive(Debug, Clone)]
pub enum Report {
    /// Request was processed successfully.
    Normal = 1,
    /// Incorrect number of parameters were sent.
    IncorrectDataSize = 2,
    /// Incorrect data was sent
    InvalidData = 3,
    /// The device I/O is busy.
    Busy = 4,
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

    /// Returns a first byte in the slice.
    fn sync(&self) -> u8 {
        self.as_ref()[0]
    }

    fn set_sync(&mut self) -> &mut Self {
        self.as_mut()[0] = SYNC_BYTE;
        self
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
        self.as_ref()[self.len_of_packet() - 1]
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

/// A trait that add's additional setters for Response Packets.
///
/// All responses from jvs has report code that will indicate whether the request was processed successfully or not.
pub trait ReportField: Packet {
    const REPORT_INDEX: usize;

    /// Returns a report code.
    fn report(&self) -> Report {
        self.as_ref()[Self::REPORT_INDEX].into()
    }

    /// Sets a report code.
    fn set_report(&mut self, report: impl Into<u8>) -> &mut Self {
        self.as_mut()[Self::REPORT_INDEX] = report.into();
        self
    }
}

/// Additional methods for [`std::io::Read`] trait to read a single (escaped) byte.
pub trait ReadByteExt: Read {
    /// Reads a single byte.
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    /// Check if the first byte is [`MARK_BYTE`] and if it is, it will read the next byte and add one to it.
    fn read_u8_escaped(&mut self) -> io::Result<u8> {
        let mut b = self.read_u8()?;
        if b == MARK_BYTE {
            b = self.read_u8()?.wrapping_add(1);
        }
        Ok(b)
    }
}

impl<R: Read + ?Sized> ReadByteExt for R {}

/// Additional methods for [`std::io::Write`] trait to write a single byte.
pub trait WriteByteExt: Write {
    /// Writes a single byte.
    fn write_u8(&mut self, b: u8) -> io::Result<()> {
        self.write_all(&[b])
    }
    /// Will check if first byte is [`SYNC_BYTE`] or [`MARK_BYTE`] and if it is,
    /// it will write a byte value sub 1, followed by [`MARK_BYTE`].
    fn write_u8_escaped(&mut self, b: u8) -> io::Result<usize> {
        if b == SYNC_BYTE || b == MARK_BYTE {
            self.write_all(&[MARK_BYTE, b.wrapping_sub(1)])?;
            Ok(2)
        } else {
            self.write_all(&[b])?;
            Ok(1)
        }
    }
}

impl<W: Write + ?Sized> WriteByteExt for W {}

/// A helper trait which implemented for [`std::io::Read`]. Contains methods for reading [`Packet`]s from the Reader.
///
/// It is better to use [`std::io::BufReader`] to avoid unnecessary syscalls, since we have to read one byte at a time to check for escaped by [`MARK_BYTE`] bytes.
pub trait ReadPacket: Read {
    fn read_packet<P: Packet>(&mut self, packet: &mut P) -> io::Result<u8> {
        let sync = self.read_u8()?;

        if sync != SYNC_BYTE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Expected SYNC byte (0xE0), found: {:#04x}", sync),
            ));
        }
        let buf = packet.as_mut();
        buf[0] = sync;

        // Read to the SIZE byte first
        for b in &mut buf[1..=P::SIZE_INDEX] {
            *b = self.read_u8_escaped()?;
        }

        let len = buf[P::SIZE_INDEX] as usize + P::SIZE_INDEX;

        for b in &mut buf[P::SIZE_INDEX + 1..=len] {
            *b = self.read_u8_escaped()?;
        }

        Ok(packet.len_of_packet() as u8)
    }
}

impl<R: Read + ?Sized> ReadPacket for R {}

/// A helper trait which implemented for [`std::io::Write`]. Contains methods for writing [`Packet`]s to the Writer.
///
/// It is better to use [`std::io::BufWriter`] to avoid unnecessary syscalls, since we have to read one byte at a time to check for escaped by [`MARK_BYTE`] bytes.
pub trait WritePacket: Write {
    /// Writes a packet to the Writer.
    ///
    /// The function doesn't calculate checksum and instead writes whatever is present in the packet itself. So you have to use [`Packet::calculate_checksum`] before writing.
    /// Use [`Self::write_packet_with_checksum`] to calculate checksum while writing bytes.
    ///
    /// # Errors
    /// Will return [`Err`] if [`Packet::len_of_packet`] less than [`Packet::DATA_BEGIN_INDEX`] + 1 which is nonsense.
    fn write_packet<P: Packet>(&mut self, packet: &P) -> io::Result<usize> {
        if packet.len_of_packet() < P::DATA_BEGIN_INDEX + 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "The size of packet is can't be less than {}",
                    P::DATA_BEGIN_INDEX + 1
                ),
            ));
        }
        let mut bytes_written = 1;

        self.write_u8(SYNC_BYTE)?;

        for &b in &packet.as_slice()[1..] {
            bytes_written += self.write_u8_escaped(b)?;
        }

        Ok(bytes_written)
    }

    /// Similar to [`WritePacket::write_packet`], but it will calculate checksum while writing bytes to the writer.
    ///
    /// # Errors
    /// Will return [`Err`] if [`Packet::len_of_packet`] less than [`Packet::DATA_BEGIN_INDEX`] + 1 which is nonsense.
    fn write_packet_with_checksum<P: Packet>(&mut self, packet: &P) -> io::Result<usize> {
        if packet.len_of_packet() < P::DATA_BEGIN_INDEX + 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "The size of packet is can't be less than {}",
                    P::DATA_BEGIN_INDEX + 1
                ),
            ));
        }

        self.write_u8(SYNC_BYTE)?;
        let mut bytes_written: usize = 2;
        let mut checksum: u8 = 0;

        for &b in &packet.as_slice()[1..packet.len_of_packet()] {
            bytes_written += self.write_u8_escaped(b)?;
            checksum = checksum.wrapping_add(b);
        }

        self.write_u8_escaped(checksum)?;

        Ok(bytes_written)
    }
}
