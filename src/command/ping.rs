use super::Command;
use serenity::prelude::Context;

pub fn run(context: Context, command: &Command) {
    let _ = command.message.channel_id.say(&context.http, "pong!");
}
