use bakbon::{
    Message,
    MessageKind,
};

#[test]
fn test_build_message() {
    let id = "random_id";
    let kind = MessageKind::Command;
    let body = "random_body_content";

    let message = Message::builder(id, kind, body).build();

    assert_eq!(message.id(), id);
    assert_eq!(message.kind(), &MessageKind::Command);
    assert_eq!(message.body(), &body)
}
