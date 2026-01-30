<h1 align=center>
    BakBon
    <br>
    <img alt="Ferris" src="../docs/ferris.svg">
</h1>

## Table of Contents

- [Table of Contents](#table-of-contents)
- [Overview](#overview)
  - [Features](#features)
- [Installation](#installation)
- [File System](#file-system)
- [Modules](#modules)
- [Usage](#usage)
- [Attribution](#attribution)

## Overview

BakBon is an lightweight infrastructure framework created to help configure, compose and build any type of message-driven distributed systems whether it is microservices, blockchain insfrastructure IoT networks, by providing core infastructure building blocks such as gateways, balancers, queues, caching systems.

### Features

- **Protocol-Agnostic**: works with TCP, UDP, HTTP(s), gRPC, MQTT, Serial, InProc or custom protocols.
- **Composable**: mix and match different components (Router, Queue, Gateway, Balancer, Cache, etc).
- **Type-Safe**: strong typing with runtime flexibility.
- **Production-Ready**: Comprehensive [tests](./tests/), clean architecture.

## Installation

## File System
```
📂 bakbon
    │
    ├── 📂 src
    │       │
    │       ├── 📂 balancer
    │       │       │
    │       │       ├── 📄 mod.rs
    │       │       └── 📄 strategy.rs
    │       │
    │       ├── 📂 cache.rs
    │       │       │
    │       │       ├── 📄 builder.rs
    │       │       ├── 📄 eviction.rs
    │       │       └── 📄 mod.rs
    │       │
    │       ├── 📂 core
    │       │       │
    │       │       ├── 📄 address.rs
    │       │       ├── 📄 error.rs
    │       │       ├── 📄 mod.rs
    │       │       └── 📄 protocol.rs
    │       │
    │       ├── 📂 infra
    │       │       │
    │       │       ├── 📄 gateway.rs
    │       │       ├── 📄 middleware.rs
    │       │       ├── 📄 mod.rs
    │       │       ├── 📄 processor.rs
    │       │       ├── 📄 service.rs
    │       │       └── 📄 storage.rs
    │       │
    │       ├── 📂 message
    │       │       │
    │       │       ├── 📄 envelope.rs
    │       │       ├── 📄 mod.rs
    │       │       └── 📄 route.rs
    │       │
    │       ├── 📂 queue
    │       │       │
    │       │       ├── 📂 attributes
    │       │       │       │
    │       │       │       ├── 📄 delivery.rs
    │       │       │       ├── 📄 durability.rs
    │       │       │       ├── 📄 mod.rs
    │       │       │       ├── 📄 ordering.rs
    │       │       │       └── 📄 provider.rs
    │       │       │
    │       │       ├── 📄 builder.rs
    │       │       └── 📄 mod.rs
    │       │
    │       ├── 📂 registry
    │       │       │
    │       │       ├── 📄 builder.rs
    │       │       └── 📄 mod.rs
    │       │
    │       ├── 📂 router
    │       │       │ 
    │       │       ├── 📄 builder.rs
    │       │       └── 📄 mod.rs
    │       │
    │       └── 📄 lib.rs
    │
    ├── 📂 tests
    │       │
    │       ├── 📂 common
    │       │       │
    │       │       ├── 📂 services
    │       │       │       │
    │       │       │       ├── 📄 echo.rs
    │       │       │       └── 📄 mod.rs
    │       │       │
    │       │       ├── 📄 gateway.rs
    │       │       └── 📄 mod.rs
    │       │
    │       ├── 📄 integration_gateway.rs
    │       ├── 📄 integration_queue.rs
    │       └── 📄 integration_router.rs
    │
    ├── ⚙️ Cargo.toml
    ├── 🔑 LICENSE
    └── 📖 README.md

    14 directories, 40 files
```

## Modules

BakBon provides:
- **Balancer**: Balancer.
- **Core**: Address, Protocol, Error, Result.
- **Gateway**: Gateway.
- **Infra**: Cache, Middleware.
- **Message**: Envelope, Route, Reply, Headers, Payload.
- **Queue**: Queue.
- **Registry**: Registry.
- **Router**: Router.
- **Service**: Service and Processor interfaces.

## Usage

```rust
use bakbon::*;

let url = "http://services.com/echo";

// Create an address from url
let address: Result<Address> = Address::parse(url);
assert!(address.is_ok());

// Create a service with the address
let service = EchoService::new(address.unwrap());

// Register the service while building a registry
let registry = Registry::builder()
    .register(service)
    .build();

// Build a router
let mut router = Router::builder()
    .registry(registry)
    .build();

// Create a message.
let message = Envelope::new(client_addr, url, bytes);

// Route the message to the appropriate service
let reply: Result<Reply> = router.route(message);
assert!(reply.is_ok());

```

## Attribution

If you use this project in your application, service, or research, please include the following credit:

> Based on **Bakbon** by Xn!l0 (https://github.com/Xnil0/backlab), licensed under the MIT License.