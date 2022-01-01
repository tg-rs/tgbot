mod animation;
mod audio;
mod callback_query;
mod chat;
mod commands;
mod contact;
mod dice;
mod document;
mod file;
mod game;
mod inline_query;
mod invoice;
mod location;
mod media_group;
mod message;
mod method;
mod passport;
mod photo;
mod poll;
mod pre_checkout_query;
mod shipping_query;
mod sticker;
mod updates;
mod user;
mod venue;
mod video;
mod video_note;
mod voice;
mod webhook;

pub use self::{
    animation::*, audio::*, callback_query::*, chat::*, commands::*, contact::*, dice::*, document::*, file::*,
    game::*, inline_query::*, invoice::*, location::*, media_group::*, message::*, method::*, passport::*, photo::*,
    poll::*, pre_checkout_query::*, shipping_query::*, sticker::*, updates::*, user::*, venue::*, video::*,
    video_note::*, voice::*, webhook::*,
};
