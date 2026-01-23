use crate::Address;

#[derive(Debug)]
pub(super) struct Route {
    source:      Address,
    destination: Address,
}

impl Route {
    pub(super) fn new(src: Address, dst: Address) -> Self {
        Self {
            source:      src,
            destination: dst,
        }
    }

    pub(super) fn swap_endpoints(&mut self) {
        std::mem::swap(&mut self.source, &mut self.destination);
    }

    pub(super) fn source(&self) -> &Address { &self.source }

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
