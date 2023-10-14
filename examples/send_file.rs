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
        UpdateKind,
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
            if let UpdateKind::Message(message) = update.kind {
                let chat_id = message.chat.get_id();
                if let Some(reply_to) = message.reply_to {
                    match reply_to.data {
                        // Change animation to document
                        MessageData::Animation(_) => {
                            let input_media = InputMedia::with_thumb(
                                InputFileReader::new(Cursor::new(b"Hello World!"))
                                    .info(("hello.txt", mime::TEXT_PLAIN)),
                                InputFile::path(this.document_thumb_path).await.unwrap(),
                                InputMediaAnimation::default().caption("test"),
                            )
                            .unwrap();
                            this.client
                                .execute(EditMessageMedia::new(chat_id, reply_to.id, input_media))
                                .await
                                .unwrap();
                        }
                        // Change document to animation
                        MessageData::Document { .. } => {
                            this.client
                                .execute(EditMessageMedia::new(
                                    chat_id,
                                    reply_to.id,
                                    InputMedia::new(
                                        InputFile::url(this.gif_url),
                                        InputMediaAnimation::default().caption("test"),
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
                                .execute(EditMessageMedia::new(chat_id, reply_to.id, input_media))
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
                                .execute(EditMessageMedia::new(chat_id, reply_to.id, input_media))
                                .await
                                .unwrap();
                        }
                        _ => {}
                    }
                } else if let MessageData::Document { data, .. } = message.data {
                    // Resend document by file id (you also can send a document using URL)
                    this.client
                        .execute(SendDocument::new(chat_id, InputFile::file_id(data.file_id)))
                        .await
                        .unwrap();
                } else if let Some(text) = message.get_text() {
                    match text.data.as_str() {
                        // Send animation by URL (you also can send animation using a file_id)
                        "/gif" => {
                            let method = SendAnimation::new(chat_id, InputFile::url(this.gif_url));
                            this.client.execute(method).await.unwrap();
                        }
                        "/photo" => {
                            let markup = vec![vec![InlineKeyboardButton::with_callback_data("test", "cb-data")]];
                            let method = SendPhoto::new(chat_id, InputFile::path(this.photo_path).await.unwrap())
                                .reply_markup(markup)
                                .unwrap();
                            this.client.execute(method).await.unwrap();
                        }
                        "/text" => {
                            let document = Cursor::new(b"Hello World!");
                            let reader = InputFileReader::new(document).info(("hello.txt", mime::TEXT_PLAIN));
                            let method = SendDocument::new(chat_id, reader)
                                .thumb(InputFile::path(this.document_thumb_path).await.unwrap());
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
