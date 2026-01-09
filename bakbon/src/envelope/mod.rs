mod message;
mod route;

pub use message::Message;
use route::Route;

pub struct Envelope<P> {
    route:   Route,
    message: Message<P>,
}

impl<P> Envelope<P> {
    pub fn new(source: impl ToString, destination: impl ToString, message: Message<P>) -> Self {
        Self {
            route: Route::new(source, destination),
            message,
        }
    }

    pub fn message(&self) -> &Message<P> { &self.message }

    pub fn source(&self) -> &str { &self.route.source() }

    pub fn destination(&self) -> &str { &self.route.destination() }
}
