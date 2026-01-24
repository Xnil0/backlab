use {
    crate::common::EchoService,
    bakbon::{
        Address,
        Envelope,
        Error,
        Payload,
        Queue,
        Registry,
        Result,
        Router,
        Service,
    },
};

mod common;

#[test]
fn queue_to_router_to_service() -> Result<()> {
    // Create Client Address.
    let client_uri = "http://client-service.com";
    let client_addr = Address::parse(client_uri)?;

    // Create Echo Service.
    let srv_uri = "http://echo";
    let srv_addr = Address::parse(srv_uri)?;

    let service = EchoService::new(srv_addr.clone());
    assert_eq!(service.address(), &srv_addr);

    // Create the Message.
    let payload = Payload::from("Hello...");
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

    queue.enqueue(msg1)?;
    assert_eq!(queue.len(), 1);
    queue.enqueue(msg2)?;
    assert_eq!(queue.len(), 2);

    let msg3 = queue.enqueue(msg3);
    assert!(msg3.is_err());
    let msg3 = match msg3.unwrap_err() {
        Error::QueueFull(msg) => msg,
        _ => panic!("Unexpected error"),
    };

    let msg = queue.dequeue()?;
    assert_eq!(queue.len(), 1);
    assert!(msg.is_some());
    let msg = msg.unwrap();

    queue.enqueue(msg3)?;
    assert_eq!(queue.len(), 2);

    // Get Reply from Router.
    let reply = router.route(msg)?;
    assert!(reply.is_some());

    // Check Reply Payload.
    let reply = reply.unwrap();
    assert_eq!(reply.payload(), &payload);

    // Check Addresses Swap.
    assert_eq!(reply.source().to_string(), srv_uri);
    assert_eq!(reply.destination(), &client_addr);

    Ok(())
}
