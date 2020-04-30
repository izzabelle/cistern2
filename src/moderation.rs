use chrono::prelude::*;
use serenity::client::Context;
use serenity::model::prelude::*;
use serenity::utils::Colour;

pub fn user_banned(ctx: Context, user: User, channel_id_u64: u64) {
    println!("user banned: {}#{}", user.name, user.discriminator);
    let channel_id = ChannelId::from(channel_id_u64);

    let msg = channel_id.send_message(&ctx.http, |m| {
        m.content("user banned");
        m.embed(|e| {
            e.title(":hammer: User Banned :hammer:");
            e.field("", format!("user: {}#{}", user.name, user.discriminator), true);
            e.color(Colour::from_rgb(255, 0, 0));
            e.timestamp(Local::now().to_rfc3339())
        })
    });
    crate::print_if_err(msg);
}
