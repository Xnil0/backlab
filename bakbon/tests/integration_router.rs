mod common;

use {
    crate::common::EchoService,
    bakbon::{
        Address,
        Envelope,
        Registry,
        Router,
        Service,
    },
    bytes::Bytes,
};

#[test]
fn router_to_service() {
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

    // Create the Message.
    let payload = Bytes::from("Hello...");
    let msg = Envelope::new(
        client_addr.clone(),
        srv_addr,
        payload.clone(),
    )
    .header("content-type", "text/plain")
    .header("encoding", "utf-8");

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

    // Check Headers.
    let content_type = reply.get_header("content-type");
    assert!(content_type.is_some());
    assert_eq!(content_type.unwrap(), "text/plain");

    let encoding = reply.get_header("encoding");
    assert!(encoding.is_some());
    assert_eq!(encoding.unwrap(), "utf-8");
}
