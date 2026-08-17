# Changelog

## 0.47.0 (17.08.2026)

- Added Bot API 10.2 support.
- Added `Client::with_max_retry_after` method.
- `Client::download_file` now logs a file path only.
- Changed serialization of integer form values from `ToString` to itoa.
- Changed serialization of floating form values from `ToString` to zmij.
- Changed the type of the `certificate` parameter in `SetWebhook` to `InputFileReader`-like.
- Fixed corrupted `attach://%id%` form values in poll media.
- Fixed the missing `can_delete_sent_messages` field in `BusinessBotRights`.
- Fixed the missing `subscription_period` and `subscription_price` fields in `ChatInviteLink`.
- Fixed inconsistent serialization of `MaybeInaccessibleMessage`.
- Fixed `PollAnswerVoter` (de)serialization.
- Fixed missing `TryFrom<Update>` implementations.
- Fixed inaccessible `LivePhoto` struct fields.
- Fixed inaccessible `Gifts.gifts` field.
- Fixed inaccessible `PreparedKeyboardButton` fields.
- Fixed inaccessible `StarTransaction` fields.
- Moved form serialization to `Method::into_payload`:
  The `Result<Self, %SerializationError%>` return type of methods in form-based structs changed to `Self`.
- Converted `InputMessageContent` to a struct.
- Converted `InlineQueryResult` to a struct.
- Removed `InputMedia` factory methods; use `From`/`Into` instead.
- Removed `InputMessageContentLocation`; use `Location::into` instead.
- Removed `InputMessageContentRich`; use `InputRichMessage::into` instead.
- Removed `InputPaidMediaGroupItem`; use corresponding `InputPaidMedia` types instead.
- Removed `MediaGroupItem` factory methods; use `From`/`Into` instead.
- Removed `GetUpdates::add_allowed_update`.
- Removed `with_*_entities` and `with_*parse_mode` methods in favor of `InputText` and `InputTextFormat`.
- Removed constructors and setters from types with public fields.
  These types are used in responses and rarely instantiated.

## 0.46.0 (13.06.2026)

- Added Bot API 10.1 support.

## 0.45.0 (18.05.2026)

- Added Bot API 10.0 support.
- Added `webpki-roots` feature [#30](https://github.com/tg-rs/tgbot/pull/30).

## 0.44.0 (08.04.2026)

- Added Bot API 9.6 support.

## 0.43.0 (02.03.2026)

- Added Bot API 9.5 support.
- Reduced size of large enums.

## 0.42.0 (09.02.2026)

- Added Bot API 9.4 support.
- Added `UpdateHandler` trait implementation for `Arc<T: UpdateHandler>`.
- Added `LongPollOptions::with_concurrency_limit` to limit the number of spawned tasks.

## 0.41.0 (02.01.2026)

- Added Bot API 9.3 support.
- Updated reqwest to 0.13.


## 0.40.0 (16.08.2025)

- Added Bot API 9.2 support.

## 0.39.1 (08.07.2025)

- Changed internal RawResponse type to struct to avoid obscure deserialization errors.
- Fixed custom emoji reaction type deserialization.

## 0.39.0 (05.07.2025)

- Added Bot API 9.1 support.
- Set bytes version to 1.
- Set derive_more version to 2.
- Set mime_guess version to 2.
- Set serde version to 1.
- Set serde_json version to 1.
- Set serde_with version to 3.
- Set shellwords version to 1.
- Set tokio version to 1.

## 0.38.0 (17.06.2025)

- Fixed parsing of `ChatFullInfo.photo` field.

## 0.37.0 (17.06.2025)

- Changed type of `Client::with_max_retries` argument to `u8`.
- Removed `ExecuteError::TooManyRequests` enum variant, use `ResponseError::retry_after()` method instead.
- Fixed: always sleep before sending a request in `Client::execute` method.

## 0.36.2 (17.06.2025)

- Added `with_max_retries` method.

## 0.36.1 (05.05.2025)

- Fixed incorrect response type for `GetBusinessConnection`.

## 0.36.0 (12.04.2025)

- Added Bot API 9.0 support.
- Updated tokio to 1.44.

## 0.35.0 (12.02.2025)

- Added Bot API 8.3 support.
- Added serde_with at 3.12.
- Updated bytes to 1.10.
- Updated derive_more to 2.0.
- Updated tokio to 1.43.

## 0.34.0 (02.01.2025)

- Added Bot API 8.2 support.
- Updated axum to 0.8.

## 0.33.0 (04.12.2024)

- Added Bot API 8.1 support.
- Updated bytes to 1.9.
- Updated tokio to 1.42.

## 0.32.0 (17.11.2024)

- Added Bot API 8.0 support.

## 0.31.0 (01.11.2024)

- Added Bot API 7.11 support.
- Updated bytes to 1.8.
- Updated tokio to 1.41.

## 0.30.0 (07.09.2024)

- Added Bot API 7.10 support.
- Updated tokio to 1.40.

## 0.29.0 (18.08.2024)

- Added Bot API 7.9 support.
- Updated derive_more to 1.0.

## 0.28.0 (31.07.2024)

- Added Bot API 7.8 support.
- Updated tokio to 1.39.
- Updated bytes to 1.7.

## 0.27.0 (07.07.2024)

- Added Bot API 7.7 support.

## 0.26.0 (02.07.2024)

- Added Bot API 7.6 support.

## 0.25.0 (18.06.2024)

- Added Bot API 7.5 support.
- Updated tokio to 1.38.

## 0.24.0 (29.05.2024)

- Added Bot API 7.4 support.

## 0.23.0 (07.05.2024)

- Added Bot API 7.3 support.

## 0.22.0 (01.04.2024)

- Added Bot API 7.2 support.
- Updated bytes to 1.6.
- Updated reqwest to 0.12.
- Updated tokio to 1.37.

## 0.21.0 (18.02.2024)

- Added Bot API 7.1 support.
- Updated tokio to 1.36.

## 0.20.0 (01.01.2024)

- Added Bot API 7.0 support.
- Updated tokio to 1.35.
- `Client::with_client` method renamed to `Client::with_http_client`.
- Removed `Future` associated type from `UpdateHandler` trait.

## 0.19.0 (05.12.2023)

- Added Bot API 6.9 support.
- Updated tokio to 1.34.
- Updated bytes to 1.5.
- Removed vec1.
- `Api` struct renamed to `api::Client`.
- `ApiError`, `DownloadFileError`, `ExecuteError` moved to `api` module.
- Merged `UpdateHandler`, `longpoll` and `webhook` into `handler` module.
- Webhooks:
  - Migrated from hyper to axum.
  - `webhook::run_server` and `webhook::SyncedUpdateHandler` replaced by `handler::WebhookServer` type.
  - Webhooks support is disabled by default and can be enabled using `webhook` feature.
- Long polling:
  - Added `#[must_use]` attribute to `LongPoll::get_handle`.
  - Renamed `LongPoll::options` to `LongPoll::with_options`.
  - Renamed methods of `LongPollOptions`: `allowed_update` to `with_allowed_update`,
    `error_timeout` to `with_error_timeout`, `limit` to `with_limit`,
    `poll_timeout` to `with_poll_timeout`.
- Merged `methods` module into `types` module.
- `serde::Deserialize` and `serde::Serialize` are implemented for all types.
- Moved `mime` reexport to `types` module.
- `Vec<TextEntity>` and `Vec1<TextEntity>` replaced with a new `TextEntities` struct.
- Fixed:
  - Use different type parameters for strings in `PassportElementError` factory methods and `SendContact::new` method.
  - Added missing variants to the `AllowedUpdate` enum: `BotStatus`, `ChatJoinRequest`.

## 0.18.0 (10.02.2022)

- Added Bot API 5.7 support.
- Updated tokio to 1.16.
- `Update::get_user_username()` now returns `Option<&str>` instead of `Option<String>`.
- `Update::get_message()` now includes a message from `CallbackQuery`.
- Added `Message::get_user_id()` and `Message::get_user_username()` methods.

## 0.17.0 (02.02.2022)

- Removed `Config`.
- Added `Api::with_client` method.
- Spawn a new task for each incoming update in `LongPoll`.
- Retry a request on timeout error.
- Added `message_auto_delete_time` field to chat-related structs.

## 0.16.0 (01.01.2022)

- Added Bot API 5.6 support.
- Rust 2021 edition.
- Added `Update::get_user_id` and `Update::get_user_username` methods.

## 0.15.0 (27.12.2021)

- Added Bot API 5.5 support.
- Updated tokio to 1.15.

## 0.14.0 (06.09.2021)

- Added Bot API 5.3 support.
- Fixed parsing of command arguments when bot name is a part of the command.

## 0.13.0 (14.03.2021)

- Added Bot API 5.1 support.
- Updated async-stream to 0.3.
- Updated tokio to 1.3.
- New `UpdateHandler` API.
  - Removed `async_trait` dependency.
  - `handle` method now takes an immutable reference to `self`.
  - Added `SyncedUpdateHandler` wrapper for non-sync handlers.
- New `TextEntity` API.
  - Use `Vec1` instead of `Vec` to guarantee that entities are not empty.
  - `TextEntityPosition` now contains `u32` values instead of `i64`.
  - You can convert ranges into `TextEntityPosition`.
- Added `Text::as_ref()` method to get data as `&str`.
- Implemented `String` to `Text` conversion.
- Added `UpdateKind::Unknown` for updates introduced in new Bot API versions.
- Added `Chat::get_id()` and `Chat::get_username()` methods.
- Fixed error timeout in longpoll.

## 0.12.1 (11.01.2021)

- Fixed a panic when getting a list of bot commands.

## 0.12.0 (05.01.2021)

- Tokio 1.0 support.

## 0.11.0 (15.11.2020)

- Added Bot API 5.0 support.

## 0.10.0 (20.06.2020)

- Added Bot API 4.9 support.
- Fixed a bug with proxy without username/password.

## 0.9.0 (26.04.2020)

- Added Bot API 4.8 support
- Public access to InlineKeyboard data.

## 0.8.0 (01.04.2020)

- Added Bot API 4.7 support
- Renamed message entity structs:
  * `BotCommand` -> `TextEntityBotCommand`
  * `TextLink` -> `TextEntityLink`
  * `TextMention` -> `TextEntityMention`

## 0.7.1 (16.03.2020)

- Impl `Eq` for `ChatId` and `UserId`.

## 0.7.0 (08.03.2020)

- `ChatId` type implements `Display` trait instead of `ToString`.
-  Added `Display` and `Serialize` trait implementations for `UserId` type.
- `ResponseError` fields are private now and `ResponseParameters` struct was removed,
   use corresponding methods instead.
- Added `Copy`, `Hash`, `PartialEq` and `PartialOrd` trait implementations for several types.
- Reduced memory usage when sending files.
  `InputFileReader` now takes `tokio::io::AsyncRead` instead of `std::io::Read`.
  `InputFile::path` is async and returns a `tokio::io::Result`.
- `download_file` method now returns a stream of bytes.
- Added `Command` type.

## 0.6.0 (27.01.2020)

- Added Bot API 4.6 support.
- Added `ResponseError::can_retry()` method.
- Added `ParseMode::escape()` method.
- Added `get_full_name`, `get_link` and `get_mention` methods to `User` type.
- Removed deprecated `RestrictChatMember` methods.

## 0.5.1 (10.01.2020)

- Return socks proxy support.
- Deprecate some `RestrictChatMember` methods (to be removed in 0.6.0).

## 0.5.0 (07.01.2020)

- Added Bot API 4.5 support.
- Added async/await support.
- Added rustls support.
- Fixed some errors when parsing update.
- Temporarily disabled socks proxy support.

## 0.4.4 (31.07.2019)

- Added Bot API 4.4 support.
- Added information about response data to poll request error.
- `RestrictChatMember` methods:
  `can_send_messages`,
  `can_send_media_messages`,
  `can_send_other_messages`,
  `can_add_web_page_previews` are now deprecated and will be removed in 0.5.0.
  Use `with_permissions` method instead.

## 0.4.3 (07.07.2019)

- Ignore invalid updates in webhook handler.

## 0.4.2 (01.06.2019)

- Added Bot API 4.3 support.

## 0.4.1 (28.05.2019)

- Fixed ChatMemberAdministrator parsing.

## 0.4.0 (07.05.2019)

- Added Bot API 4.2 support.
- Added `prelude` module.
- Added `ChatMember::is_member()` method.
- Added `UpdatesStream::should_retry()` method.
- Added `Api::download_file()` method.
- Added option to set custom base url for Telegram API (see `Config` struct).
- Added multipart support (you can upload files using multipart/form-data now).
- Added `UserId` type.
- You can (de)serialize callback data using `serde_json` (see `CallbackQuery` and `InlineKeyboardButton` types).
- `Api::new()` now takes a `Config` object.
- `Api::execute()` takes a method by value instead of reference.
- `Method::get_request(&self)` replaced by `Method::into_request(self)`
- `RequestBuilder` is now private.
- `WebhookServiceFactory` is now private.
- Removed use of `Mutex` in `WebhookService`.
- Removed `WebhookServiceFactoryError`.
- Removed `InlineKeyboardButton::new()` method as the user must use exactly one of the option fields.

## 0.3.0 (12.03.2019)

- Dispatcher moved to [carapax](https://github.com/tg-rs/carapax).
- Added `Update::get_chat_id()`, `Update::get_chat_username()` and `Update::get_user()` methods.
- Added `Message::is_edited()` and `Message::get_chat_username()` methods.
- Added `Message.commands` property.
- Added `UpdatesStreamOptions`.
- Removed `Api::create()` and `Api::with_proxy()` in favor of `Api::new()`.
- Removed `Api::get_updates()`, use `tgbot::handle_updates()` instead.
- `WebhookService` is public now.
- Respect `retry_after` parameter on polling error.

## 0.2.0 (27.02.2019)

- Migrated from curl to hyper.
- Added dispatcher.
- Added webhooks support.

## 0.1.0 (23.12.2018)

- First release.
