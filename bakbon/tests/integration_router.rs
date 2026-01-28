mod common;

use {
    crate::common::EchoService,
    bakbon::prelude::*,
};

#[test]
fn router_to_service() -> Result<()> {
    // Create Client Address.
    let client_uri = "http://client-service.com";
    let client_addr = Address::parse(client_uri)?;

    // Create Echo Service.
    let srv_uri = "http://echo";
    let srv_addr = Address::parse(srv_uri)?;
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
    let payload = Payload::from("Hello...");
    let msg = Envelope::new(
        client_addr.clone(),
        srv_addr,
        payload.clone(),
    )
    .header("content-type", "text/plain")
    .header("encoding", "utf-8");

    // Get Reply from Router.
    let reply = router.route(msg)?;
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

    Ok(())
}
