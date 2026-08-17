#![allow(missing_docs)]

use tgbot::types::*;

mod cx;

use cx::Cx;

#[tokio::test]
async fn execute() {
    let mut cx = Cx::new().await;
    cx.execute_batch([
        (
            "send-checklist-base",
            SendChecklist::new(
                "c-id",
                1,
                InputChecklist::new([InputChecklistTask::new(1, "test")], "test"),
            ),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Checklist(data) = x.data {
                    assert_eq!(data.title, "test");
                    assert_eq!(data.tasks.len(), 1);
                    let task = &data.tasks[0];
                    assert_eq!(task.id, 1);
                    assert_eq!(task.text, "test");
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
        (
            "send-checklist-full",
            SendChecklist::new(
                "c-id",
                1,
                InputChecklist::new(
                    [
                        InputChecklistTask::new(1, ("test 1", [TextEntity::bold(0..2)])),
                        InputChecklistTask::new(2, ("test 2", ParseMode::Html)),
                    ],
                    ("test", [TextEntity::bold(0..2)]),
                )
                .with_others_can_add_tasks(true)
                .with_others_can_mark_tasks_as_done(true),
            )
            .with_disable_notification(true)
            .with_protect_content(true)
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
            .with_reply_parameters(ReplyParameters::new(1))
            .with_message_effect_id("effect-id"),
            |x| {
                assert_eq!(x.id, 1);
                if let MessageData::Checklist(data) = x.data {
                    assert_eq!(data.title, "test");
                    let mut title_entities = data.title_entities.unwrap().into_iter();
                    let title_entity = title_entities.next().unwrap();
                    assert!(matches!(
                        title_entity,
                        TextEntity::Bold(TextEntityPosition { offset: 0, length: 2 })
                    ));
                    assert!(title_entities.next().is_none());
                    assert!(data.others_can_add_tasks.unwrap());
                    assert!(data.others_can_mark_tasks_as_done.unwrap());
                    let task_1 = &data.tasks[0];
                    assert_eq!(task_1.id, 1);
                    assert_eq!(task_1.text, "test 1");
                    assert!(task_1.text_entities.is_some());
                    assert_eq!(task_1.completed_by_user.as_ref().map(|x| x.id).unwrap(), 1);
                    assert!(task_1.completed_by_chat.is_none());
                    assert_eq!(task_1.completion_date.unwrap(), 0);
                    let task_2 = &data.tasks[1];
                    assert_eq!(task_2.id, 2);
                    assert!(task_2.completed_by_user.is_none());
                    assert!(task_2.completed_by_chat.is_some());
                } else {
                    panic!("Got an unexpected message data: {:?}", x.data);
                }
            },
        ),
    ])
    .await;
    cx.execute_batch([
        (
            "edit-message-checklist-base",
            EditMessageChecklist::new(
                "c-id",
                1,
                InputChecklist::new([InputChecklistTask::new(1, "test")], "test"),
                2,
            ),
            |x| assert_eq!(x.id, 1),
        ),
        (
            "edit-message-checklist-all",
            EditMessageChecklist::new(
                "c-id",
                1,
                InputChecklist::new([InputChecklistTask::new(1, "test")], "test"),
                2,
            )
            .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]),
            |x| assert_eq!(x.id, 1),
        ),
    ])
    .await;
}
