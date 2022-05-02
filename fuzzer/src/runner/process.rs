use std::path::{Path, PathBuf};

use crate::{config, FuzzTarget, TargetConfig};

use super::Runner;

pub struct ProcessRunner {
    subfolder: PathBuf,
}

impl ProcessRunner {
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            subfolder: path.into(),
        }
    }

    fn setup(
        &self,
        pname: &str,
        name: &str,
        source: &TargetConfig,
    ) -> (PathBuf, Box<dyn FnOnce()>) {
        let project_path = self.subfolder.join(pname);

        dbg!(name);

        match source {
            TargetConfig::Git { repo, folder } => {
                let repo_path = project_path.join(name);
                let repo_path_str = repo_path.to_str().unwrap();

                let result = std::process::Command::new("git")
                    .arg("clone")
                    .arg(repo)
                    .arg(repo_path_str)
                    .output()
                    .unwrap();

                dbg!(result);

                let run_path = repo_path.join(folder);
                let cleanup = move || {
                    std::fs::remove_dir_all(repo_path).unwrap();
                };

                (run_path, Box::new(cleanup))
            }
        }
    }

    fn run(&self, project_path: &Path, config: &config::Runner) -> Option<Vec<Vec<u8>>> {
        match config {
            config::Runner::CargoFuzz { target } => {
                let artifacts_path = project_path.join("fuzz").join("artifacts").join(target);

                dbg!(&project_path, &artifacts_path);

                let output = std::process::Command::new("cargo")
                    .current_dir(project_path)
                    .arg("fuzz")
                    .arg("run")
                    .arg(target)
                    .output();

                match output {
                    Ok(_) => {}
                    Err(_) => {
                        todo!()
                    }
                };

                let entry_iter = std::fs::read_dir(artifacts_path)
                    .ok()?
                    .filter_map(|e| e.ok())
                    .filter_map(|e| {
                        let file_type = e.file_type().ok()?;
                        if file_type.is_dir() {
                            return None;
                        }

                        Some(e.path())
                    });

                let results = entry_iter
                    .filter_map(|path| std::fs::read(path).ok())
                    .collect();

                Some(results)
            }
        }
    }
}

impl Runner for ProcessRunner {
    fn run(&self, target: FuzzTarget) -> Option<Vec<Vec<u8>>> {
        let (run_dir, cleanup) = self.setup(target.project_name(), target.name(), target.config());

        let result = self.run(&run_dir, target.runner());

        cleanup();

        result
    }
}
