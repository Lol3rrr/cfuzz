use serde::{Deserialize, Serialize};

/// A single Project which could contain multiple Targets
#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    pub source: Source,
    pub targets: Vec<Target>,
}

/// A single Source for a Project
#[derive(Debug, Deserialize, Serialize)]
pub enum Source {
    /// A Git Repository as Source for the Project
    Git {
        /// The Repository link
        repo: String,
    },
}

/// A single Fuzzing Target for a Project
#[derive(Debug, Deserialize, Serialize)]
pub struct Target {
    /// The Name of the Target
    pub name: String,
    /// The Folder of the Project in which the Target exists
    pub folder: String,
    /// The actual Target to run
    pub target: RunTarget,
}

/// A single runnable Fuzzing Target that specifies how the Target should be fuzzed
#[derive(Debug, Deserialize, Serialize)]
pub enum RunTarget {
    /// The Cargo-Fuzz
    CargoFuzz {
        /// The Name of the fuzzing Target
        name: String,
    },
}