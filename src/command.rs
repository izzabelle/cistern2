// command handler

// command modules
mod ping;

// namespacing
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use std::collections::VecDeque;

// type def for args just to make things a little prettier
type Args = VecDeque<String>;

// wrap all of the command handling into a single message
pub fn handle(context: Context, message: Message) {
    Command::from(&message).execute(context);
}

struct Command {
    name: CommandName,
    args: Args,
    message: Message,
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
            CommandName::Ping => ping::run(context, &self.message),
            CommandName::InvalidCommand => {}
        }
    }
}

#[derive(Debug)]
enum CommandName {
    Ping,
    InvalidCommand,
}

impl std::convert::From<Option<String>> for CommandName {
    fn from(command_name: Option<String>) -> Self {
        let command_name = match command_name {
            Some(command_name) => command_name[1..].to_lowercase(),
            None => "bad data".to_string(),
        };

        match command_name.as_str() {
            "ping" => Self::Ping,
            _ => Self::InvalidCommand,
        }
    }
}
