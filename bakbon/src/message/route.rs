use {
    super::address::Address,
    crate::MyResult,
};

pub struct Route {
    source:      Address,
    destination: Address,
}

impl Route {
    pub(super) fn new(source: impl Into<String>, destination: impl Into<String>) -> MyResult<Self> {
        Ok(Self {
            source:      Address::new(source.into())?,
            destination: Address::new(destination.into())?,
        })
    }

    pub fn source(&self) -> &Address { &self.source }

    pub fn destination(&self) -> &Address { &self.destination }
}

//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_route() {
        let source = "https://source.com/";
        let destination = "https://destination.com/";

        let route = Route::new(source, destination);
        assert!(route.is_ok());

        let route = route.unwrap();
        assert_eq!(route.source().to_string(), source);
        assert_eq!(
            route
                .destination()
                .to_string(),
            destination
        );
    }
}
