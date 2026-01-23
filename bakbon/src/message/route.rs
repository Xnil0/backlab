use crate::Address;

#[derive(Debug)]
pub(super) struct Route {
    source:      Address,
    destination: String,
}

impl Route {
    pub(super) fn new(src: Address, dst: impl ToString) -> Self {
        Self {
            source:      src,
            destination: dst.to_string(),
        }
    }

    pub(super) fn source(&self) -> &Address { &self.source }

    pub(super) fn destination(&self) -> &str { &self.destination }
}

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {

    use {
        super::*,
        crate::Error,
    };

    #[test]
    fn new_route() {
        let source = Address::new("https://source.com/");
        assert!(source.is_ok());
        let source = source.unwrap();
        let source_str = source.to_string();

        let destination = "https://destination.com/";
        let route = Route::new(source, destination);

        assert_eq!(
            route.source(),
            &Address::new(source_str.as_str()).unwrap()
        );
        assert_eq!(
            route
                .destination()
                .to_string(),
            destination
        );
    }

    #[test]
    fn new_route_with_empty_source() {
        let source = Address::new("");
        assert!(source.is_err());
        assert!(matches!(
            source.unwrap_err(),
            Error::InvalidAddress
        ));
    }

    #[test]
    fn new_route_with_empty_destination() {
        let source = Address::new("https://source.com/");
        assert!(source.is_ok());
        let source = source.unwrap();
        let source_str = source.to_string();

        let destination = "";
        let route = Route::new(source, destination);

        assert_eq!(
            route.source(),
            &Address::new(source_str.as_str()).unwrap()
        );
        assert!(route.destination().is_empty(),);
    }
}
