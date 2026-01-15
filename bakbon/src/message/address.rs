use {
    super::MyResult,
    crate::{
        MyErr,
        protocol::Protocol,
    },
};

pub struct Address {
    scheme:    Protocol,
    authority: String,
    path:      String,
    query:     Option<String>,
    fragment:  Option<String>,
}

impl Address {
    pub fn new(uri: impl ToString) -> MyResult<Self> { Self::try_from(uri.to_string()) }

    pub fn scheme(&self) -> &Protocol { &self.scheme }

    pub fn authority(&self) -> &str { &self.authority }

    pub fn path(&self) -> &str { &self.path }

    pub fn query(&self) -> &Option<String> { &self.query }

    pub fn fragment(&self) -> &Option<String> { &self.fragment }
}

impl TryFrom<String> for Address {
    type Error = MyErr;

    fn try_from(value: String) -> MyResult<Self> {
        let (scheme, remains) = match value.split_once("://") {
            Some(splitted) => (Protocol::from(splitted.0), splitted.1),
            None => return Err(MyErr::InvalidAddress),
        };

        let (authority, path) = match remains.split_once("/") {
            Some(splitted) => (
                splitted.0.to_string(),
                format!("/{}", splitted.1),
            ),
            None => (remains.to_string(), String::new()),
        };

        let query = match path.split_once("?") {
            Some(splitted) => Some(format!("?{}", splitted.1)),
            None => None,
        };

        let fragment = match path.split_once("#") {
            Some(splitted) => Some(format!("#{}", splitted.1)),
            None => None,
        };

        Ok(Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_address() {
        let url = "https://address-url.com";
        let address = Address::new(url);
        assert!(address.is_ok());

        let address = address.unwrap();
        assert_eq!(address.scheme, Protocol::Https);
        assert_eq!(address.authority, "address-url.com");
        assert_eq!(address.path, "");
        assert_eq!(address.query, None);
        assert_eq!(address.fragment, None);
        assert_eq!(address.to_string(), url);
    }
}
