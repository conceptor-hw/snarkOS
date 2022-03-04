use serde::{Deserialize, Serialize};
use uuid::Uuid;

//订阅发布redis message
#[derive(Debug, Serialize, Deserialize)]
pub struct PubSubMessage {
    pub id: String,
    pub channel: String,
    pub payload: Order,
}

impl PubSubMessage {
    pub fn new(payload: Order, channel: String) -> PubSubMessage {
        PubSubMessage {
            id: PubSubMessage::generate_id(),
            channel,
            payload,
        }
    }

    fn generate_id() -> String {
        Uuid::new_v4().to_simple().to_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub description: String,
    pub quantity: u64,
    pub index: i32,
}

impl Order {
    pub fn new(description: String, quantity: u64, index: i32) -> Order {
        Order {
            description,
            quantity,
            index,
        }
    }
}
