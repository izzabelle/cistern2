// namespacing
use serenity::model::prelude::Message;
use serenity::prelude::Context;

pub fn run(context: Context, message: &Message) {
    let _ = message.channel_id.say(&context.http, "pong!");
}
