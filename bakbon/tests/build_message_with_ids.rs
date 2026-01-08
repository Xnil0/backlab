use bakbon::{
    Message,
    MessageKind,
};

#[test]
fn build_message_with_id() {
    let id = "random_id";
    let kind = MessageKind::Event;
    let body = "random_body_content";

    let msg = Message::builder(kind, body)
        .id(id)
        .build();

    assert_eq!(msg.id(), id);
}

#[test]
fn build_message_with_empty_id() {
    let id = "";
    let kind = MessageKind::Query;
    let body = 45;

    let message = Message::builder(kind, body)
        .id(id)
        .build();

    assert!(!message.id().is_empty());
}

#[test]
fn build_message_with_correlation_id() {
    let id = "random_correlation_id";
    let kind = MessageKind::Event;
    let body = "random_body_content";

    let msg = Message::builder(kind, body)
        .correlation_id(id)
        .build();

    assert!(msg.correlation_id().is_some());
    assert_eq!(msg.correlation_id().unwrap(), id);
}

#[test]
fn build_message_with_empty_correlation_id() {
    let id = "";
    let kind = MessageKind::Command;
    let body = "random_body_content";

    let msg = Message::builder(kind, body)
        .correlation_id(id)
        .build();

    assert!(msg.correlation_id().is_none());
}

#[test]
fn build_message_with_causation_id() {
    let id = "random_causation_id";
    let kind = MessageKind::Event;
    let body = "random_body_content";

    let msg = Message::builder(kind, body)
        .causation_id(id)
        .build();

    assert!(msg.causation_id().is_some());
    assert_eq!(msg.causation_id().unwrap(), id);
}
