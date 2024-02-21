# JVS-Packets &emsp; [![Latest Version]][crates.io] [![Docs Status]][docs.rs]

[Latest Version]: https://img.shields.io/crates/v/jvs-packets
[crates.io]: https://crates.io/crates/jvs-packets
[Docs Status]: https://img.shields.io/docsrs/jvs-packets/latest
[docs.rs]: https://docs.rs/jvs-packets
[LICENSE-MIT]: https://github.com/foresterre/cargo-msrv/blob/HEAD/LICENSE-MIT
[JAMMA Video Standart]: https://en.wikipedia.org/wiki/Japan_Amusement_Machine_and_Marketing_Association#Video

A packet structures for [JAMMA Video Standart] protocols.

This crate provides a wrapper around `[u8]` array with getter and setter methods for easily changing/writing/reading data.

---

# Example
```rust
use jvs_packets::{jvs::{RequestPacket}, ReadPacket, Packet};

fn main() -> std::io::Result<()> {
    // This is only for example. You can use any structure, that implements std::io::Read. 
    let mut reader = std::io::Cursor::new([0xE0, 0xFF, 0x03, 0x01, 0x02, 0x05]);
    let mut req_packet: RequestPacket = RequestPacket::new();
    reader.read_packet(&mut req_packet)?;
    
    assert_eq!(req_packet.size(), 0x03);
    Ok(())
}
```

#### License
<sub>
Licensed under either of Apache License, Version 2.0 or MIT license at your option.
</sub>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in JVS-packets-rs by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
</sub>
