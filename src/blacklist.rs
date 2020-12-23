pub mod bl {
    use serenity::{
        model::channel::Message,
        prelude::{Context, SerenityError},
        utils::Color,
    };
    use std::fs;

    //Treats a blacklist message
    pub async fn treat(ctx: &Context, msg: &Message) -> Result<(), SerenityError> {
        //Deletes blacklist message
        msg.delete(&ctx.http).await?;

        //Shitty code that sends to message's author an embed
        //(I don't wanna clean this code, accept that lmao)
        match msg.author.dm(&ctx.http,
                            |message| {
                                message.embed(
                                    |embed| {
                                        embed.author(|author| {
                                            author.name("Lucky Luke - Le balayeur de skids.")
                                        });
                                        embed.title(format!("Hey {} !", msg.author.name));
                                        embed.description(format!("Il semblerait que tu sois un skid...\nJe t'ai entendu dire :\n```\n{}\n```\nEt mon devoir est de balayer ton message ! :wind_blowing_face:", msg.content));
                                        embed.thumbnail(format!("{}", match &msg.author.avatar_url() {
                                            Some(url) => url,
                                            None => "https://cdn.discordapp.com/embed/avatars/0.png",
                                        }));
                                        embed.footer(|footer| {
                                            footer.text("Ton message a été supprimé.")
                                        });
                                        embed.color(Color::from_rgb(220, 159, 66))
                                    })
                            }
        ).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    //Returns true if the message is blacklist, else it returns false.
    pub fn is_blacklisted(msg: &Message) -> Option<bool> {
        //Blacklist
        //Please help me to find words :'(
        let blacklisted_words = match fs::read_to_string("blacklist") {
            Ok(list) => list,
            Err(err) => {
                println!("Failed to read blacklist :\n{}", err);
                return None;
            }
        };

        //Gets message's content
        let content = msg.content.as_str();

        //Loop that returns true if message's content contains a blacklist word.
        for blacklisted_word in blacklisted_words.split_whitespace().into_iter() {
            if content.to_lowercase().contains(blacklisted_word) { return Some(true) }
        }

        Some(false)
    }
}