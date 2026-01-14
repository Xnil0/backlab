use {
    super::endpoint::Endpoint,
    crate::MyResult,
};

pub struct Route {
    source:      Endpoint,
    destination: Endpoint,
}

impl Route {
    pub(super) fn new(source: impl Into<String>, destination: impl Into<String>) -> MyResult<Self> {
        Ok(Self {
            source:      Endpoint::new(source.into())?,
            destination: Endpoint::new(destination.into())?,
        })
    }

    pub fn source(&self) -> &Endpoint { &self.source }

    pub fn destination(&self) -> &Endpoint { &self.destination }
}

//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_route_from_str() {
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
