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

pub(crate) struct MqttPacket {
    fixed_header: u8,
    pub packet_type: PacketType,
}

impl MqttPacket {}
