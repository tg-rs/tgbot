use serde::{Deserialize, Deserializer};

use crate::types::{
    animation::Animation,
    photo_size::PhotoSize,
    primitive::Integer,
    text::{Text, TextEntities},
    user::User,
};

/// Game
///
/// Use BotFather to create and edit games,
/// their short names will act as unique identifiers
#[derive(Clone, Debug, Deserialize)]
pub struct Game {
    /// Title of the game
    pub title: String,
    /// Description of the game
    pub description: String,
    /// Photo that will be displayed in the game message in chats
    pub photo: Vec<PhotoSize>,
    /// Brief description of the game or high scores included in the game message
    /// Can be automatically edited to include current high scores for the game
    /// when the bot calls setGameScore, or manually edited using editMessageText
    /// 0-4096 characters
    #[serde(deserialize_with = "deserialize_text")]
    #[serde(flatten)]
    pub text: Option<Text>,
    /// Animation that will be displayed in the game message in chats
    /// Upload via BotFather
    pub animation: Option<Animation>,
}

fn deserialize_text<'de, D>(deserializer: D) -> Result<Option<Text>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper {
        text: String,
        text_entities: Option<TextEntities>,
    }

    Option::<Wrapper>::deserialize(deserializer).map(|wrapper| {
        wrapper.map(
            |Wrapper {
                 text: data,
                 text_entities: entities,
             }| Text { data, entities },
        )
    })
}

/// One row of the high scores table for a game
#[derive(Clone, Debug, Deserialize)]
pub struct GameHighScore {
    /// Position in high score table for the game
    pub position: Integer,
    /// User
    pub user: User,
    /// Score
    pub score: Integer,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_game_full() {
        let game: Game = serde_json::from_value(serde_json::json!({
            "title": "title",
            "description": "description",
            "photo": [
                {
                    "file_id": "photo file id",
                    "file_unique_id": "photo unique file id",
                    "width": 200,
                    "height": 200
                }
            ],
            "text": "text",
            "animation": {
                "file_id": "animation file id",
                "file_unique_id": "animation unique file id",
                "width": 200,
                "height": 200,
                "duration": 24
            }
        }))
        .unwrap();
        assert_eq!(game.title, "title");
        assert_eq!(game.description, "description");
        assert_eq!(game.photo.len(), 1);
        assert_eq!(game.photo[0].file_id, "photo file id");
        assert_eq!(game.photo[0].file_unique_id, "photo unique file id");
        assert_eq!(game.text.unwrap().data, "text");
        let animation = game.animation.unwrap();
        assert_eq!(animation.file_id, "animation file id");
        assert_eq!(animation.file_unique_id, "animation unique file id");
    }

    #[test]
    fn deserialize_game_partial() {
        let game: Game = serde_json::from_value(serde_json::json!({
            "title": "title",
            "description": "description",
            "photo": []
        }))
        .unwrap();
        assert_eq!(game.title, "title");
        assert_eq!(game.description, "description");
        assert_eq!(game.photo.len(), 0);
        assert!(game.text.is_none());
        assert!(game.animation.is_none());
    }

    #[test]
    fn deserialize_game_high_score() {
        let score: GameHighScore = serde_json::from_value(serde_json::json!({
            "position": 1,
            "user": {
                "id": 2,
                "first_name": "test",
                "is_bot": false
            },
            "score": 3
        }))
        .unwrap();
        assert_eq!(score.position, 1);
        assert_eq!(score.user.id, 2);
        assert_eq!(score.score, 3);
    }
}
