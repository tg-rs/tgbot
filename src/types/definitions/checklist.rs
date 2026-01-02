use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{Chat, InlineKeyboardMarkup, Integer, Message, ParseMode, ReplyParameters, TextEntities, TextEntity, User},
};

/// Describes a task in a checklist.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChecklistTask {
    /// Unique identifier of the task.
    pub id: Integer,
    /// Text of the task.
    pub text: String,
    /// Chat that completed the task; omitted if the task wasn't completed by a chat.
    pub completed_by_chat: Option<Chat>,
    /// User that completed the task; omitted if the task wasn't completed.
    pub completed_by_user: Option<User>,
    /// Point in time (Unix timestamp) when the task was completed; 0 if the task wasn't completed.
    pub completion_date: Option<Integer>,
    /// Special entities that appear in the task text.
    pub text_entities: Option<TextEntities>,
}

impl ChecklistTask {
    /// Creates a new `ChecklistTask`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the task.
    /// * `text` - Text of the task.
    pub fn new<T>(id: Integer, text: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            id,
            text: text.into(),
            completed_by_chat: None,
            completed_by_user: None,
            completion_date: None,
            text_entities: None,
        }
    }

    /// Sets a new chat that completed the task.
    ///
    /// # Arguments
    ///
    /// * `value` - Chat that completed the task; omitted if the task wasn't completed by a chat.
    pub fn with_completed_by_chat<T>(mut self, value: T) -> Self
    where
        T: Into<Chat>,
    {
        self.completed_by_chat = Some(value.into());
        self
    }

    /// Sets a new user that completed the task.
    ///
    /// # Arguments
    ///
    /// * `value` - User that completed the task; omitted if the task wasn't completed.
    pub fn with_completed_by_user(mut self, value: User) -> Self {
        self.completed_by_user = Some(value);
        self
    }

    /// Sets a new completion date
    ///
    /// # Arguments
    ///
    /// * `value` - Point in time (Unix timestamp) when the task was completed; 0 if the task wasn't completed.
    pub fn with_completion_date(mut self, value: Integer) -> Self {
        self.completion_date = Some(value);
        self
    }

    /// Sets a new list of text entities.
    ///
    /// # Arguments
    ///
    /// * `value` - Special entities that appear in the task text.
    pub fn with_text_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.text_entities = Some(TextEntities::from_iter(value));
        self
    }
}

/// Describes a checklist.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Checklist {
    /// List of tasks in the checklist.
    pub tasks: Vec<ChecklistTask>,
    /// Title of the checklist.
    pub title: String,
    /// Whether users other than the creator of the list can add tasks to the list.
    pub others_can_add_tasks: Option<bool>,
    /// Whether users other than the creator of the list can mark tasks as done or not done.
    pub others_can_mark_tasks_as_done: Option<bool>,
    /// Special entities that appear in the checklist title.
    pub title_entities: Option<TextEntities>,
}

impl Checklist {
    /// Creates a new `Checklist`.
    ///
    /// # Arguments
    ///
    /// * `tasks` - List of tasks in the checklist.
    /// * `title` - Title of the checklist.
    pub fn new<A, B>(tasks: A, title: B) -> Self
    where
        A: IntoIterator<Item = ChecklistTask>,
        B: Into<String>,
    {
        Self {
            tasks: tasks.into_iter().collect(),
            title: title.into(),
            others_can_add_tasks: None,
            others_can_mark_tasks_as_done: None,
            title_entities: None,
        }
    }

    /// Sets a new value for the `others_can_add_tasks` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether users other than the creator of the list can add tasks to the list.
    pub fn with_others_can_add_tasks(mut self, value: bool) -> Self {
        self.others_can_add_tasks = Some(value);
        self
    }

    /// Sets a new value for the `others_can_mark_tasks_as_done` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether users other than the creator of the list can mark tasks as done or not done.
    pub fn with_others_can_mark_tasks_as_done(mut self, value: bool) -> Self {
        self.others_can_mark_tasks_as_done = Some(value);
        self
    }

    /// Sets a new list of title entities.
    ///
    /// # Arguments
    ///
    /// * `value` - Special entities that appear in the checklist title
    pub fn with_title_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.title_entities = Some(TextEntities::from_iter(value));
        self
    }
}

/// Describes a task to add to a checklist.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputChecklistTask {
    /// Unique identifier of the task.
    ///
    /// Must be positive and unique among all task identifiers currently present in the checklist.
    pub id: Integer,
    /// Text of the task.
    ///
    /// 1-100 characters after entities parsing.
    pub text: String,
    /// Mode for parsing entities in the text.
    pub parse_mode: Option<ParseMode>,
    /// List of special entities that appear in the text.
    ///
    /// Can be specified instead of `parse_mode`.
    /// Currently, only bold, italic, underline, strikethrough, spoiler, and custom_emoji entities are allowed.
    pub text_entities: Option<TextEntities>,
}

impl InputChecklistTask {
    /// Creates a new `InputChecklistTask`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the task.
    /// * `text` - Text of the task
    pub fn new<T>(id: Integer, text: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            id,
            text: text.into(),
            parse_mode: None,
            text_entities: None,
        }
    }

    /// Sets a new parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Mode for parsing entities in the text.
    ///
    /// Text entities will be set to [`None`] when this method is called.
    pub fn with_parse_mode(mut self, value: ParseMode) -> Self {
        self.text_entities = None;
        self.parse_mode = Some(value);
        self
    }

    /// Sets a new list of text entities.
    ///
    /// # Arguments
    ///
    /// * `value` - List of special entities that appear in the text.
    ///
    /// Parse mode will be set to [`None`] when this method is called.
    pub fn with_text_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.parse_mode = None;
        self.text_entities = Some(TextEntities::from_iter(value));
        self
    }
}

/// Describes a checklist to create.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputChecklist {
    /// List of 1-30 tasks in the checklist.
    pub tasks: Vec<InputChecklistTask>,
    /// Title of the checklist.
    ///
    /// 1-255 characters after entities parsing.
    pub title: String,
    /// Whether other users can add tasks to the checklist.
    pub others_can_add_tasks: Option<bool>,
    /// Whether other users can mark tasks as done or not done in the checklist.
    pub others_can_mark_tasks_as_done: Option<bool>,
    /// Mode for parsing entities in the title.
    pub parse_mode: Option<ParseMode>,
    /// List of special entities that appear in the title.
    ///
    /// Can be specified instead of parse_mode.
    /// Currently, only bold, italic, underline, strikethrough, spoiler, and custom_emoji entities are allowed.
    pub title_entities: Option<TextEntities>,
}

impl InputChecklist {
    /// Creates a new `InputChecklist`.
    ///
    /// # Arguments
    ///
    /// * `tasks` - List of 1-30 tasks in the checklist.
    /// * `title` - Title of the checklist.
    pub fn new<A, B>(tasks: A, title: B) -> Self
    where
        A: IntoIterator<Item = InputChecklistTask>,
        B: Into<String>,
    {
        Self {
            tasks: tasks.into_iter().collect(),
            title: title.into(),
            others_can_add_tasks: None,
            others_can_mark_tasks_as_done: None,
            parse_mode: None,
            title_entities: None,
        }
    }

    /// Sets a new value for the `others_can_add_tasks` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether other users can add tasks to the checklist.
    pub fn with_others_can_add_tasks(mut self, value: bool) -> Self {
        self.others_can_add_tasks = Some(value);
        self
    }

    /// Sets a new value for the `others_can_mark_tasks_as_done` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether other users can mark tasks as done or not done in the checklist.
    pub fn with_others_can_mark_tasks_as_done(mut self, value: bool) -> Self {
        self.others_can_mark_tasks_as_done = Some(value);
        self
    }

    /// Sets a new parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Mode for parsing entities in the title.
    ///
    /// Title entities will be set to [`None`] when this method is called.
    pub fn with_parse_mode(mut self, value: ParseMode) -> Self {
        self.title_entities = None;
        self.parse_mode = Some(value);
        self
    }

    /// Sets a new list of text entities.
    ///
    /// # Arguments
    ///
    /// * `value` - List of special entities that appear in the title.
    ///
    /// Parse mode will be set to [`None`] when this method is called.
    pub fn with_title_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.parse_mode = None;
        self.title_entities = Some(TextEntities::from_iter(value));
        self
    }
}

/// Describes a service message about checklist tasks marked as done or not done.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ChecklistTasksDone {
    /// Message containing the checklist whose tasks were marked as done or not done.
    ///
    /// Note that the Message object in this field will not contain
    /// the reply_to_message field even if it itself is a reply.
    pub checklist_message: Option<Box<Message>>,
    /// Identifiers of the tasks that were marked as done.
    pub marked_as_done_task_ids: Option<Vec<Integer>>,
    /// Identifiers of the tasks that were marked as not done.
    pub marked_as_not_done_task_ids: Option<Vec<Integer>>,
}

impl ChecklistTasksDone {
    /// Sets a new checklist message.
    ///
    /// # Arguments
    ///
    /// * `value` - Message containing the checklist whose tasks were marked as done or not done.
    pub fn with_checklist_message(mut self, value: Message) -> Self {
        self.checklist_message = Some(Box::new(value));
        self
    }

    /// Sets a new list of task identifiers.
    ///
    /// # Arguments
    ///
    /// * `value` - Identifiers of the tasks that were marked as done.
    pub fn with_marked_as_done_task_ids<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = Integer>,
    {
        self.marked_as_done_task_ids = Some(value.into_iter().collect());
        self
    }

    /// Sets a new list of task identifiers.
    ///
    /// # Arguments
    ///
    /// * `value` - Identifiers of the tasks that were marked as not done.
    pub fn with_marked_as_not_done_task_ids<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = Integer>,
    {
        self.marked_as_not_done_task_ids = Some(value.into_iter().collect());
        self
    }
}

/// Describes a service message about tasks added to a checklist.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChecklistTasksAdded {
    /// List of tasks added to the checklist.
    pub tasks: Vec<ChecklistTask>,
    /// Message containing the checklist to which the tasks were added.
    ///
    /// Note that the Message object in this field will not contain
    /// the reply_to_message field even if it itself is a reply.
    pub checklist_message: Option<Box<Message>>,
}

impl ChecklistTasksAdded {
    /// Creates a new `ChecklistTasksAdded`.
    ///
    /// # Arguments
    ///
    /// * `tasks` - List of tasks added to the checklist.
    pub fn new<T>(tasks: T) -> Self
    where
        T: IntoIterator<Item = ChecklistTask>,
    {
        Self {
            tasks: tasks.into_iter().collect(),
            checklist_message: None,
        }
    }

    /// Sets a new checklist message.
    ///
    /// # Arguments
    ///
    /// * `value` - Message containing the checklist to which the tasks were added.
    pub fn with_checklist_message(mut self, value: Message) -> Self {
        self.checklist_message = Some(Box::new(value));
        self
    }
}

/// Sends a checklist on behalf of a connected business account.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SendChecklist {
    business_connection_id: String,
    chat_id: Integer,
    checklist: InputChecklist,
    disable_notification: Option<bool>,
    message_effect_id: Option<String>,
    protect_content: Option<bool>,
    reply_markup: Option<InlineKeyboardMarkup>,
    reply_parameters: Option<ReplyParameters>,
}

impl SendChecklist {
    /// Creates a new `SendChecklist`.
    ///
    /// # Arguments
    ///
    /// * `business_connection_id` - Unique identifier of the business connection.
    /// * `chat_id` - Unique identifier for the target chat.
    /// * `checklist` - The checklist to send.
    pub fn new<T>(business_connection_id: T, chat_id: Integer, checklist: InputChecklist) -> Self
    where
        T: Into<String>,
    {
        Self {
            business_connection_id: business_connection_id.into(),
            chat_id,
            checklist,
            disable_notification: None,
            message_effect_id: None,
            protect_content: None,
            reply_markup: None,
            reply_parameters: None,
        }
    }

    /// Sets a new value for the `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to send the message silently.
    ///
    /// Users will receive a notification with no sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    /// Sets a new message effect ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the message effect to be added to the message.
    pub fn with_message_effect_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.message_effect_id = Some(value.into());
        self
    }

    /// Sets a new value for the `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to protect the contents of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.protect_content = Some(value);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    // * `value` - An inline keyboard.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }

    /// Sets a new list of reply parameters.
    ///
    /// # Arguments
    ///
    // * `value` - Description of the message to reply to.
    pub fn with_reply_parameters(mut self, value: ReplyParameters) -> Self {
        self.reply_parameters = Some(value);
        self
    }
}

impl Method for SendChecklist {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::json("sendChecklist", self)
    }
}

/// Edits a checklist on behalf of a connected business account. On success, the edited Message is returned.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EditMessageChecklist {
    business_connection_id: String,
    chat_id: Integer,
    checklist: InputChecklist,
    message_id: Integer,
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageChecklist {
    /// Creates a new `EditMessageChecklist`.
    ///
    /// # Arguments
    ///
    /// * `business_connection_id` - Unique identifier of the business connection.
    /// * `chat_id` - Unique identifier for the target chat.
    /// * `checklist` - The new checklist.
    /// * `message_id` - Unique identifier for the target message.
    pub fn new<T>(business_connection_id: T, chat_id: Integer, checklist: InputChecklist, message_id: Integer) -> Self
    where
        T: Into<String>,
    {
        Self {
            business_connection_id: business_connection_id.into(),
            chat_id,
            checklist,
            message_id,
            reply_markup: None,
        }
    }

    /// Sets a new reply markup.
    ///
    /// * `value` - The new inline keyboard for the message
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }
}

impl Method for EditMessageChecklist {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::json("editMessageChecklist", self)
    }
}
