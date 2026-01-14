use crate::{
    Envelope,
    MyResult,
};

pub trait Router {
    fn route(&self, message: Envelope) -> MyResult<()>;
}
