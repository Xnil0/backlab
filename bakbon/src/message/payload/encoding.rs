#[derive(Debug, PartialEq, Eq)]
pub enum Encoding {
    Utf8,
    Utf16,
    Binary,
    Gzip,
    Custom(String),
}

impl Default for Encoding {
    fn default() -> Self { Self::Utf8 }
}
