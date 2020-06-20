use async_trait::async_trait;
use dotenv::dotenv;
use std::env;
use tgbot::{
    longpoll::LongPoll,
    methods::SendMediaGroup,
    types::{InputFile, InputMediaPhoto, InputMediaVideo, MediaGroup, Update},
    Api, Config, UpdateHandler,
};

struct Handler {
    api: Api,
    photo_path: String,
    photo_url: String,
    video_path: String,
}

#[async_trait]
impl UpdateHandler for Handler {
    async fn handle(&mut self, update: Update) {
        log::info!("got an update: {:?}\n", update);
        if let Some(chat_id) = update.get_chat_id() {
            let photo_url = InputFile::url(self.photo_url.clone());
            let photo_path = InputFile::path(self.photo_path.clone()).await.unwrap();
            let video_path = InputFile::path(self.video_path.clone()).await.unwrap();
            let media = MediaGroup::default()
                .add_item(photo_url, InputMediaPhoto::default().caption("Photo 01"))
                .add_item(photo_path, InputMediaPhoto::default().caption("Photo 02"))
                .add_item(video_path, InputMediaVideo::default().caption("Video 01"));
            let method = SendMediaGroup::new(chat_id, media).unwrap();
            self.api.execute(method).await.unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGBOT_TOKEN").expect("TGBOT_TOKEN is not set");
    let proxy = env::var("TGBOT_PROXY").ok();
    let photo_path = env::var("TGBOT_PHOTO_PATH").expect("TGBOT_PHOTO_PATH is not set");
    let photo_url = env::var("TGBOT_PHOTO_URL").expect("TGBOT_PHOTO_URL is not set");
    let video_path = env::var("TGBOT_VIDEO_PATH").expect("TGBOT_VIDEO_PATH is not set");
    let mut config = Config::new(token);
    if let Some(proxy) = proxy {
        config = config.proxy(proxy).expect("Failed to set proxy");
    }
    let api = Api::new(config).expect("Failed to create API");
    LongPoll::new(
        api.clone(),
        Handler {
            api,
            photo_path,
            photo_url,
            video_path,
        },
    )
    .run()
    .await;
}
