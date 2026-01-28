use {
    super::{
        Headers,
        route::Route,
    },
    crate::{
        Address,
        Payload,
    },
};

/// Application-level message wrapper with [`Headers`],
/// [`Route`] and [`Payload`].
///
/// An `Envelope` is the core message unit exchanged between components
/// such as [`Gateway`](crate::Gateway), [`Router`](crate::Router),
/// [`Queue`](crate::Queue), and [`Service`](crate::Service). It bundles:
///
/// - [`Headers`] for metadata,
/// - a [`Route`] with source and destination [`Address`]es,
/// - a raw bytes [`Payload`]
#[derive(Debug)]
pub struct Envelope {
    headers: Headers,
    route:   Route,
    payload: Payload,
}

impl Envelope {
    /// Creates a new `Envelope` from source, destination, and payload.
    ///
    /// The [`Payload`] can be empty [`Payload::new()`] to
    /// represent a message without body.
    pub fn new(src: Address, dst: Address, payload: Payload) -> Self {
        Self {
            headers: Headers::default(),
            route: Route::new(src, dst),
            payload,
        }
    }

    /// Add a header by value and return the updated `Envelope`.
    ///
    /// This is convenient for builder-style construction:
    ///
    /// ```ignore
    /// let msg = Envelope::new(src, dst, payload)
    ///     .header("content-type", "application/json")
    ///     .header("encoding", "utf-8");
    /// ```
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers
            .insert(key.to_string(), value.to_string());
        self
    }

    /// Inserts or overrides a header in-place.
    ///
    /// To use when a mutable `Envelope` is already built.
    ///
    /// ```ignore
    /// let msg = Envelope::new(src, dst, payload);
    /// msg.add_header("content-type", "application/json");
    /// msg.add_header("encoding", "utf-8");
    /// ```
    pub fn add_header(&mut self, k: &str, v: &str) {
        self.headers
            .insert(k.to_string(), v.to_string());
    }

    /// Returns the value of a header given the key, if it exists.
    pub fn get_header(&self, key: &str) -> Option<&str> {
        self.headers
            .get(key)
            .map(|v| v.as_str())
    }

    /// Converts this `Envelope` into a [`Reply`](crate::Reply) message
    /// with a new [`Payload`].
    ///
    /// This swaps the source and the destination so that the
    /// [`Reply`](crate::Reply) is routed back to the original sender,
    /// and replace the [`Payload`] with the provided bytes.
    /// Existed [`Headers`] are preserved.
    /// Typical usage is inside
    /// [`Service::process()`](crate::Service::process) and/or
    /// [`Processor::execute()`](crate::Processor::execute) before
    /// returning.

    pub fn into_reply(mut self, payload: Payload) -> Self {
        self.route.swap_endpoints();
        self.payload = payload;
        self
    }

    /// Returns the reference to the raw [`Payload`] bytes.
    pub fn payload(&self) -> &Payload { &self.payload }

    /// Returns the reference to the source [`Address`] of the `Envelope`.
    pub fn source(&self) -> &Address { &self.route.source() }

    /// Returns the reference to the `Envelope`'s destination [`Address`].
    pub fn destination(&self) -> &Address { &self.route.destination() }
}

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {
    use super::*;

    const SRC: &str = "https://source.com";
    const DST: &str = "https://destination.com";

    #[test]
    fn new_message_with_payload() {
        let src = Address::parse(SRC);
        assert!(src.is_ok());
        let src = src.unwrap();

        let dst = Address::parse(DST);
        assert!(dst.is_ok());
        let dst = dst.unwrap();

        let payload = Payload::from("random_payload");
        let msg = Envelope::new(src, dst, payload.clone());

        assert!(!msg.payload().is_empty());
        assert_eq!(msg.payload(), &payload);
    }

    #[test]
    fn new_message_with_empty_payload() {
        let src = Address::parse(SRC);
        assert!(src.is_ok());
        let src = src.unwrap();

        let dst = Address::parse(DST);
        assert!(dst.is_ok());
        let dst = dst.unwrap();

        let payload = Payload::new();
        let msg = Envelope::new(src, dst, payload);

        assert!(msg.payload().is_empty());
    }

    #[test]
    fn new_message_with_headers() {
        let src = Address::parse(SRC);
        assert!(src.is_ok());
        let src = src.unwrap();

        let dst = Address::parse(DST);
        assert!(dst.is_ok());
        let dst = dst.unwrap();

        let payload = Payload::from("random_payload");

        let msg = Envelope::new(src, dst, payload)
            .header("content-type", "text/plain")
            .header("encoding", "utf-8");

        let content_type = msg.get_header("content-type");
        assert!(content_type.is_some());
        assert_eq!(content_type.unwrap(), "text/plain");

        let encoding = msg.get_header("encoding");
        assert!(encoding.is_some());
        assert_eq!(encoding.unwrap(), "utf-8");
    }

    #[test]
    fn copy_headers() {
        let src = Address::parse(SRC);
        assert!(src.is_ok());
        let src = src.unwrap();

        let dst = Address::parse(DST);
        assert!(dst.is_ok());
        let dst = dst.unwrap();

        let payload = Payload::from("random_payload");

        let msg = Envelope::new(src.clone(), dst.clone(), payload.clone())
            .header("content-type", "text/plain")
            .header("encoding", "utf-8");

        let reply = msg.into_reply(payload);

        let content_type = reply.get_header("content-type");
        assert!(content_type.is_some());
        assert_eq!(content_type.unwrap(), "text/plain");

        let encoding = reply.get_header("encoding");
        assert!(encoding.is_some());
        assert_eq!(encoding.unwrap(), "utf-8");
    }
}
