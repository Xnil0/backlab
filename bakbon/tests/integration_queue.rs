use {
    crate::common::EchoService,
    bakbon::{
        Address,
        Envelope,
        Error,
        Queue,
        Registry,
        Router,
        Service,
    },
    bytes::Bytes,
};

mod common;

#[test]
fn queue_to_router_to_service() {
    // Create Client Address.
    let client_uri = "http://client-service.com";
    let client_addr = Address::parse(client_uri);
    assert!(client_addr.is_ok());
    let client_addr = client_addr.unwrap();

    // Create Echo Service.
    let srv_uri = "http://service.com/echo";
    let srv_addr = Address::parse(srv_uri);
    assert!(srv_addr.is_ok());
    let srv_addr = srv_addr.unwrap();
    let service = EchoService::new(srv_addr.clone());
    assert_eq!(service.address(), &srv_addr);

    // Create the Message.
    let payload = Bytes::from("Hello...");
    let msg1 = Envelope::new(
        client_addr.clone(),
        srv_addr.clone(),
        payload.clone(),
    );
    let msg2 = Envelope::new(
        client_addr.clone(),
        srv_addr.clone(),
        payload.clone(),
    );
    let msg3 = Envelope::new(
        client_addr.clone(),
        srv_addr,
        payload.clone(),
    );

    // Build Queue.
    let queue = Queue::builder()
        .capacity(2)
        .build();
    assert_eq!(queue.len(), 0);

    // Build Registry.
    let registry = Registry::builder()
        .register(service)
        .build();
    assert!(!registry.list().is_empty());

    // Build Router.
    let mut router = Router::builder()
        .registry(registry)
        .build();
    assert_eq!(router.balancing_strategy(), "round_robin");

    let result = queue.enqueue(msg1);
    assert!(result.is_ok());
    assert_eq!(queue.len(), 1);

    let result = queue.enqueue(msg2);
    assert!(result.is_ok());
    assert_eq!(queue.len(), 2);

    let msg3 = queue.enqueue(msg3);
    assert!(msg3.is_err());
    let msg3 = match msg3.unwrap_err() {
        Error::QueueFull(msg) => msg,
        _ => panic!("Unexpected error"),
    };

    let msg = queue.dequeue();
    assert!(msg.is_ok());
    assert_eq!(queue.len(), 1);

    let msg = msg.unwrap();
    assert!(msg.is_some());
    let msg = msg.unwrap();

    let result = queue.enqueue(msg3);
    assert!(result.is_ok());
    assert_eq!(queue.len(), 2);

    // Get Reply from Router.
    let reply = router.route(msg);
    assert!(reply.is_ok());
    let reply = reply.unwrap();
    assert!(reply.is_some());

    // Check Reply Payload.
    let reply = reply.unwrap();
    assert_eq!(reply.payload(), &payload);

    // Check Addresses Swap.
    assert_eq!(reply.source().to_string(), srv_uri);
    assert_eq!(reply.destination(), &client_addr);
}
