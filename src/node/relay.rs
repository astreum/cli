use std::error::Error;

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

impl TryInto<Vec<u8>> for &Message {
    type Error = Box<dyn Error>;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let message_buffer_list: [Vec<u8>; 2] = [self.topic.clone().into(), self.body.clone()];
        let message_buffer = astro_format::encode(message_buffer_list)?;
        Ok(message_buffer)
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
            Err("Internal error!")?
        }
    }
}