mod common;

use {
    crate::common::EchoService,
    bakbon::{
        Address,
        Gateway,
        Protocol,
        Registry,
        Result,
        Router,
        Service,
    },
    bytes::Bytes,
};

#[test]
fn gateway_to_router_to_service() -> Result<()> {
    let path = "/echo";
    let payload = Bytes::from("Hello, World!");

    // Create Echo Service.
    let srv_uri = "inproc://service.com/echo";
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

    // Build Gateway.
    let gw_url = "https://service.com";
    let gw_port = 8080;

    let gw_builder = Gateway::builder(gw_url, gw_port);
    assert!(gw_builder.is_ok());
    let gw_builder = gw_builder.unwrap();

    let gateway = gw_builder
        .protocol("inproc")
        .build();
    assert_eq!(gateway.protocol(), &Protocol::InProc);

    let msg = gateway.handle(path, payload.clone())?;
    assert_eq!(msg.source(), gateway.address());
    assert_eq!(msg.destination(), &srv_addr);
    assert_eq!(msg.payload(), &payload);

    // Get and Process Reply.
    let reply = router.route(msg)?;
    // assert!(reply.is_ok());

    // let reply = reply.unwrap();
    assert!(reply.is_some());

    let reply = reply.unwrap();
    assert_eq!(reply.payload(), &payload);
    assert_eq!(reply.source(), &srv_addr);
    assert_eq!(reply.destination(), gateway.address());
    Ok(())
}
