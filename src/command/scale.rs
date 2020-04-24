use super::Command;
use serenity::prelude::Context;

enum Verse {
    Top,
    TopLeaning,
    Verse,
    BottomLeaning,
    Bottom,
}

impl Verse {
    fn rand() -> Self {
        use rand::{thread_rng, Rng};
        match thread_rng().gen_range(0, 4) {
            0 => Self::Top,
            1 => Self::TopLeaning,
            2 => Self::Verse,
            3 => Self::BottomLeaning,
            4 => Self::Bottom,
            _ => unreachable!(),
        }
    }
}

pub fn verse(context: Context, command: &Command) {
    if command.args.len() < 1 {
        return;
    }

    let response = match Verse::rand() {
        Verse::Top => format!("{} is a top", &command.args_as_string()),
        Verse::TopLeaning => format!("{} is a top-leaning verse", &command.args_as_string()),
        Verse::Verse => format!("{} is a verse", &command.args_as_string()),
        Verse::BottomLeaning => format!("{} is a bottom-leaning verse", &command.args_as_string()),
        Verse::Bottom => format!("{} is a bottom", &command.args_as_string()),
    };

    let msg = command.message.channel_id.say(&context.http, response);
    crate::print_if_err(msg);
}

enum Vore {
    Pred,
    Prey,
    Vorse,
    Observer,
}

impl Vore {
    fn rand() -> Self {
        use rand::{thread_rng, Rng};
        match thread_rng().gen_range(0, 3) {
            0 => Self::Pred,
            1 => Self::Prey,
            2 => Self::Vorse,
            3 => Self::Observer,
            _ => unreachable!(),
        }
    }
}

pub fn vore(context: Context, command: &Command) {
    if command.args.len() < 1 {
        return;
    }

    let response = match Vore::rand() {
        Vore::Pred => format!("{} is a pred", &command.args_as_string()),
        Vore::Prey => format!("{} is prey", &command.args_as_string()),
        Vore::Vorse => format!("{} is vorse", &command.args_as_string()),
        Vore::Observer => format!("{} is an observer", &command.args_as_string()),
    };

    let msg = command.message.channel_id.say(&context.http, response);
    crate::print_if_err(msg);
}

enum Big {
    Macro,
    Normie,
    Micro,
}

impl Big {
    fn rand() -> Self {
        use rand::{thread_rng, Rng};
        match thread_rng().gen_range(0, 2) {
            0 => Self::Macro,
            1 => Self::Normie,
            2 => Self::Micro,
            _ => unreachable!(),
        }
    }
}

pub fn big(context: Context, command: &Command) {
    if command.args.len() < 1 {
        return;
    }

    let response = match Big::rand() {
        Big::Macro => format!("{} is a macro", &command.args_as_string()),
        Big::Normie => format!("{} is normal sized", &command.args_as_string()),
        Big::Micro => format!("{} is a micro", &command.args_as_string()),
    };

    let msg = command.message.channel_id.say(&context.http, response);
    crate::print_if_err(msg);
}
