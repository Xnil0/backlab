use {
    crate::{
        MyErr,
        MyResult,
    },
    std::collections::HashMap,
};

type Weights = HashMap<String, u32>;
type Connections = HashMap<String, u32>;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Strategy {
    RoundRobin { index: usize },
    Weighted { index: usize, weights: Weights },
    LeastConnections { connections: Connections },
    Random,
}

impl Strategy {
    pub fn new(value: &str) -> Self { value.into() }

    pub fn index(&self) -> MyResult<usize> {
        match self {
            Self::RoundRobin { index } | Self::Weighted { index, .. } => Ok(*index),
            _ => Err(MyErr::WrongStrategy),
        }
    }

    pub fn weights(&self) -> MyResult<&Weights> {
        match self {
            Self::Weighted {
                weights, ..
            } => Ok(weights),
            _ => Err(MyErr::WrongStrategy),
        }
    }

    pub fn connection_count(&self) -> MyResult<&Connections> {
        match self {
            Self::LeastConnections {
                connections,
                ..
            } => Ok(connections),
            _ => Err(MyErr::WrongStrategy),
        }
    }
}

impl Default for Strategy {
    fn default() -> Self { Self::RoundRobin { index: 0 } }
}

impl From<&str> for Strategy {
    fn from(value: &str) -> Self {
        match value {
            "round_robin" => Self::RoundRobin { index: 0 },
            "weighted" => Self::Weighted {
                index:   0,
                weights: Weights::new(),
            },
            "least_connections" => Self::LeastConnections {
                connections: Connections::new(),
            },
            "random" => Self::Random,
            _ => Self::default(),
        }
    }
}

impl AsRef<str> for Strategy {
    fn as_ref(&self) -> &str {
        match self {
            Self::RoundRobin { .. } => "round_robin",
            Self::Weighted { .. } => "weighted",
            Self::LeastConnections { .. } => "least_connections",
            Self::Random => "random",
        }
    }
}

//  +------------+
//  | UNIT TESTS |
//  +------------+

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_strategy() {
        let strategy = Strategy::default();
        let round_robin = Strategy::RoundRobin { index: 0 };
        assert_eq!(strategy, round_robin);
        assert_eq!(strategy.as_ref(), "round_robin");

        let index = strategy.index();
        assert!(index.is_ok());
        assert_eq!(index.unwrap(), 0)
    }

    #[test]
    fn strategy_from_str() {
        let strategy_str = "random";
        let strategy = Strategy::from(strategy_str);
        assert_eq!(strategy, Strategy::Random);
        assert_eq!(strategy.as_ref(), strategy_str);
    }

    #[test]
    fn str_into_strategy() {
        let strategy_str = "random";
        let strategy: Strategy = strategy_str.into();
        assert_eq!(strategy, Strategy::Random);
        assert_eq!(strategy.as_ref(), strategy_str);
    }

    #[test]
    fn weighted_strategy() {
        let strategy_str = "weighted";
        let strategy = Strategy::from(strategy_str);
        assert_eq!(strategy.as_ref(), strategy_str);

        let weighted = Strategy::Weighted {
            index:   0,
            weights: Weights::new(),
        };
        assert_eq!(strategy, weighted);

        let index = strategy.index();
        assert!(index.is_ok());
        assert_eq!(index.unwrap(), 0);

        let weights = strategy.weights();
        assert!(weights.is_ok());
        assert!(weights.unwrap().is_empty());
    }

    #[test]
    fn least_connections_strategy() {
        let strategy_str = "least_connections";
        let strategy = Strategy::from(strategy_str);
        assert_eq!(strategy.as_ref(), strategy_str);

        let least_connections = Strategy::LeastConnections {
            connections: Connections::new(),
        };
        assert_eq!(strategy, least_connections);

        let connections = strategy.connection_count();
        assert!(connections.is_ok());
        assert!(
            connections
                .unwrap()
                .is_empty()
        );
    }
}
