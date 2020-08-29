use bytes::{Buf, Bytes};
use std::convert::TryInto;
use std::io::Cursor;

#[repr(u8)]
pub enum PacketType {
    RESERVED = 0,
    CONNECT = 1,
    CONNACK = 2,
    PUBLISH = 3,
    PUBACK = 4,
    PUBREC = 5,
    PUBREL = 6,
    PUBCOMP = 7,
    SUBSCRIBE = 8,
    SUBACK = 9,
    UNSUBSCRIBE = 10,
    UNSUBACK = 11,
    PINGREQ = 12,
    PINGRESP = 13,
    DISCONNECT = 14,
    AUTH = 15,
}

impl PacketType {
    pub(crate) fn parse(b: u8) -> Option<PacketType> {
        let t = b >> 4;
        Some(PacketType::from(t))
    }
}

pub struct MqttPacket {
    pub packet_type: PacketType,
    flags: u8,
}

impl MqttPacket {
    pub(crate) fn parse(cur: &mut Cursor<&[u8]>) {
        // get type
        let header = cur.get_u8();
    }
}
