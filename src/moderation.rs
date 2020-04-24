use serenity::client::Context;
use serenity::model::prelude::*;

pub fn user_banned(ctx: Context, user: User, mod_ids: (u64, u64)) {
    println!("user banned: {}#{}", user.name, user.discriminator);
}
