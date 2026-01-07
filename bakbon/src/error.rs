pub type MyResult<T> = Result<T, MyErr>;

pub enum MyErr {
    NoCorrelationId,
    NoCausationId,
    NoTraceId,
    NoSpanId,
    NoTenantId,

    NoPath,
    NoMethod,
    NoIntent,
    InvalidMessageKind,
}
