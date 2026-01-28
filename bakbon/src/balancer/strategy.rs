use {
    crate::{
        Error,
        Result,
    },
    std::collections::HashMap,
};

/// Per-service-instance weights used by the `Weighted` strategy.
///
/// `Keys` identify instances by name (like in the
/// [`Registry`](crate::Registry)), and `values` represent relative
/// weights.
type Weights = HashMap<String, u32>;

/// Per-service-instance connection count used by the `LeastConnections`
/// strategy.
///
/// `Keys` identify instances by name (like in the
/// [`Registry`](crate::Registry)), and `values` track the
/// number of active connections.
type Connections = HashMap<String, u32>;

/// Load balancing strategies supported by the
/// [`Balancer`](super::Balancer).
///
/// Each variant represents a different way to select a service instance
/// from a pool of registred instances:
///
/// - [`RoundRobin`](Strategy::RoundRobin) cycles through instances in
///   order.
/// - [`Weighted`](Strategy::Weighted) selects instances based on
///   configured weights.
/// - [`LeastConnections`](Strategy::LeastConnections) prefers instances
///   with fewer active connections.
/// - [`Random`](Strategy::Random) chooses a random instance.
///
/// Only [`RoundRobin`](Strategy::RoundRobin) is fully implemented to this
/// day; other strategies are placeholders for future development and
/// currently behave like simple round-robin or fixed selection in the
/// [`Balancer`](super::Balancer).
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Strategy {
    // Round-robin selection with internal index.
    RoundRobin { index: usize },

    // Weighted selection with internal index and per-instance weight map.
    Weighted { index: usize, weights: Weights },

    // Selection based on the number of active connections.
    LeastConnections { connections: Connections },

    // Random instance selection.
    Random,
}

#[allow(unused)]
impl Strategy {
    /// Returns a new strategy instance based on its string representation.
    ///
    /// Equivalent to `Strategy::from(value)`.
    pub fn new(value: &str) -> Self { value.into() }

    /// Returns the internal index for strategies that track one.
    ///
    /// Only valid for [`Strategy::RoundRobin`] and [`Strategy::Weighted`];
    /// calling it on other strategies will return
    /// [`Error::WrongStrategy`].
    pub fn index(&self) -> Result<usize> {
        match self {
            Self::RoundRobin { index } | Self::Weighted { index, .. } => Ok(*index),
            _ => Err(Error::WrongStrategy),
        }
    }

    /// Returns the weight map for [`Strategy::Weighted`].
    ///
    /// Calling it on another variant will return [`Error::WrongStrategy`].
    pub fn weights(&self) -> Result<&Weights> {
        match self {
            Self::Weighted {
                weights, ..
            } => Ok(weights),
            _ => Err(Error::WrongStrategy),
        }
    }

    /// Returns the connetion count map for [`Strategy::LeastConnections`].
    ///
    /// Calling it on another variant will return [`Error::WrongStrategy`].
    pub fn connection_count(&self) -> Result<&Connections> {
        match self {
            Self::LeastConnections {
                connections,
                ..
            } => Ok(connections),
            _ => Err(Error::WrongStrategy),
        }
    }
}

impl Default for Strategy {
    /// Returns the default strategy (`round_robin`).
    fn default() -> Self { Self::RoundRobin { index: 0 } }
}

impl From<&str> for Strategy {
    /// Parse a strategy name into a [`Strategy`] value.
    ///
    /// Recognized names:
    /// - "round_robin"
    /// - "weighted"
    /// - "least_connections"
    /// - "random"
    ///
    /// Any unrecognized name falls back to the default strategy
    /// (`round_robin`).
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
    /// Returns the string representation of the strategy.
    ///
    /// Values match what [`From<&str>`] accept (e.g. "round_robin",
    /// "weighted").
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
