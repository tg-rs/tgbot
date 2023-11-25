# TGBOT

A full-featured Telegram Bot API client

[![CI](https://img.shields.io/github/actions/workflow/status/tg-rs/tgbot/ci.yml?branch=master&style=flat-square)](https://github.com/tg-rs/tgbot/actions/)
[![Coverage](https://img.shields.io/codecov/c/github/tg-rs/tgbot.svg?style=flat-square)](https://codecov.io/gh/tg-rs/tgbot)
[![Version](https://img.shields.io/crates/v/tgbot.svg?style=flat-square)](https://crates.io/crates/tgbot)

## Installation

```toml
[dependencies]
tgbot = "0.18.0"
```

## Example

Long polling:

```rust no_run
use futures_util::future::BoxFuture;
use std::env;
use tgbot::api::Client;
use tgbot::handler::{LongPoll, UpdateHandler};
use tgbot::types::{SendMessage, Update};

struct Handler {
    client: Client,
}

impl UpdateHandler for Handler {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        println!("got an update: {:?}\n", update);
        let client = self.client.clone();
        Box::pin(async move {
            let chat_id = update.get_chat_id();
            let message = update.get_message();
            let text = message.and_then(|x| x.get_text());
            if let (Some(chat_id), Some(text)) = (chat_id, text) {
                let method = SendMessage::new(chat_id, text.data.clone());
                client.execute(method).await.unwrap();
            }
        })
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("TGBOT_TOKEN").expect("TGBOT_TOKEN is not set");
    let client = Client::new(token).expect("Failed to create API");
    LongPoll::new(client.clone(), Handler { client }).run().await;
}
```

Webhook:

```rust no_run
use futures_util::future::BoxFuture;
use tgbot::{handler::{run_server, UpdateHandler}, types::Update};

struct Handler;

impl UpdateHandler for Handler {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        Box::pin(async move {
            println!("got an update: {:?}\n", update);
        })
    }
}

#[tokio::main]
async fn main() {
    run_server(([127, 0, 0, 1], 8080), "/", Handler).await.unwrap();
}
```

See more examples in the [examples](https://github.com/tg-rs/tgbot/tree/0.18.0/examples) directory.

To run an example you need to create a `.env` file:

```sh
cp sample.env .env
```

Don't forget to change the value of `TGBOT_TOKEN` and other variables if required.

## Versioning

This project adheres to [ZeroVer](https://0ver.org/)

## Links

- [Latest documentation](https://docs.rs/tgbot)
- [Master documentation](https://tg-rs.github.io/tgbot/tgbot/)
- [Telegram Chat](https://t.me/tgrsusers)
- [Changelog](https://github.com/tg-rs/tgbot/tree/0.18.0/CHANGELOG.md)
- [Code of Conduct](https://github.com/tg-rs/tgbot/tree/0.18.0/CODE_OF_CONDUCT.md)

## LICENSE

The MIT License (MIT)
