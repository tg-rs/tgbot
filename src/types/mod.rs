mod animation;
mod audio;
mod bot_command;
mod callback_query;
mod chat;
mod command;
mod contact;
mod dice;
mod document;
mod file;
mod game;
mod inline_mode;
mod input_media;
mod location;
mod login_url;
mod media_group;
mod message;
mod parse_mode;
mod passport;
mod payments;
mod photo_size;
mod poll;
mod primitive;
mod reply_markup;
mod response;
mod stickers;
mod text;
mod update;
mod user;
mod venue;
mod video;
mod video_note;
mod voice;

pub use self::{
    animation::*, audio::*, bot_command::*, callback_query::*, chat::*, command::*, contact::*, dice::*, document::*,
    file::*, game::*, inline_mode::*, input_media::*, location::*, login_url::*, media_group::*, message::*,
    parse_mode::*, passport::*, payments::*, photo_size::*, poll::*, primitive::*, reply_markup::*, response::*,
    stickers::*, text::*, update::*, user::*, venue::*, video::*, video_note::*, voice::*,
};
