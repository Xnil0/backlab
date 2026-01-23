use crate::{
    Envelope,
    Result,
};

pub trait Middleware {
    fn intercept(&self, message: Envelope) -> Result<Envelope>;
}
