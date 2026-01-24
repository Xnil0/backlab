use crate::Address;

/// Internal routing information between two endpoints.
///
/// `Route` tracks the source and destination [`Address`]
/// of a message inside the system. It is used by
/// [`Envelope`](super::Envelope) to represent where a message comes from
/// and where it is going.
#[derive(Debug)]
pub(super) struct Route {
    source:      Address,
    destination: Address,
}

impl Route {
    /// Creates a new route between the given source and destination
    pub(super) fn new(src: Address, dst: Address) -> Self {
        Self {
            source:      src,
            destination: dst,
        }
    }

    /// Swaps the source and destination addresses.
    ///
    /// This is typically used when building a [`Reply`](super::Reply) so
    /// that the response travels back to the original back to the
    /// original sender.
    pub(super) fn swap_endpoints(&mut self) {
        std::mem::swap(&mut self.source, &mut self.destination);
    }

    /// Return the source [`Address`](crate::Address) reference of this
    /// route.
    pub(super) fn source(&self) -> &Address { &self.source }

    /// Return the destination [`Address`](crate::Address) reference of
    /// this route.
    pub(super) fn destination(&self) -> &Address { &self.destination }
}

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_route() {
        let source = Address::parse("https://source.com/");
        assert!(source.is_ok());
        let source = source.unwrap();

        let destination = Address::parse("https://destination.com/");
        assert!(destination.is_ok());
        let destination = destination.unwrap();

        let route = Route::new(source.clone(), destination.clone());

        assert_eq!(route.source(), &source);
        assert_eq!(route.destination(), &destination);
    }
}
