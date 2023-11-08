use std::{env, io::Cursor};

use dotenv::dotenv;
use futures_util::future::BoxFuture;

use tgbot::{
    api::Client,
    handler::{LongPoll, UpdateHandler},
    types::{
        mime,
        EditMessageMedia,
        InlineKeyboardButton,
        InputFile,
        InputFileReader,
        InputMedia,
        InputMediaAnimation,
        InputMediaPhoto,
        InputMediaVideo,
        MessageData,
        SendAnimation,
        SendDocument,
        SendPhoto,
        SendVideo,
        Update,
        UpdateType,
    },
};

#[derive(Clone)]
struct Handler {
    client: Client,
    gif_url: String,
    photo_path: String,
    video_path: String,
    document_thumb_path: String,
}

impl UpdateHandler for Handler {
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        let this = self.clone();
        Box::pin(async move {
            log::info!("got an update: {:?}\n", update);
            if let UpdateType::Message(message) = update.update_type {
                let chat_id = message.chat.get_id();
                if let Some(reply_to) = message.reply_to {
                    match reply_to.data {
                        // Change animation to document
                        MessageData::Animation(_) => {
                            let input_media = InputMedia::with_thumbnail(
                                InputFileReader::new(Cursor::new(b"Hello World!"))
                                    .with_file_name("hello.txt")
                                    .with_mime_type(mime::TEXT_PLAIN),
                                InputMediaAnimation::default().with_caption("test"),
                                InputFile::path(this.document_thumb_path).await.unwrap(),
                            )
                            .unwrap();
                            this.client
                                .execute(EditMessageMedia::for_chat_message(chat_id, reply_to.id, input_media))
                                .await
                                .unwrap();
                        }
                        // Change document to animation
                        MessageData::Document { .. } => {
                            this.client
                                .execute(EditMessageMedia::for_chat_message(
                                    chat_id,
                                    reply_to.id,
                                    InputMedia::new(
                                        InputFile::url(this.gif_url),
                                        InputMediaAnimation::default().with_caption("test"),
                                    )
                                    .unwrap(),
                                ))
                                .await
                                .unwrap();
                        }
                        // Change photo to video
                        MessageData::Photo { .. } => {
                            let input_media = InputMedia::new(
                                InputFile::path(this.video_path).await.unwrap(),
                                InputMediaVideo::default(),
                            )
                            .unwrap();
                            this.client
                                .execute(EditMessageMedia::for_chat_message(chat_id, reply_to.id, input_media))
                                .await
                                .unwrap();
                        }
                        // Change video to photo
                        MessageData::Video { .. } => {
                            let input_media = InputMedia::new(
                                InputFile::path(this.photo_path).await.unwrap(),
                                InputMediaPhoto::default(),
                            )
                            .unwrap();
                            this.client
                                .execute(EditMessageMedia::for_chat_message(chat_id, reply_to.id, input_media))
                                .await
                                .unwrap();
                        }
                        _ => {}
                    }
                } else if let MessageData::Document(document) = message.data {
                    // Resend document by file id (you also can send a document using URL)
                    this.client
                        .execute(SendDocument::new(chat_id, InputFile::file_id(document.data.file_id)))
                        .await
                        .unwrap();
                } else if let Some(text) = message.get_text() {
                    match text.data.as_str() {
                        // Send animation by URL (you also can send animation using a file_id)
                        "/gif" => {
                            let method = SendAnimation::new(InputFile::url(this.gif_url), chat_id);
                            this.client.execute(method).await.unwrap();
                        }
                        "/photo" => {
                            let markup = [[InlineKeyboardButton::for_callback_data("test", "cb-data")]];
                            let method = SendPhoto::new(chat_id, InputFile::path(this.photo_path).await.unwrap())
                                .with_reply_markup(markup)
                                .unwrap();
                            this.client.execute(method).await.unwrap();
                        }
                        "/text" => {
                            let document = Cursor::new(b"Hello World!");
                            let reader = InputFileReader::new(document)
                                .with_file_name("hello.txt")
                                .with_mime_type(mime::TEXT_PLAIN);
                            let method = SendDocument::new(chat_id, reader)
                                .with_thumbnail(InputFile::path(this.document_thumb_path).await.unwrap())
                                .unwrap();
                            this.client.execute(method).await.unwrap();
                        }
                        "/video" => {
                            let method = SendVideo::new(chat_id, InputFile::path(this.video_path).await.unwrap());
                            this.client.execute(method).await.unwrap();
                        }
                        // The same way for other file types...
                        _ => {}
                    };
                }
            }
        })
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGBOT_TOKEN").expect("TGBOT_TOKEN is not set");
    let gif_url = env::var("TGBOT_GIF_URL").expect("TGBOT_GIF_URL is not set");
    let photo_path = env::var("TGBOT_PHOTO_PATH").expect("TGBOT_PHOTO_PATH is not set");
    let video_path = env::var("TGBOT_VIDEO_PATH").expect("TGBOT_VIDEO_PATH is not set");
    let document_thumb_path = env::var("TGBOT_DOCUMENT_THUMB_PATH").expect("TGBOT_DOCUMENT_THUMB_PATH is not set");
    let client = Client::new(token).expect("Failed to create API");
    LongPoll::new(
        client.clone(),
        Handler {
            client,
            gif_url,
            photo_path,
            video_path,
            document_thumb_path,
        },
    )
    .run()
    .await;
}
