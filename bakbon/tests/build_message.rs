use bakbon::Message;

#[test]
fn build_default_message() {
    let msg = Message::default();

    assert!(msg.payload().is_empty());
    assert!(!msg.has_meta());
}

#[test]
fn build_message_with_payload() {
    let payload: Vec<u8> = "random_payload".into();
    let msg = Message::new(payload);

    assert!(!msg.payload().is_empty());
    assert_eq!(msg.payload(), &Vec::from("random_payload"));
}

#[test]
fn build_message_with_empty_payload() {
    let payload: Vec<u8> = vec![];
    let msg = Message::new(payload);

    assert!(msg.payload().is_empty());
}

#[test]
fn build_message_with_metadata() {
    let payload = String::from("random_payload");

    let msg = Message::new(payload)
        .with_header("content-type", "text/plain")
        .with_header("encoding", "utf-8");

    let content_type = msg.meta("content-type");
    assert!(content_type.is_some());
    assert_eq!(content_type.unwrap(), "text/plain");

    let encoding = msg.meta("encoding");
    assert!(encoding.is_some());
    assert_eq!(encoding.unwrap(), "utf-8");
}
