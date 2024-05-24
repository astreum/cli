use std::{error::Error, time::SystemTime};

#[derive(Clone, Debug)]
pub enum Topic {
    Ping
}

impl Into<Vec<u8>> for Topic {
    fn into(self) -> Vec<u8> {
        match self {
            Topic::Ping => vec![0],
        }
    }
}


impl TryFrom<&[u8]> for Topic {
    type Error = Box<dyn Error>;
    fn try_from(value: &[u8]) -> Result<Self, Box<dyn Error>> {
        match value {
            [0] => Ok(Topic::Ping),
            _ => Err("topic decoding error!")?
        }
    }
}

#[derive(Clone, Debug)]
pub struct Message {
    pub body: Vec<u8>,
    pub topic: Topic
}

impl Message {
    pub fn new(topic: &Topic, body: &[u8]) -> Message {
        Message { body: body.to_vec(), topic: topic.clone() }
    }
}

impl Into<Vec<u8>> for &Message {
    fn into(self) -> Vec<u8> {
        let message_buffer_list: [Vec<u8>; 2] = [self.topic.clone().into(), self.body.clone()];
        astro_format::encode(message_buffer_list).unwrap()
    }
}

impl TryFrom<&[u8]> for Message {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let decoded_message: Vec<&[u8]> = astro_format::decode(value)?;
        if decoded_message.len() == 2 {
            Ok(Message::new(
                &(decoded_message[1].try_into()?),
                decoded_message[0],
            ))
        } else {
            Err("message decoding error!")?
        }
    }
}

#[derive(Clone, Debug)]
pub struct Ping {
    pub port: u16,
    pub pke_public_key: [u8; 32],
    pub validator: bool
}

impl Ping {
    pub fn new(port: &u16, pke_public_key: &[u8; 32], validator: &bool) -> Self {
        Ping {
            port: port.clone(),
            pke_public_key: pke_public_key.clone(),
            validator: validator.clone()
        }
    }
}

impl Into<Vec<u8>> for &Ping {
    fn into(self) -> Vec<u8> {
        let encoding_input = [
            &self.port.to_le_bytes()[..],
            &self.pke_public_key[..],
            if self.validator { &[1_u8][..] } else { &[0_u8][..] },
        ];
        astro_format::encode(encoding_input).unwrap()
    }
}

impl TryFrom<&[u8]> for Ping {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let ping_fields: Vec<&[u8]> = astro_format::decode(value)?;
        if ping_fields.len() >= 3 {
            let validator = match ping_fields[2] {
                [0] => false,
                [1] => true,
                _ => Err("ping decoding error: validator detail error!")?
            };
            Ok(Ping{
                port: u16::from_be_bytes(ping_fields[0].try_into()?),
                pke_public_key: ping_fields[1].try_into()?,
                validator,
            })
        } else {
            Err("ping decoding error: expected at least 3 fields!")?
        }
    }
}

#[derive(Clone, Debug)]
pub struct Peer {
    pub port: u16,
    pub pke_shared_key: [u8; 32],
    pub updated_at: u64
}

#[derive(Clone, Debug)]
pub struct Envelope {
    pub encrypted: bool,
    pub message: Vec<u8>,
    nonce: u64,
    time: u64
}

impl Envelope {
    
    pub fn new(encrypted: bool, message: Vec<u8>) -> Envelope {

        let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        
        let time_bytes: Vec<u8> = opis::Integer::from(&time).into();

        let encrypted_byte = if encrypted { vec![1] } else { vec![0] };

        let mut nonce = 0_u64;

        let mut message_tree = fides::structs::MerkleTree::new();
        message_tree.append(encrypted_byte);
        message_tree.append(message.clone());
        message_tree.append(nonce.to_le_bytes().to_vec());
        message_tree.append(time_bytes);
        
        let mut message_hash = message_tree.hash();
        
        while message_hash[0] != 0 {
            nonce += 1;
            message_tree.replace(2, nonce.to_le_bytes().to_vec());
            message_hash = message_tree.hash();
        }

        Envelope {
            encrypted,
            message,
            nonce,
            time
        }

    }

}

impl Into<Vec<u8>> for &Envelope {
    fn into(self) -> Vec<u8> {
        let envelope_field_bytes: [&[u8]; 4] = [
            if self.encrypted { &[1_u8] } else { &[0_u8] },
            &self.message[..],
            &self.nonce.to_le_bytes()[..],
            &self.time.to_le_bytes()[..],
        ];
        astro_format::encode(envelope_field_bytes).unwrap()
    }
}

impl TryFrom<&[u8]> for Envelope {
    type Error = Box<dyn Error>;
    fn try_from(value: &[u8]) -> Result<Self, Box<dyn Error>> {
        let envelope_fields: Vec<&[u8]> = astro_format::decode(value)?;
        if envelope_fields.len() >= 4 {
            // verify difficulty and time
            let result = Envelope {
                encrypted: if envelope_fields[0] == [1] { true } else { false },
                message: envelope_fields[1].to_vec(),
                nonce: u64::from_be_bytes(envelope_fields[2].try_into()?),
                time: u64::from_be_bytes(envelope_fields[3].try_into()?),
            };
            Ok(result)
        } else {
            Err("envelope fields error!")?
        }
    }
}