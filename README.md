# TGBOT

A full-featured Telegram Bot API client

[![CI](https://img.shields.io/github/workflow/status/tg-rs/tgbot/CI?style=flat-square)](https://github.com/tg-rs/tgbot/actions/)
[![Coverage](https://img.shields.io/codecov/c/github/tg-rs/tgbot.svg?style=flat-square)](https://codecov.io/gh/tg-rs/tgbot)
[![Version](https://img.shields.io/crates/v/tgbot.svg?style=flat-square)](https://crates.io/crates/tgbot)
[![Downloads](https://img.shields.io/crates/d/tgbot.svg?style=flat-square)](https://crates.io/crates/tgbot)
[![Release Documentation](https://img.shields.io/badge/docs-release-brightgreen.svg?style=flat-square)](https://docs.rs/tgbot)
[![Master Documentation](https://img.shields.io/badge/docs-master-blueviolet.svg?style=flat-square)](https://tg-rs.github.io/tgbot/tgbot/)
[![Telegram Chat](https://img.shields.io/badge/-@tgrsusers-blue?style=flat-square&logo=telegram)](https://t.me/tgrsusers)
[![License](https://img.shields.io/crates/l/tgbot.svg?style=flat-square)](https://github.com/tg-rs/tgbot/tree/0.12.1/LICENSE)

## Installation

```toml
[dependencies]
tgbot = "0.12.1"
```

## Example

Long polling:

```rust no_run
use std::env;
use tgbot::{Api, Config, UpdateHandler, async_trait};
use tgbot::longpoll::LongPoll;
use tgbot::methods::SendMessage;
use tgbot::types::{Update, UpdateKind};

struct Handler {
    api: Api,
}

#[async_trait]
impl UpdateHandler for Handler {
    async fn handle(&mut self, update: Update) {
        println!("got an update: {:?}\n", update);
        if let UpdateKind::Message(message) = update.kind {
            if let Some(text) = message.get_text() {
                let api = self.api.clone();
                let chat_id = message.get_chat_id();
                let method = SendMessage::new(chat_id, text.data.clone());
                api.execute(method).await.unwrap();
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("TGBOT_TOKEN").expect("TGBOT_TOKEN is not set");
    let api = Api::new(Config::new(token)).expect("Failed to create API");
    LongPoll::new(api.clone(), Handler { api }).run().await;
}
```

Webhook:

```rust no_run
use tgbot::{types::Update, async_trait, webhook, UpdateHandler};

struct Handler;

#[async_trait]
impl UpdateHandler for Handler {
    async fn handle(&mut self, update: Update) {
        println!("got an update: {:?}\n", update);
    }
}

#[tokio::main]
async fn main() {
    webhook::run_server(([127, 0, 0, 1], 8080), "/", Handler).await.unwrap();
}
```

See more examples in [examples](https://github.com/tg-rs/tgbot/tree/0.12.1/examples) directory.

In order to run an example you need to create a `.env` file:
```sh
cp sample.env .env
```
Don't forget to change value of `TGBOT_TOKEN` and other variables if required.

## Changelog

See [CHANGELOG.md](https://github.com/tg-rs/tgbot/tree/0.12.1/CHANGELOG.md)

## Code of Conduct

See [CODE_OF_CONDUCT.md](https://github.com/tg-rs/tgbot/tree/0.12.1/CODE_OF_CONDUCT.md).

## LICENSE

The MIT License (MIT)
