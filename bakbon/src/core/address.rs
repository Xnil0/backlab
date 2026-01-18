use {
    super::MyResult,
    crate::{
        MyErr,
        core::Protocol,
    },
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Address {
    scheme:    Protocol,
    authority: String,
    path:      String,
    query:     Option<String>,
    fragment:  Option<String>,
}

impl Address {
    pub fn new(uri: &str) -> MyResult<Self> { uri.try_into() }

    pub fn scheme(&self) -> &str { &self.scheme.as_ref() }

    pub fn authority(&self) -> &str { &self.authority }

    pub fn path(&self) -> &str { &self.path }

    pub fn query(&self) -> &Option<String> { &self.query }

    pub fn fragment(&self) -> &Option<String> { &self.fragment }
}

impl TryInto<Address> for &str {
    type Error = MyErr;

    fn try_into(self) -> MyResult<Address> {
        let (scheme, remains) = match self.split_once("://") {
            Some((proto, rest)) if !rest.is_empty() => (proto.into(), rest),
            _ => return Err(MyErr::InvalidAddress),
        };
        let (authority, path) = match remains.split_once("/") {
            Some((host, path)) => (host.to_string(), format!("/{}", path)),
            None => (remains.to_string(), String::new()),
        };
        let query = match path.split_once("?") {
            Some((_, q)) => Some(format!("?{q}")),
            None => None,
        };
        let fragment = match path.split_once("#") {
            Some((_, f)) => Some(format!("#{f}")),
            None => None,
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
        let query = self
            .query
            .as_ref()
            .map(|q| q.as_str())
            .unwrap_or_default();

        let fragment = self
            .fragment
            .as_ref()
            .map(|f| f.as_str())
            .unwrap_or_default();

        format!(
            "{}://{}{}{}{}",
            self.scheme, self.authority, self.path, query, fragment,
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
        let address: MyResult<Address> = Address::new(uri);
        assert!(address.is_ok());

        let address = address.unwrap();
        assert_eq!(address.scheme(), "tcp");
        assert_eq!(address.authority(), "url-address.com");
        assert!(address.path().is_empty());
        assert_eq!(address.query(), &None);
        assert_eq!(address.fragment(), &None);
    }

    #[test]
    fn str_into_address() {
        let url = "https://address-url.com";
        let address: MyResult<Address> = url.try_into();
        assert!(address.is_ok());

        let address = address.unwrap();
        assert_eq!(address.authority, "address-url.com");
        assert!(address.path.is_empty());
        assert_eq!(address.query, None);
        assert_eq!(address.fragment, None);
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
        assert_eq!(address.scheme(), "mpsc");
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
        assert_eq!(address.path, "/path".to_string());
    }

    #[test]
    fn address_with_query() {
        let uri = "tcp://address-url.com/path?query=param";
        let address = Address::new(uri);
        assert!(address.is_ok());

        let address = address.unwrap();
        let query = "?query=param";
        assert_eq!(address.query(), &Some(query.to_string()));
        assert_eq!(address.query, Some(query.to_string()));
    }

    #[test]
    fn address_with_fragment() {
        let uri = "tcp://address-url.com/path#fragment";
        let address = Address::new(uri);
        assert!(address.is_ok());

        let address = address.unwrap();
        let fragment = "#fragment";
        assert_eq!(
            address.fragment(),
            &Some(fragment.to_string())
        );
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
}
