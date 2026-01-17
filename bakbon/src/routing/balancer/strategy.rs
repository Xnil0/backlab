use std::collections::HashMap;

pub(super) enum Strategy {
    RoundRobin {
        index: usize,
    },
    Weighted {
        index:   usize,
        weights: HashMap<String, u32>,
    },
    LeastConnections {
        connection_count: HashMap<String, u32>,
    },
    Random,
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
                weights: HashMap::new(),
            },
            "least_connections" => Self::LeastConnections {
                connection_count: HashMap::new(),
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
