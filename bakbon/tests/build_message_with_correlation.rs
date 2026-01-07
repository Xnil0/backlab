use bakbon::{
    Message,
    MessageKind,
};

#[test]
fn test_build_message_with_correlation_id() {
    let id = "random_id";
    let kind = MessageKind::Query;
    let body = "random_body_content";
    let correlation_id = "random_correlation_id";

    let message = Message::builder(id, kind, body)
        .correlation(correlation_id)
        .build();

    assert_eq!(message.id(), id);
    assert_eq!(message.kind(), &MessageKind::Query);
    assert_eq!(message.body(), &body);
    assert_eq!(
        message.correlation_id(),
        Some(correlation_id)
    );
}

#[test]
fn test_build_message_with_empty_correlation_id() {
    let id = "random_id";
    let kind = MessageKind::Telemetry;
    let body = "random_body_content";
    let correlation_id = "";

    let message = Message::builder(id, kind, body)
        .correlation(correlation_id)
        .build();

    assert_eq!(message.id(), id);
    assert_eq!(message.kind(), &MessageKind::Telemetry);
    assert_eq!(message.body(), &body);
    assert_eq!(
        message.correlation_id(),
        Some(correlation_id)
    );
}

#[test]
fn test_build_message_with_no_correlation_id() {
    let id = "random_id";
    let kind = MessageKind::Reply;
    let body = "random_body_content";

    let message = Message::builder(id, kind, body).build();

    assert_eq!(message.id(), id);
    assert_eq!(message.kind(), &MessageKind::Reply);
    assert_eq!(message.body(), &body);
    assert_eq!(message.correlation_id(), None);
}
