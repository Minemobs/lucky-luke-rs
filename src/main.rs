mod blacklist;

use std::fs;
use blacklist::bl;
use serenity::{
    async_trait,
    Client,
    model::{
        channel::Message,
        gateway::Ready,
        prelude::Activity
    },
    prelude::{EventHandler, Context},
};


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
        //Gets if message contains a blacklist word
        let is_blacklisted = bl::is_blacklisted(&msg);

        match is_blacklisted {
            Some(is_blacklisted) => {
                //If message has been written by a skid (contains blacklist words)
                if is_blacklisted {
                    //Treats skid's message
                    if let Err(err) = bl::treat(&ctx, &msg).await {
                        println!(
                            "Error occurred while treating blacklist message :\n{}",
                            err
                        )
                    }
                }
            },
            None => println!(
                "Failed to check if message has been written by a skid.\nMessage ID : {}",
                &msg.id
            ),
        }
    }

    async fn ready(&self, ctx: Context, _: Ready) {
        println!("Connected.");

        //Sets activity
        ctx.set_activity(Activity::listening("les skids afin de les balayer.")).await;
    }
}

