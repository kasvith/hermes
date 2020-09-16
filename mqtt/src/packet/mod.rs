use std::io::{self, Read, Write};

#[repr(u8)]
pub(crate) enum PacketType {
    RESERVED = 0,
    CONNECT,
    CONNACK,
    PUBLISH,
    PUBACK,
    PUBREC,
    PUBREL,
    PUBCOMP,
    SUBSCRIBE,
    SUBACK,
    UNSUBSCRIBE,
    UNSUBACK,
    PINGREQ,
    PINGRESP,
    DISCONNECT,
    AUTH,
}

impl PacketType {
    pub(crate) fn get_type(&self) -> PacketType {
        match self.fixed_header >> 4 {
            0 => PacketType::RESERVED,
            1 => PacketType::CONNECT,
            2 => PacketType::CONNACK,
            3 => PacketType::PUBLISH,
            4 => PacketType::PUBACK,
            5 => PacketType::PUBREC,
            6 => PacketType::PUBREL,
            7 => PacketType::PUBCOMP,
            8 => PacketType::SUBSCRIBE,
            9 => PacketType::SUBACK,
            10 => PacketType::UNSUBSCRIBE,
            11 => PacketType::UNSUBACK,
            12 => PacketType::PINGREQ,
            13 => PacketType::PINGRESP,
            14 => PacketType::DISCONNECT,
            15 => PacketType::AUTH,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum MqttError {
    Io(io::Error),
}

pub trait Packet {
    fn parse<Reader: Read>(rdr: Reader) -> Result<(), MqttError>;
}
