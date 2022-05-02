use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::{Runner, TargetConfig};

pub struct FuzzTarget {
    pname: String,
    name: String,
    runner: Runner,
    config: TargetConfig,
}

impl FuzzTarget {
    pub fn new<PN, N>(pname: PN, name: N, runner: Runner, config: TargetConfig) -> Self
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
    pub fn config(&self) -> &TargetConfig {
        &self.config
    }
    pub fn runner(&self) -> &Runner {
        &self.runner
    }

    pub fn setup(&self) -> Option<RunableTarget> {
        let base_path = Path::new("./fuzzing");
        let target_path = base_path.join(&self.name);
        std::fs::create_dir_all(&target_path).unwrap();

        match &self.config {
            TargetConfig::Git { repo, folder } => {
                let target_path_str = target_path.to_str().unwrap();
                let clone_output = Command::new("git")
                    .args(["clone", repo, target_path_str])
                    .output()
                    .unwrap();

                dbg!(std::str::from_utf8(&clone_output.stderr).unwrap());

                dbg!(target_path_str);
                assert!(clone_output.status.success());

                Some(RunableTarget {
                    folder: target_path.join(&folder),
                    runner: self.runner.clone(),
                    cleanup: Box::new(move || {
                        std::fs::remove_dir_all(target_path).unwrap();
                    }),
                })
            }
        }
    }
}

pub struct RunableTarget {
    folder: PathBuf,
    runner: Runner,
    cleanup: Box<dyn FnOnce() + Send>,
}

impl RunableTarget {
    pub fn run(self) -> Option<Vec<Vec<u8>>> {
        match self.runner {
            Runner::CargoFuzz { target } => {
                let artifacts_path = self.folder.join("fuzz").join("artifacts").join(&target);

                let output = Command::new("cargo")
                    .current_dir(&self.folder)
                    .args(["fuzz", "run", &target])
                    .output();

                match output {
                    Ok(_) => {
                        let files: Vec<_> = std::fs::read_dir(artifacts_path)
                            .unwrap()
                            .filter_map(|e| e.ok())
                            .filter(|entry| entry.file_type().unwrap().is_file())
                            .map(|entry| {
                                let e_path = entry.path();
                                std::fs::read(e_path).unwrap()
                            })
                            .collect();

                        (self.cleanup)();

                        Some(files)
                    }
                    Err(err) => {
                        dbg!(err);
                        todo!()
                    }
                }
            }
        }
    }
}
