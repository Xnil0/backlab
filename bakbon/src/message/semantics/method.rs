#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    Create,
    Read,
    Update,
    Delete,
    Custom(String),
}

impl From<String> for Method {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "CREATE" => Self::Create,
            "READ" => Self::Read,
            "UPDATE" => Self::Update,
            "DELETE" => Self::Delete,
            _ => Self::Custom(value),
        }
    }
}

impl From<Method> for String {
    fn from(value: Method) -> Self {
        match value {
            Method::Create => "CREATE",
            Method::Read => "READ",
            Method::Update => "UPDATE",
            Method::Delete => "DELETE",
            Method::Custom(ref method) => method,
        }
        .to_string()
    }
}
