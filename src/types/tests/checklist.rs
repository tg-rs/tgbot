use crate::types::*;

#[test]
fn checklist() {
    let expected_struct = Checklist::new([], "test");
    insta::assert_json_snapshot!(expected_struct.clone());
    insta::assert_json_snapshot!(
        expected_struct
            .with_others_can_add_tasks(true)
            .with_others_can_mark_tasks_as_done(true)
            .with_title_entities([TextEntity::bold(0..2)]),
    );
}

#[test]
fn checklist_task() {
    let expected_struct = ChecklistTask::new(1, "test");
    insta::assert_json_snapshot!(expected_struct.clone());
    insta::assert_json_snapshot!(
        expected_struct
            .with_completed_by_user(User::new(1, "John", false))
            .with_completion_date(1)
            .with_text_entities([TextEntity::bold(0..2)])
    );
}

#[test]
fn checklist_tasks_added() {
    let expected_struct = ChecklistTasksAdded::new([ChecklistTask::new(1, "test")]);
    insta::assert_json_snapshot!(expected_struct.clone());
    insta::assert_json_snapshot!(expected_struct.with_checklist_message(Message::new(
        1,
        0,
        SupergroupChat::new(1, "test"),
        MessageData::Unknown(serde_json::json!({})),
        User::new(1, "test", false),
    )));
}

#[test]
fn checklist_tasks_done() {
    let expected_struct = ChecklistTasksDone::default();
    insta::assert_json_snapshot!(expected_struct.clone());
    insta::assert_json_snapshot!(
        expected_struct
            .with_checklist_message(Message::new(
                1,
                0,
                SupergroupChat::new(1, "test"),
                MessageData::Unknown(serde_json::json!({})),
                User::new(1, "test", false),
            ))
            .with_marked_as_done_task_ids([1, 2, 3])
            .with_marked_as_not_done_task_ids([4, 5, 6]),
    );
}

#[test]
fn edit_message_checklist() {
    let checklist = InputChecklist::new([InputChecklistTask::new(1, "test")], "test");
    let method = EditMessageChecklist::new("c-id", 1, checklist, 2);
    assert_payload_eq!(POST JSON "editMessageChecklist" => method.clone());
    let method = method.with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]]);
    assert_payload_eq!(POST JSON "editMessageChecklist" => method);
}

#[test]
fn input_checklist() {
    let expected_struct = InputChecklist::new([InputChecklistTask::new(1, "test")], "test");
    insta::assert_json_snapshot!(expected_struct.clone());
    let expected_struct = expected_struct.with_parse_mode(crate::types::ParseMode::MarkdownV2);
    insta::assert_json_snapshot!(expected_struct.clone());
    insta::assert_json_snapshot!(
        expected_struct
            .with_title_entities([TextEntity::bold(0..2)])
            .with_others_can_add_tasks(true)
            .with_others_can_mark_tasks_as_done(true),
    );
}

#[test]
fn input_checklist_task() {
    let expected_struct = InputChecklistTask::new(1, "test");
    insta::assert_json_snapshot!(expected_struct.clone());
    let expected_struct = expected_struct.with_parse_mode(crate::types::ParseMode::Html);
    insta::assert_json_snapshot!(expected_struct.clone());
    insta::assert_json_snapshot!(expected_struct.with_text_entities([TextEntity::bold(0..2)]));
}

#[test]
fn send_checklist() {
    let checklist = InputChecklist::new([InputChecklistTask::new(1, "test")], "test");
    let method = SendChecklist::new("c-id", 1, checklist);
    assert_payload_eq!(POST JSON "sendChecklist" => method.clone());
    let method = method
        .with_disable_notification(true)
        .with_protect_content(true)
        .with_reply_markup([[InlineKeyboardButton::for_url("text", "url")]])
        .with_reply_parameters(ReplyParameters::new(1))
        .with_message_effect_id("effect-id");
    assert_payload_eq!(POST JSON "sendChecklist" => method);
}
