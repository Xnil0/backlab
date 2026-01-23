mod strategy;

use {
    crate::Service,
    strategy::Strategy,
};

#[derive(Default)]
pub(super) struct Balancer(Strategy);

impl Balancer {
    pub(super) fn new(strategy: &str) -> Self { Self(strategy.into()) }

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

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            Address,
            Envelope,
            Reply,
            Result,
        },
    };

    #[derive(Debug, Clone)]
    struct NoService(Address);

    impl Service for NoService {
        fn address(&self) -> &Address { &self.0 }

        fn duplicate(&self) -> Box<dyn Service> { Box::new(self.clone()) }

        fn process(&self, _message: Envelope) -> Result<Reply> { Ok(None) }
    }

    #[test]
    fn default_balancer() {
        let balancer = Balancer::default();
        let round_robin = Strategy::RoundRobin { index: 0 };
        assert_eq!(balancer.0, round_robin);
        assert_eq!(balancer.strategy(), "round_robin");
    }

    #[test]
    fn new_balancer() {
        let strategy = "random";
        let balancer = Balancer::new(strategy);
        assert_eq!(balancer.0, Strategy::Random);
        assert_eq!(balancer.strategy(), "random");
    }

    #[test]
    fn balancer_select() -> Result<()> {
        let src1 = "http://no-service-1.com";
        let src2 = "http://no-service-2.com";
        let src3 = "http://no-service-3.com";

        let addr1 = Address::parse(src1)?;
        let addr2 = Address::parse(src2)?;
        let addr3 = Address::parse(src3)?;

        let srv1 = NoService(addr1);
        let srv2 = NoService(addr2);
        let srv3 = NoService(addr3);

        let instances: Vec<Box<dyn Service>> = vec![Box::new(srv1), Box::new(srv2), Box::new(srv3)];

        let mut balancer = Balancer::default();

        let selected = balancer.select(&instances);
        assert_eq!(selected.address().to_string(), src1);

        let selected = balancer.select(&instances);
        assert_eq!(selected.address().to_string(), src2);

        let selected = balancer.select(&instances);
        assert_eq!(selected.address().to_string(), src3);

        let selected = balancer.select(&instances);
        assert_eq!(selected.address().to_string(), src1);

        Ok(())
    }
}
