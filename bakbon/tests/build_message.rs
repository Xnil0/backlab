use bakbon::{
    ContentType,
    Encoding,
    Fanout,
    Guarantee,
    Message,
    MessageKind,
    Priority,
};

#[test]
fn build_message() {
    let kind = MessageKind::Command;
    let body = "random_body_content";

    let msg = Message::builder(kind, body).build();

    assert!(!msg.id().is_empty());
    assert!(msg.correlation_id().is_none());
    assert!(msg.causation_id().is_none());
    assert!(msg.trace_id().is_none());
    assert!(msg.span_id().is_none());

    assert_eq!(msg.kind(), &MessageKind::Command);
    assert_eq!(msg.kind().as_ref(), "command");
    assert!(!msg.expect_reply());
    assert!(msg.method().is_none());
    assert!(msg.intent().is_none());

    assert_eq!(msg.content_type(), &ContentType::default());
    assert_eq!(msg.encoding(), &Encoding::default());
    assert_eq!(msg.body(), &body);

    assert_eq!(msg.guarantee(), &Guarantee::default());
    assert_eq!(msg.fanout(), &Fanout::default());
    assert_eq!(msg.priority(), &Priority::default());
    assert!(msg.deadline().is_none());
    assert!(msg.delay().is_none());
    assert_eq!(msg.retries(), 0);
    assert!(msg.time_to_live().is_none());
    assert!(msg.ordering_key().is_none());

    assert!(msg.subject().is_none());
    assert!(msg.roles().is_empty());
    assert!(msg.permissions().is_empty());
    assert!(msg.integrity_tag().is_none());

    assert!(
        msg.timestamp()
            .elapsed()
            .is_ok()
    );
}
