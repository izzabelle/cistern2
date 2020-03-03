// modules
mod command;
mod config;

//  module namespacing
use config::*;

// std/crate namespacing
use serenity::client::{Client, EventHandler};
use serenity::model::gateway::Ready;
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use std::convert::TryFrom;
use structopt::StructOpt;

// lazy idiot result type
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// bot handler
struct Handler {
    api_token: String,
    prefix: String,
}

impl Handler {
    // intitialize the handler from args
    fn new(opt: Options) -> Result<Self> {
        let path = match opt.configuration_path {
            Some(path) => path,
            None => std::path::PathBuf::from("./bot_config.toml"),
        };

        let config = Config::try_from(path)?;

        let discord_api_token = if opt.use_production_token {
            config.production_token
        } else {
            config.development_token
        };

        Ok(Self { api_token: discord_api_token, prefix: config.command_prefix })
    }

    // check if a message contains a command
    fn is_command(&self, message: &Message) -> bool {
        message.content.as_bytes()[0] == self.prefix.as_bytes()[0]
    }
}

impl EventHandler for Handler {
    fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        println!(
            "{}#{} is online in {} servers!",
            data_about_bot.user.name,
            data_about_bot.user.discriminator,
            data_about_bot.guilds.len()
        );
    }

    fn message(&self, ctx: Context, message: Message) {
        // if bot ignore otherwise idk, the program logic is pretty straight forward
        if message.is_own(ctx.cache.to_owned()) {
            return;
        } else if self.is_command(&message) {
            println!("recieved command");
            command::handle(ctx, message);
        }
    }
}

// wrap everything so i can be lazy about error handling....
pub fn main_wrapper() -> Result<()> {
    let opt = config::Options::from_args();
    let handler = Handler::new(opt)?;
    let mut client = Client::new(handler.api_token.to_owned(), handler)?;

    if let Err(bruh_what_broke) = client.start() {
        println!("cistern has encountered an error: {:?}", bruh_what_broke);
    }

    Ok(())
}
