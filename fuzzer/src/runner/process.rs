use std::path::{Path, PathBuf};

use tokio::sync::oneshot;

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

                // TODO
                let _ = result;

                let run_path = repo_path.join(folder);
                let cleanup = move || {
                    std::fs::remove_dir_all(repo_path).unwrap();
                };

                (run_path, Box::new(cleanup))
            }
        }
    }

    fn run(
        &self,
        project_path: &Path,
        config: &config::Runner,
        mut cancel: oneshot::Receiver<()>,
    ) -> Option<Vec<Vec<u8>>> {
        match config {
            config::Runner::CargoFuzz { target } => {
                let artifacts_path = project_path.join("fuzz").join("artifacts").join(target);

                let output = std::process::Command::new("cargo")
                    .current_dir(project_path)
                    .arg("fuzz")
                    .arg("run")
                    .arg(target)
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn();

                let mut child = match output {
                    Ok(c) => c,
                    Err(_) => {
                        todo!()
                    }
                };

                loop {
                    // println!("Waiting for Child");

                    // If the child is done, we exit
                    if child.try_wait().unwrap().is_some() {
                        println!("Child Done");
                        break;
                    }
                    // If we received a signal to cancel the Run, we kill the Child and exit
                    if cancel.try_recv().is_ok() {
                        child.kill().unwrap();
                        return None;
                    }

                    // Otherwise we wait a second before polling again
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }

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
    fn run(&self, target: FuzzTarget, cancel: oneshot::Receiver<()>) -> Option<Vec<Vec<u8>>> {
        let (run_dir, cleanup) = self.setup(target.project_name(), target.name(), target.config());

        let result = self.run(&run_dir, target.runner(), cancel);

        cleanup();

        result
    }
}
