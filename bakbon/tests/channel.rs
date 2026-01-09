mod common;

use {
    crate::common::{
        MpscReceiver,
        MpscSender,
    },
    bakbon::{
        Channel,
        Envelope,
        Message,
    },
    std::sync::mpsc,
};

#[test]
fn run_channel_with_mpsc() {
    let source = "Sender";
    let destination = "Receiver";
    let data = "Hello, world!".to_string();

    let (sender, receiver) = mpsc::channel();
    let mpsc_sender = MpscSender::new(source, sender);
    let mpsc_receiver = MpscReceiver::new(destination, receiver);
    let channel = Channel::new(mpsc_sender, mpsc_receiver);
    assert_eq!(channel.sender().address(), source);
    assert_eq!(channel.receiver().address(), destination);

    let message = Message::new(data)
        .add_header("content-type", "text/plain")
        .add_header("encoding", "utf-8");
    let envelope = Envelope::new(source, destination, message);

    let queueing = channel.enqueue(envelope);
    assert!(queueing.is_ok());
    queueing.unwrap();

    let dequeueing = channel.dequeue();
    assert!(dequeueing.is_ok());

    let received_envelope = dequeueing.unwrap();
    assert_eq!(received_envelope.source(), source);
    assert_eq!(received_envelope.destination(), destination);

    assert_eq!(
        received_envelope.source(),
        channel.sender().address(),
    );
    assert_eq!(
        received_envelope.destination(),
        channel.receiver().address(),
    );

    let received_message = received_envelope.message();
    assert_eq!(received_message.payload(), "Hello, world!");

    let content_type = received_message.get_header("content-type");
    assert!(content_type.is_some());
    assert_eq!(content_type.unwrap(), "text/plain");

    let encoding = received_message.get_header("encoding");
    assert!(encoding.is_some());
    assert_eq!(encoding.unwrap(), "utf-8");
}
