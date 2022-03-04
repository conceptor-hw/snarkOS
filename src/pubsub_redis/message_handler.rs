use crate::pubsub_redis::message::PubSubMessage;

pub fn handle(message: PubSubMessage) {
    println!(
        "subscribe: id{} channel {} desciption {} index:{}",
        message.id, message.channel, message.payload.description, message.payload.index,
    );
}
