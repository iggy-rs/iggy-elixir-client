// use iggy::messages::send_messages::Message as RustSendMessage;
use std::str::FromStr;

pub struct SendMessage {
    pub(crate) inner: RustSendMessage,
}

impl Clone for SendMessage {
    fn clone(&self) -> Self {
        Self {
            inner: RustSendMessage::from_str(&self.inner.to_string()).unwrap(),
        }
    }
}

impl SendMessage {
    pub fn new(data: String) -> Self {
        // TODO: handle errors
        let inner = RustSendMessage::from_str(&data).unwrap();
        Self { inner }
    }
}
