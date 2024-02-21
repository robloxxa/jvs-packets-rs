A packet structures for [JAMMA Video Standart] protocols.

This crate provides a wrapper around `[u8]` array with getter and setter methods for easily changing/writing/reading data.

# Example
```rust
use jvs_packets::{jvs::{RequestPacket}, ReadPacket, Packet};

fn main() -> std::io::Result<()> {
    // This is only for example. You can use any structure, that implements std::io::Read. 
    let mut reader = std::io::Cursor::new([0xE0, 0xFF, 0x03, 0x01, 0x02, 0x05]);
    let mut req_packet: RequestPacket = RequestPacket::new();
    reader.read_packet(&mut req_packet);
    
    assert_eq!(req_packet.size(), 0x03);
    Ok(())
}
```

[JAMMA Video Standart]: https://en.wikipedia.org/wiki/Japan_Amusement_Machine_and_Marketing_Association#Video