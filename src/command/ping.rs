use super::Command;
use serenity::prelude::Context;

pub fn run(context: Context, command: &Command) {
    let msg = command.message.channel_id.say(&context.http, "pong!");
    crate::print_if_err(msg);
}
