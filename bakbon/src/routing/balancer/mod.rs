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

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            Envelope,
            MyResult,
            Reply,
        },
    };

    #[derive(Debug, Clone)]
    struct NoService(String);

    impl Service for NoService {
        fn address(&self) -> &str { self.0.as_str() }

        fn duplicate(&self) -> Box<dyn Service> { Box::new(self.clone()) }

        fn process(&self, _message: Envelope) -> MyResult<Reply> { Ok(None) }
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
    fn balancer_select() {
        let srv1 = NoService("http://no-service-1.com".to_string());
        let srv2 = NoService("http://no-service-2.com".to_string());
        let srv3 = NoService("http://no-service-3.com".to_string());
        let instances: Vec<Box<dyn Service>> = vec![
            Box::new(srv1.clone()),
            Box::new(srv2.clone()),
            Box::new(srv3.clone()),
        ];

        let mut balancer = Balancer::default();

        let selected = balancer.select(&instances);
        assert_eq!(selected.address(), srv1.0);

        let selected = balancer.select(&instances);
        assert_eq!(selected.address(), srv2.0);

        let selected = balancer.select(&instances);
        assert_eq!(selected.address(), srv3.0);

        let selected = balancer.select(&instances);
        assert_eq!(selected.address(), srv1.0);
    }
}
