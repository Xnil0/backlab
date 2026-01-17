mod strategy;

use {
    crate::Service,
    strategy::Strategy,
};

#[derive(Default)]
pub(super) struct Balancer(Strategy);

impl Balancer {
    pub fn new(strategy: &str) -> Self { Self(strategy.into()) }

    pub fn strategy(&self) -> &str { self.0.as_ref() }

    pub fn select<'a>(&'a mut self, instances: &'a [Box<dyn Service>]) -> &'a Box<dyn Service> {
        match &mut self.0 {
            Strategy::RoundRobin { index } => {
                let service = &instances[*index % instances.len()];
                *index += 1;
                service
            }
            Strategy::Weighted { index, .. } => {
                // todo!("Implement the weighted logic");
                let service = &instances[*index % instances.len()];
                *index += 1;
                service
            }
            Strategy::LeastConnections { .. } => {
                // todo!("Implement least connection logic");
                &instances[0]
            }
            Strategy::Random => {
                // todo!("Implement random logic");
                &instances[0]
            }
        }
    }
}

