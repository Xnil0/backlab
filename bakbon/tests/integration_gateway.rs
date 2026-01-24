mod common;

use {
    crate::common::EchoService,
    bakbon::{
        Address,
        Gateway,
        Payload,
        Protocol,
        Registry,
        Result,
        Router,
        Service,
    },
};

#[test]
fn gateway_to_router_to_service() -> Result<()> {
    let path = "/echo";
    let payload = Payload::from("Hello, World!");

    // Create Echo Service.
    let srv_uri = "inproc://echo";
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

    // Build Gateway.
    let gw_url = "https://service.com";
    let gw_port = 8080;

    let gateway = Gateway::builder(gw_url, gw_port)?
        .protocol("inproc")
        .build();
    assert_eq!(gateway.protocol(), &Protocol::InProc);

    let msg = gateway.handle(path, payload.clone())?;
    assert_eq!(msg.source(), gateway.address());
    assert_eq!(
        msg.destination().to_string(),
        "inproc://echo"
    );
    assert_eq!(msg.payload(), &payload);

    // Get and Process Reply.
    let reply = router.route(msg)?;
    assert!(reply.is_some());

    let reply = reply.unwrap();
    assert_eq!(reply.payload(), &payload);
    assert_eq!(reply.source(), &srv_addr);
    assert_eq!(reply.destination(), gateway.address());
    Ok(())
}
