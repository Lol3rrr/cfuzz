use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum Runner {
    CargoFuzz { target: String },
}

#[derive(Debug, Deserialize)]
pub enum TargetConfig {
    Git { repo: String, folder: String },
}
