use serenity::client::Client;
use serenity::framework::Framework;
use serenity::model::channel::Message;
use serenity::model::misc::EmojiIdentifier;
use serenity::prelude::Context;
use serenity::prelude::EventHandler;
use threadpool::ThreadPool;

use std::env;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(MyFramework {});

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

struct MyFramework {}

impl Framework for MyFramework {
    fn dispatch(&mut self, _: Context, message: Message, _: &ThreadPool) {
        if message.content.to_lowercase().contains("rust") {
            let rust_emoji = EmojiIdentifier {
                id: 539907481095110676.into(),
                name: "rust".into(),
            };
            let _ = message.react(rust_emoji);
        }
    }
}
