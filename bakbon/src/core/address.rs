use {
    super::Result,
    crate::{
        Error,
        core::Protocol,
    },
};

/// Uniform Resource Identifier for BakBon.
///
/// Parses and stores URI-like string with the format
/// 'scheme://authority/path?query#fragment' where:
///
/// - `scheme` is backed by [`Protocol`](crate::Protocol) (for example
///   `http`, `tcp`, `grpc`, `inproc`, etc.).
/// - `authority` is typically a host or logical service name.
/// - `path` is used as authority to format the services address.
///
/// Used in [`Gateway`](crate::Gateway), [`Service`](crate::Service),
/// [`Envelope`](crate::Envelope) for messages source and destination.
///
/// # Examples
///
/// ```rust
/// use bakbon::*;
///
/// let uri = "https://services.com/path/to/resource?id=123&name=test#section1";
/// let address: Result<Address> = Address::new(uri); // or uri.try_into()
/// assert!(address.is_ok());
///
/// let address = address.unwrap();
///
/// if let Protocol::Http { secure } = address.scheme() {
///     assert!(secure);
/// }
///
/// assert_eq!(address.scheme().as_ref(), "https");
/// assert_eq!(address.authority(), "services.com");
/// assert_eq!(address.path(), "/path/to/resource");
/// assert_eq!(address.query(), "?id=123&name=test");
/// assert_eq!(address.fragment(), "#section1");
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Address {
    scheme:    Protocol,
    authority: String,
    path:      Option<String>,
    query:     Option<String>,
    fragment:  Option<String>,
}

impl Address {
    pub fn new(uri: &str) -> Result<Self> { uri.try_into() }

    pub fn scheme(&self) -> &Protocol { &self.scheme }

    pub fn authority(&self) -> &str { &self.authority }

    pub fn path(&self) -> &str {
        self.path
            .as_deref()
            .unwrap_or_default()
    }

    pub fn query(&self) -> &str {
        self.query
            .as_deref()
            .unwrap_or_default()
    }

    pub fn fragment(&self) -> &str {
        self.fragment
            .as_deref()
            .unwrap_or_default()
    }
}

impl TryInto<Address> for &str {
    type Error = Error;

    fn try_into(self) -> Result<Address> {
        let (scheme, authority) = match self.split_once("://") {
            Some((proto, authority)) if !authority.is_empty() => (proto.into(), authority),
            _ => return Err(Error::InvalidAddress),
        };
        let (authority, fragment) = match authority.split_once("#") {
            Some((a, f)) => (a, Some(format!("#{f}"))),
            None => (authority, None),
        };
        let (authority, query) = match authority.split_once("?") {
            Some((a, q)) => (a, Some(format!("?{q}"))),
            None => (authority, None),
        };
        let (authority, path) = match authority.split_once("/") {
            Some((a, p)) => (a.to_string(), Some(format!("/{p}"))),
            None => (authority.to_string(), None),
        };
        Ok(Address {
            scheme,
            authority,
            path,
            query,
            fragment,
        })
    }
}

impl ToString for Address {
    fn to_string(&self) -> String {
        format!(
            "{}://{}{}{}{}",
            self.scheme,
            self.authority,
            self.path(),
            self.query(),
            self.fragment(),
        )
    }
}

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_address() {
        let uri = "tcp://url-address.com";
        let address: Result<Address> = Address::new(uri);
        assert!(address.is_ok());

        let address = address.unwrap();
        assert_eq!(address.scheme().as_ref(), "tcp");
        assert_eq!(address.authority(), "url-address.com");
        assert!(address.path().is_empty());
        assert_eq!(address.query(), "");
        assert_eq!(address.fragment(), "");
    }

    #[test]
    fn str_into_address() {
        let url = "https://address-url.com";
        let address: Result<Address> = url.try_into();
        assert!(address.is_ok());

        let address = address.unwrap();
        assert_eq!(address.authority, "address-url.com");
        assert!(address.path.is_none());
        assert!(address.query.is_none());
        assert!(address.fragment.is_none());
        assert_eq!(
            address.scheme,
            Protocol::Http {
                secure: true,
            }
        );
    }

    #[test]
    fn address_to_string() {
        let uri = "grpc://address-url.com";
        let address = Address::new(uri);
        assert!(address.is_ok());

        let address = address.unwrap();
        assert_eq!(address.to_string(), uri);
    }

    #[test]
    fn address_with_custom_protocol() {
        let uri = "mpsc://uri-address.custom";
        let address = Address::new(uri);
        assert!(address.is_ok());

        let address = address.unwrap();
        assert_eq!(address.scheme().as_ref(), "mpsc");
        assert_eq!(
            address.scheme,
            Protocol::Custom("mpsc".to_string())
        );
    }

    #[test]
    fn address_with_path() {
        let uri = "tcp://address-url.com/path";
        let address = Address::new(uri);
        assert!(address.is_ok());

        let address = address.unwrap();
        assert_eq!(address.path(), "/path");
        assert_eq!(address.path, Some("/path".to_string()));
    }

    #[test]
    fn address_with_query() {
        let uri = "tcp://address-url.com/path?query=param";
        let address = Address::new(uri);
        assert!(address.is_ok());

        let address = address.unwrap();
        let query = "?query=param";
        assert_eq!(address.query(), query);
        assert_eq!(address.query, Some(query.to_string()));
    }

    #[test]
    fn address_with_fragment() {
        let uri = "tcp://address-url.com/path#fragment";
        let address = Address::new(uri);
        assert!(address.is_ok());

        let address = address.unwrap();
        let fragment = "#fragment";
        assert_eq!(address.fragment(), fragment);
        assert_eq!(address.fragment, Some(fragment.to_string()));
    }

    #[test]
    fn invalid_address() {
        let uri = "clearly_invalid";
        let address = Address::new(uri);
        assert!(address.is_err());
    }

    #[test]
    fn empty_address() {
        let address = Address::new("");
        assert!(address.is_err());
    }

    #[test]
    fn incomplete_address() {
        let uri = "grpc://";
        let address = Address::new(uri);
        assert!(address.is_err());
    }

    #[test]
    fn complete_address_without_path() {
        let uri = "tcp://authority.com?query=param#fragment";
        let address = Address::new(uri);
        assert!(address.is_ok());

        let address = address.unwrap();
        assert_eq!(address.scheme().as_ref(), "tcp");
        assert_eq!(address.authority(), "authority.com");
        assert_eq!(address.path(), "");
        assert_eq!(address.query(), "?query=param");
        assert_eq!(address.fragment(), "#fragment");
    }

    #[test]
    fn complete_address_without_query() {
        let uri = "http://authority.com/path#fragment";
        let address = Address::new(uri);
        assert!(address.is_ok());

        let address = address.unwrap();
        assert_eq!(address.scheme().as_ref(), "http");
        assert_eq!(address.authority(), "authority.com");
        assert_eq!(address.path(), "/path");
        assert_eq!(address.query(), "");
        assert_eq!(address.fragment(), "#fragment");
    }

    #[test]
    fn complete_address_without_fragment() {
        let uri = "grpc://authority.com/path?query=param";
        let address = Address::new(uri);
        assert!(address.is_ok());

        let address = address.unwrap();
        assert_eq!(address.scheme().as_ref(), "grpc");
        assert_eq!(address.authority(), "authority.com");
        assert_eq!(address.path(), "/path");
        assert_eq!(address.query(), "?query=param");
        assert_eq!(address.fragment(), "");
    }
}
