use std::path::Path;

use rusqlite::Connection;

use crate::{
    project::{Project, RunTarget, Source, Target},
    FuzzResult,
};

use super::{StorageBackend, StorageRequest, StorageResult};

pub struct SqliteBackend {
    connection: Connection,
}

impl SqliteBackend {
    pub fn new<F>(file: F) -> Self
    where
        F: AsRef<Path>,
    {
        Self {
            connection: Connection::open(file).unwrap(),
        }
    }

    fn handle(&self, req: StorageRequest) -> StorageResult {
        match req {
            StorageRequest::StoreResult {
                project_name,
                result,
            } => {
                self.connection
                            .execute(
                                "INSERT INTO results (pname, tname, input) VALUES (:pname, :tname, :data)",
                                rusqlite::named_params![":pname": project_name, ":tname": result.name, ":data": result.content],
                            )
                            .unwrap();

                StorageResult::Store
            }
            StorageRequest::LoadResults { project } => {
                let mut preped = self
                    .connection
                    .prepare("SELECT tname, input FROM results WHERE pname=:pname")
                    .unwrap();

                let results = preped
                    .query_map(rusqlite::named_params! { ":pname": project }, |row| {
                        let name: String = row.get("tname")?;
                        let input: Vec<u8> = row.get("input")?;

                        Ok(FuzzResult {
                            name,
                            content: input,
                        })
                    })
                    .unwrap()
                    .filter_map(|r| r.ok());

                StorageResult::LoadResults(results.collect())
            }
            StorageRequest::StoreProject(project) => {
                let src_str = serde_json::to_string(&project.source).unwrap();
                self.connection
                    .execute(
                        "INSERT OR REPLACE INTO projects (name, source) VALUES (:name, :source)",
                        rusqlite::named_params![":name": project.name, ":source": src_str],
                    )
                    .unwrap();

                StorageResult::StoreProject
            }
            StorageRequest::RemoveProject { name } => {
                self.connection
                    .execute(
                        "DELETE FROM projects WHERE name=:pname",
                        rusqlite::named_params! {":pname": name},
                    )
                    .unwrap();

                self.connection
                    .execute(
                        "DELETE FROM results WHERE pname=:pname",
                        rusqlite::named_params! {":pname": name},
                    )
                    .unwrap();

                self.connection
                    .execute(
                        "DELETE FROM targets WHERE pname=:pname",
                        rusqlite::named_params! {":pname": name},
                    )
                    .unwrap();

                StorageResult::RemoveProject
            }
            StorageRequest::LoadProjects => {
                let mut preped = self
                    .connection
                    .prepare("SELECT name, source FROM projects")
                    .unwrap();

                let mut preped_targets = self
                    .connection
                    .prepare("SELECT name, folder, target FROM targets WHERE pname=:pname")
                    .unwrap();

                let results = preped
                    .query_map([], |row| {
                        let name: String = row.get("name")?;
                        let raw_source: String = row.get("source")?;

                        let source: Source = serde_json::from_str(&raw_source).unwrap();

                        let targets = preped_targets
                            .query_map(rusqlite::named_params! { ":pname": name }, |row| {
                                let t_name: String = row.get("name")?;
                                let t_folder: String = row.get("folder")?;
                                let raw_t_target: String = row.get("target")?;

                                let t_target: RunTarget =
                                    serde_json::from_str(&raw_t_target).unwrap();

                                Ok(Target {
                                    name: t_name,
                                    folder: t_folder,
                                    target: t_target,
                                })
                            })
                            .unwrap()
                            .filter_map(|r| r.ok())
                            .collect();

                        Ok(Project {
                            name,
                            source,
                            targets,
                        })
                    })
                    .unwrap()
                    .filter_map(|r| r.ok());

                StorageResult::LoadProjects(results.collect())
            }
            StorageRequest::AddProjectTarget {
                project_name,
                target,
            } => {
                let target_str = serde_json::to_string(&target.target).unwrap();
                self.connection.execute(
                            "INSERT OR REPLACE INTO targets (pname, name, folder, target) VALUES (:pname, :target_name, :target_folder, :target_target)", 
                            rusqlite::named_params! { ":pname": project_name, ":target_name": target.name, ":target_folder": target.folder, ":target_target": target_str }).unwrap();

                StorageResult::AddProjectTarget
            }
        }
    }
}

impl StorageBackend for SqliteBackend {
    fn run(
        self,
        mut recv: tokio::sync::mpsc::Receiver<(
            super::StorageRequest,
            tokio::sync::oneshot::Sender<super::StorageResult>,
        )>,
    ) {
        self.connection
            .execute(
                "CREATE TABLE if not exists results (pname string, tname string, input binary)",
                [],
            )
            .expect("");
        self.connection
            .execute(
                "CREATE TABLE if not exists projects (name string primary key, source string)",
                [],
            )
            .expect("");
        self.connection.execute("CREATE TABLE if not exists targets (pname string, name string, folder string, target string, PRIMARY KEY (pname, name))", []).expect("");

        std::thread::spawn(move || loop {
            let (req, res_channel) = match recv.blocking_recv() {
                Some(d) => d,
                None => return,
            };

            let res = self.handle(req);

            if res_channel.send(res).is_err() {
                println!("Sending Result");
            }
        });
    }
}
