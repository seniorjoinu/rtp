use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RepairPacket<'a> {
    #[serde(with = "serde_bytes")]
    message_id: &'a [u8],

    #[serde(with = "serde_bytes")]
    data: &'a [u8],

    message_size_bytes: u64,
    packet_size_bytes: u32,
    packet_id: u32,
}

#[derive(Serialize, Deserialize)]
struct RepairAckPacket<'a> {
    #[serde(with = "serde_bytes")]
    message_id: &'a [u8],

    packet_id: u32,
}

#[derive(Serialize, Deserialize)]
struct MessageAckPacket<'a> {
    #[serde(with = "serde_bytes")]
    message_id: &'a [u8],
}

#[cfg(test)]
mod tests {
    use crate::packet::MessageAckPacket;

    #[test]
    fn it_works() {
        let k = MessageAckPacket {
            message_id: &[0u8; 32],
        };

        let s = serde_cbor::to_vec(&k);
        assert!(s.is_ok());

        let d: MessageAckPacket = serde_cbor::from_slice(s.unwrap().as_slice()).unwrap();
    }
}
