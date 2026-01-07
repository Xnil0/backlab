#[derive(Debug, PartialEq, Eq)]
pub enum ContentType {
    Json,
    Text,
    Html,
    Bincode,
    Protobuf,
    Avro,
    Custom(String),
}

impl Default for ContentType {
    fn default() -> Self { Self::Json }
}
