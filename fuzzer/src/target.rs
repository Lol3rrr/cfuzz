

use crate::project::{Source, Target};

pub struct FuzzTarget {
    pname: String,
    name: String,
    runner: Target,
    config: Source,
}

impl FuzzTarget {
    pub fn new<PN, N>(pname: PN, name: N, runner: Target, config: Source) -> Self
    where
        PN: Into<String>,
        N: Into<String>,
    {
        Self {
            pname: pname.into(),
            name: name.into(),
            runner,
            config,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn project_name(&self) -> &str {
        &self.pname
    }
    pub fn config(&self) -> &Source {
        &self.config
    }
    pub fn runner(&self) -> &Target {
        &self.runner
    }
}
