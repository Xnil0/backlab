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
        MyResult,
    },
    bytes::Bytes,
    std::sync::mpsc,
};

#[test]
fn run_channel_with_mpsc() -> MyResult<()> {
    let source = "mpsc://sender.com";
    let destination = "mpsc://receiver.com";
    let payload = Bytes::from("Hello, World!");

    let (from, to) = mpsc::channel();
    let mpsc_sender = MpscSender::new(source, from);
    let mpsc_receiver = MpscReceiver::new(destination, to);
    let channel = Channel::new(&mpsc_sender, &mpsc_receiver);
    assert_eq!(channel.from().address(), source);
    assert_eq!(channel.to().address(), destination);

    let envelope = Envelope::new(source, destination, payload.clone())?
        .add_header("content-type", "text/plain")
        .add_header("encoding", "utf-8");

    let queueing = channel.enqueue(envelope);
    assert!(queueing.is_ok());
    queueing.unwrap();

    let dequeueing = channel.dequeue();
    assert!(dequeueing.is_ok());

    let message = dequeueing.unwrap();
    assert_eq!(
        message.source().to_string(),
        channel.from().address(),
    );
    assert_eq!(
        message
            .destination()
            .to_string(),
        channel.to().address(),
    );
    assert_eq!(message.payload(), &payload);

    let content_type = message.get_header("content-type");
    assert!(content_type.is_some());
    assert_eq!(content_type.unwrap(), "text/plain");

    let encoding = message.get_header("encoding");
    assert!(encoding.is_some());
    assert_eq!(encoding.unwrap(), "utf-8");

    Ok(())
}

#[test]
fn run_duplex_channels_with_mpsc() -> MyResult<()> {
    // Create Client Envelope.
    let source_address = "mpsc://sender.com";
    let destination_address = "mpsc://receiver.com";
    let payload = Bytes::from("Hello Server");
    let envelope = Envelope::new(
        source_address,
        destination_address,
        payload.clone(),
    )?;

    // Instantiate MPSC channels.
    let (client_sender, server_receiver) = mpsc::channel();
    let (server_sender, client_receiver) = mpsc::channel();

    // Cross MPSC channels froms and tos to create connected Endpoints.
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
    let message = dequeueing.unwrap();
    assert_eq!(
        message.source().to_string(),
        mpsc_client.address()
    );
    assert_eq!(
        message
            .destination()
            .to_string(),
        mpsc_server.address()
    );
    assert_eq!(message.payload(), &payload);

    // Create Server Envelope.
    let payload = Bytes::from("Hello Client");
    let message = Envelope::new(
        destination_address,
        source_address,
        payload.clone(),
    )?
    .add_header("encoding", "utf8")
    .add_header("content-type", "text/plain");

    // Create second Channel (Server-Client) and send response.
    let chan_server_client = Channel::new(&mpsc_server, &mpsc_client);
    let enqueueing = chan_server_client.enqueue(message);
    assert!(enqueueing.is_ok());

    let dequeueing = chan_server_client.dequeue();
    assert!(dequeueing.is_ok());

    // Check Envelope Integrity.
    let message = dequeueing.unwrap();
    assert_eq!(
        message.source().to_string(),
        mpsc_server.address()
    );
    assert_eq!(
        message
            .destination()
            .to_string(),
        mpsc_client.address()
    );
    assert_eq!(message.payload(), &payload);

    let encoding = message.get_header("encoding");
    assert!(encoding.is_some());
    assert_eq!(encoding.unwrap(), "utf8");

    let content_type = message.get_header("content-type");
    assert!(content_type.is_some());
    assert_eq!(content_type.unwrap(), "text/plain");

    Ok(())
}
