mod answer;
mod chat;
mod chat_member;
mod commands;
mod game;
mod get_file;
mod message;
mod method;
mod passport;
mod poll;
mod send;
mod sticker;
mod updates;
mod user;

pub use self::{
    answer::*, chat::*, chat_member::*, commands::*, game::*, get_file::*, message::*, method::*, passport::*, poll::*,
    send::*, sticker::*, updates::*, user::*,
};
