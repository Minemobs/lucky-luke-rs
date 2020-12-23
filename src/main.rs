mod blacklist;

use std::fs;
use serenity::{
    async_trait,
    Client,
    model::{channel::Message, gateway::Ready, prelude::Activity},
    prelude::{EventHandler, Context},
};
use blacklist::bl;

const PREFIX: char = ',';

#[tokio::main]
async fn main() {
    //Gets token from token file
    let token = fs::read_to_string("token")
        .expect("Unable to read token file.");

    //Builds client with token and handler
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Unable to create client.");

    //Starts client
    if let Err(err) = client.start().await {
        println!("{}", err);
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        //If message contains a blacklisted word
        if bl::is_blacklisted(&msg){
            //Treats it
            if let Err(err) = bl::treat(&ctx, &msg).await {
                println!("Error occured while treating blacklisted message :\n{}", err);
            }
        }
        else if msg.content.starts_with(PREFIX) {
            //Command received.
        }
    }

    async fn ready(&self, ctx: Context, _: Ready) {
        println!("Connected.");

        //Sets activity
        ctx.set_activity(Activity::listening("les skids afin de les balayer.")).await;
    }
}

