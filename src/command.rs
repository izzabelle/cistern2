// command handler

// command modules
mod ping;
mod scale;

use serenity::model::prelude::Message;
use serenity::prelude::Context;
use std::collections::VecDeque;

// type def for args just to make things a little prettier
type Args = VecDeque<String>;

// wrap all of the command handling into a single message
pub fn handle(context: Context, message: Message) {
    Command::from(&message).execute(context);
}

pub struct Command {
    pub name: CommandName,
    pub args: Args,
    pub message: Message,
}

impl std::convert::From<&Message> for Command {
    // god this is fucking ~~awful~~ not half bad i should not be allowed to write code
    fn from(message: &Message) -> Self {
        let content = message.content.to_owned();
        let mut args: VecDeque<String> = content.split(" ").map(|arg| arg.to_string()).collect();
        let name = CommandName::from(args.pop_front());
        Self { args, name, message: message.clone() }
    }
}

impl Command {
    // execute a command
    fn execute(&self, context: Context) {
        match self.name {
            CommandName::Ping => ping::run(context, self),
            CommandName::VerseScale => scale::verse(context, self),
            CommandName::VoreScale => scale::vore(context, self),
            CommandName::BigScale => scale::big(context, self),
            CommandName::InvalidCommand => {}
        }
    }

    // returns command's args as a string
    fn args_as_string(&self) -> String {
        let mut string = String::new();
        self.args.iter().for_each(|arg| string = format!("{} {}", string, arg));
        string
    }
}

pub enum CommandName {
    Ping,
    InvalidCommand,
    VerseScale,
    VoreScale,
    BigScale,
}

impl std::convert::From<Option<String>> for CommandName {
    fn from(command_name: Option<String>) -> Self {
        let command_name = match command_name {
            Some(command_name) => command_name[1..].to_lowercase(),
            None => "bad data".to_string(),
        };

        match command_name.as_str() {
            "ping" => Self::Ping,
            "versescale" => Self::VerseScale,
            "vorescale" => Self::VoreScale,
            "bigscale" => Self::BigScale,
            _ => Self::InvalidCommand,
        }
    }
}
