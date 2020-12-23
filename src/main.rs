use std::fs;
use serenity::{
    async_trait,
    Client,
    model::{channel::Message, gateway::Ready},
    prelude::{EventHandler, Context},
};

const PREFIX: char = ',';

#[tokio::main]
async fn main() {
    let token = fs::read_to_string("token")
        .expect("Unable to read token file.");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Unable to create client.");

    if let Err(err) = client.start().await {
        println!("CE : {:?}", err);
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _: Context, msg: Message) {
        if msg.content.starts_with(PREFIX) {
            //Command received.
        }
    }

    async fn ready(&self, _: Context, _: Ready) {
        println!("Connected.");
    }
}

