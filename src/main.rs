use std::io::Cursor;
use sg_packets::{ReadPacket, Packet};
use sg_packets::jvs::RequestPacket;



fn main() {

    let mut cursor = Cursor::new([0xE0u8, 0x00, 0x03, 0x01, 0x02, 0x06]);
    let mut packet = RequestPacket::<256>::new();
    cursor.read_packet(&mut packet).unwrap();
    println!("{:?}", packet.as_slice());
}
