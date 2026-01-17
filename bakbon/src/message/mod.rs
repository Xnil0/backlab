mod envelope;

use std::collections::HashMap;

pub use envelope::Envelope;

pub type Reply = Option<Envelope>;
pub type Headers = HashMap<String, String>;

struct Route {
    source:      String,
    destination: String,
}

impl Route {
    pub(super) fn new(source: impl ToString, destination: impl ToString) -> Self {
        Self {
            source:      source.to_string(),
            destination: destination.to_string(),
        }
    }

    pub fn source(&self) -> &str { &self.source }

    pub fn destination(&self) -> &str { &self.destination }
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
}
