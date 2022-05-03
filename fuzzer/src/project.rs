use serde::{Deserialize, Serialize};

/// A single Project which could contain multiple Targets
#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    /// The Name of the Project
    pub name: String,
    /// The Source of the Projects Code
    pub source: Source,
    /// The Fuzzing Targets of the Project
    pub targets: Vec<Target>,
}

/// A single Source for a Project
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Source {
    /// A Git Repository as Source for the Project
    Git {
        /// The Repository link
        repo: String,
    },
}

/// A single Fuzzing Target for a Project
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Target {
    /// The Name of the Target
    pub name: String,
    /// The Folder of the Project in which the Target exists
    pub folder: String,
    /// The actual Target to run
    pub target: RunTarget,
    /// If the Target should be executed in a loop or only once
    pub repeating: bool,
}

/// A single runnable Fuzzing Target that specifies how the Target should be fuzzed
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RunTarget {
    /// The Cargo-Fuzz
    CargoFuzz {
        /// The Name of the fuzzing Target
        name: String,
    },
}
