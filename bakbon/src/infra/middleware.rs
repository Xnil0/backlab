use crate::{
    Envelope,
    MyResult,
};

pub trait Middleware {
    fn intercept(&self, message: Envelope) -> MyResult<Envelope>;
}
