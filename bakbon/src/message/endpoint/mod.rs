mod scheme;

use {
    crate::{
        MyErr,
        MyResult,
    },
    scheme::Scheme,
};

pub struct Endpoint {
    scheme:    Scheme,
    authority: String,
    path:      String,
    query:     Option<String>,
    fragment:  Option<String>,
}

impl Endpoint {
    pub fn new(uri: impl ToString) -> MyResult<Self> { Self::try_from(uri.to_string()) }

    pub fn scheme(&self) -> &Scheme { &self.scheme }

    pub fn authority(&self) -> &str { &self.authority }

    pub fn path(&self) -> &str { &self.path }

    pub fn query(&self) -> &Option<String> { &self.query }

    pub fn fragment(&self) -> &Option<String> { &self.fragment }
}

impl TryFrom<String> for Endpoint {
    type Error = MyErr;

    fn try_from(value: String) -> MyResult<Self> {
        let (scheme, remains) = match value.split_once("://") {
            Some(splitted) => (Scheme::from(splitted.0), splitted.1),
            None => return Err(MyErr::InvalidEndpoint),
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

impl ToString for Endpoint {
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
    fn new_endpoint_from_str() {
        let url = "https://endpoint-url.com";
        let endpoint = Endpoint::new(url);
        assert!(endpoint.is_ok());

        let endpoint = endpoint.unwrap();
        assert_eq!(endpoint.scheme, Scheme::Https);
        assert_eq!(endpoint.authority, "endpoint-url.com");
        assert_eq!(endpoint.path, "");
        assert_eq!(endpoint.query, None);
        assert_eq!(endpoint.fragment, None);
        assert_eq!(endpoint.to_string(), url);
    }
}
