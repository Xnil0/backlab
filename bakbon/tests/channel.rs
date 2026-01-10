mod common;

use {
    crate::common::{
        MpscEndpoint,
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
    let data = 1;

    let (sender, receiver) = mpsc::channel();
    let mpsc_sender = MpscSender::new(source, sender);
    let mpsc_receiver = MpscReceiver::new(destination, receiver);
    let channel = Channel::new(&mpsc_sender, &mpsc_receiver);
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
    assert_eq!(
        received_envelope.source(),
        channel.sender().address(),
    );
    assert_eq!(
        received_envelope.destination(),
        channel.receiver().address(),
    );

    let received_message = received_envelope.message();
    assert_eq!(received_message.payload(), &data);

    let content_type = received_message.get_header("content-type");
    assert!(content_type.is_some());
    assert_eq!(content_type.unwrap(), "text/plain");

    let encoding = received_message.get_header("encoding");
    assert!(encoding.is_some());
    assert_eq!(encoding.unwrap(), "utf-8");
}

#[test]
fn run_duplex_channels_with_mpsc() {
    // Create Client Envelope.
    let source_address = "Client";
    let destination_address = "Server";
    let data = "Hello Server";
    let message = Message::new(data);
    let envelope = Envelope::new(source_address, destination_address, message);

    // Instantiate MPSC channels.
    let (client_sender, server_receiver) = mpsc::channel();
    let (server_sender, client_receiver) = mpsc::channel();

    // Cross MPSC channels senders and receivers to create connected Endpoints.
    let mpsc_client = MpscEndpoint::new(
        source_address,
        client_sender,
        client_receiver,
    );
    assert_eq!(mpsc_client.address(), source_address);

    let mpsc_server = MpscEndpoint::new(
        destination_address,
        server_sender,
        server_receiver,
    );
    assert_eq!(mpsc_server.address(), destination_address);

    // Create first Channel (Client-Server) and send envelope.
    let chan_client_server = Channel::new(&mpsc_client, &mpsc_server);
    let enqueueing = chan_client_server.enqueue(envelope);
    assert!(enqueueing.is_ok());
    let dequeueing = chan_client_server.dequeue();
    assert!(dequeueing.is_ok());
    
    // Check Envelope Integrity.
    let envelope = dequeueing.unwrap();
    assert_eq!(envelope.source(), mpsc_client.address());
    assert_eq!(envelope.destination(), mpsc_server.address());
    assert_eq!(envelope.message().payload(), &data);

    // Create Server Envelope.
    let data = "Hello Client";
    let message = Message::new(data).add_header("encoding", "utf8");
    let envelope = Envelope::new(destination_address, source_address, message);

    // Create second Channel (Server-Client) and send response.
    let chan_server_client = Channel::new(&mpsc_server, &mpsc_client);
    let enqueueing = chan_server_client.enqueue(envelope);
    assert!(enqueueing.is_ok());
    let dequeueing = chan_server_client.dequeue();
    assert!(dequeueing.is_ok());

    // Check Envelope Integrity.
    let envelope = dequeueing.unwrap();
    assert_eq!(envelope.source(), mpsc_server.address());
    assert_eq!(envelope.destination(), mpsc_client.address());
    let message = envelope.message();
    assert_eq!(message.payload(), &data);
    let encoding = message.get_header("encoding");
    assert!(encoding.is_some());
    assert_eq!(encoding.unwrap(), "utf8");
}
