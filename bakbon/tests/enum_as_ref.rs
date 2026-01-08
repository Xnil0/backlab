use bakbon::MessageKind;

#[test]
fn message_kind_as_ref() {
    let command = MessageKind::Command;
    let query = MessageKind::Query;
    let reply = MessageKind::Reply;
    let event = MessageKind::Event;
    let telemetry = MessageKind::Telemetry;
    let custom = MessageKind::Custom("custom".to_string());

    assert_eq!(command.as_ref(), "command");
    assert_eq!(query.as_ref(), "query");
    assert_eq!(reply.as_ref(), "reply");
    assert_eq!(event.as_ref(), "event");
    assert_eq!(telemetry.as_ref(), "telemetry");
    assert_eq!(custom.as_ref(), "custom");
}
