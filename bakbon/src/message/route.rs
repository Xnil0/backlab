#[derive(Debug, Clone)]
pub(super) struct Route {
    source:      String,
    destination: String,
}

impl Route {
    pub(super) fn new(src: impl ToString, dst: impl ToString) -> Self {
        Self {
            source:      src.to_string(),
            destination: dst.to_string(),
        }
    }

    pub(super) fn source(&self) -> &str { &self.source }

    pub(super) fn destination(&self) -> &str { &self.destination }
}

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_route() {
        let source = "https://source.com/";
        let destination = "https://destination.com/";
        let route = Route::new(source, destination);

        assert_eq!(route.source().to_string(), source);
        assert_eq!(
            route
                .destination()
                .to_string(),
            destination
        );
    }

    #[test]
    fn new_route_with_empty_source() {
        let source = "";
        let destination = "https://destination.com/";
        let route = Route::new(source, destination);

        assert_eq!(route.source().to_string(), source);
        assert_eq!(
            route
                .destination()
                .to_string(),
            destination
        );
    }

    #[test]
    fn new_route_with_empty_destination() {
        let source = "https://source.com/";
        let destination = "";
        let route = Route::new(source, destination);

        assert_eq!(route.source().to_string(), source);
        assert_eq!(
            route
                .destination()
                .to_string(),
            destination
        );
    }
}
