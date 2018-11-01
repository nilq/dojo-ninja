extern crate serenity;

use serenity::prelude::*;

use serenity::model::channel::Message;

use std::env;

struct DojoHandler;

impl EventHandler for DojoHandler {
  fn message(&self, _: Context, message: Message) {
    if &message.content[..6] == "!ninja" {
      match message.content[6..] {
        _ => if let Err(why) = message.channel_id.say("Hvad??") {
          println!("Error: {}", why)
        },
      }
    }
  }
}

fn main() {
  let token = env::var("DISCORD_TOKEN").expect("Expected token in environment!");

  let mut client = Client::new(&token, DojoHandler).expect("Error creating client");

  client.start();
}