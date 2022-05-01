use std::fmt::Debug;

use tokio::sync::{mpsc, oneshot};

use crate::{
    project::{Project, RunTarget, Source, Target},
    FuzzResult,
};

enum StorageRequest {
    StoreResult {
        project_name: String,
        result: FuzzResult,
    },
    LoadResults {
        project: String,
    },
    StoreProject(Project),
    RemoveProject {
        name: String,
    },
    LoadProjects,
    AddProjectTarget {
        project_name: String,
        target: Target,
    },
}

enum StorageResult {
    Store,
    LoadResults(Vec<FuzzResult>),
    StoreProject,
    RemoveProject,
    LoadProjects(Vec<Project>),
    AddProjectTarget,
}

pub struct StorageHandle {
    coms: mpsc::Sender<(StorageRequest, oneshot::Sender<StorageResult>)>,
}

impl Debug for StorageHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StorageHandle").finish()
    }
}

fn storage_handler(
    connection: rusqlite::Connection,
    mut recv: mpsc::Receiver<(StorageRequest, oneshot::Sender<StorageResult>)>,
) {
    loop {
        let (req, res_channel) = match recv.blocking_recv() {
            Some(d) => d,
            None => return,
        };

        let res = match req {
            StorageRequest::StoreResult {
                project_name,
                result,
            } => {
                connection
                    .execute(
                        "INSERT INTO results (pname, tname, input) VALUES (:pname, :tname, :data)",
                        rusqlite::named_params![":pname": project_name, ":tname": result.name, ":data": result.content],
                    )
                    .unwrap();

                StorageResult::Store
            }
            StorageRequest::LoadResults { project } => {
                let mut preped = connection
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
                connection
                    .execute(
                        "INSERT OR REPLACE INTO projects (name, source) VALUES (:name, :source)",
                        rusqlite::named_params![":name": project.name, ":source": src_str],
                    )
                    .unwrap();

                StorageResult::StoreProject
            }
            StorageRequest::RemoveProject { name } => {
                connection
                    .execute(
                        "DELETE FROM projects WHERE name=:pname",
                        rusqlite::named_params! {":pname": name},
                    )
                    .unwrap();

                connection
                    .execute(
                        "DELETE FROM results WHERE pname=:pname",
                        rusqlite::named_params! {":pname": name},
                    )
                    .unwrap();

                connection
                    .execute(
                        "DELETE FROM targets WHERE pname=:pname",
                        rusqlite::named_params! {":pname": name},
                    )
                    .unwrap();

                StorageResult::RemoveProject
            }
            StorageRequest::LoadProjects => {
                let mut preped = connection
                    .prepare("SELECT name, source FROM projects")
                    .unwrap();

                let mut preped_targets = connection
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
                connection.execute(
                    "INSERT OR REPLACE INTO targets (pname, name, folder, target) VALUES (:pname, :target_name, :target_folder, :target_target)", 
                    rusqlite::named_params! { ":pname": project_name, ":target_name": target.name, ":target_folder": target.folder, ":target_target": target_str }).unwrap();

                StorageResult::AddProjectTarget
            }
        };

        if res_channel.send(res).is_err() {
            println!("Sending Result");
        }
    }
}

pub fn start() -> StorageHandle {
    let connection = rusqlite::Connection::open("./data.db").expect("");

    connection
        .execute(
            "CREATE TABLE if not exists results (pname string, tname string, input binary)",
            [],
        )
        .expect("");
    connection
        .execute(
            "CREATE TABLE if not exists projects (name string primary key, source string)",
            [],
        )
        .expect("");
    connection.execute("CREATE TABLE if not exists targets (pname string, name string, folder string, target string, PRIMARY KEY (pname, name))", []).expect("");

    let (coms, recv) = mpsc::channel::<(StorageRequest, oneshot::Sender<StorageResult>)>(64);

    std::thread::spawn(move || storage_handler(connection, recv));

    StorageHandle { coms }
}

impl StorageHandle {
    async fn request(&self, req: StorageRequest) -> Option<StorageResult> {
        let (send, recv) = oneshot::channel();

        self.coms.send((req, send)).await.ok()?;

        recv.await.ok()
    }

    pub async fn store_result(&self, project: String, data: FuzzResult) {
        self.request(StorageRequest::StoreResult {
            project_name: project,
            result: data,
        })
        .await
        .unwrap();
    }

    pub async fn load_results(&self, project: String) -> Vec<FuzzResult> {
        match self
            .request(StorageRequest::LoadResults { project })
            .await
            .unwrap()
        {
            StorageResult::LoadResults(r) => r,
            _ => unreachable!(),
        }
    }

    pub async fn update_project(&self, project: Project) {
        match self
            .request(StorageRequest::StoreProject(project))
            .await
            .unwrap()
        {
            StorageResult::StoreProject => {}
            _ => unreachable!(),
        };
    }

    pub async fn remove_project(&self, name: String) {
        match self
            .request(StorageRequest::RemoveProject { name })
            .await
            .unwrap()
        {
            StorageResult::RemoveProject => {}
            _ => unreachable!(),
        };
    }

    pub async fn load_projects(&self) -> Vec<Project> {
        match self.request(StorageRequest::LoadProjects).await.unwrap() {
            StorageResult::LoadProjects(r) => r,
            _ => unreachable!(),
        }
    }

    pub async fn add_project_target(&self, pname: String, target: Target) {
        match self
            .request(StorageRequest::AddProjectTarget {
                project_name: pname,
                target,
            })
            .await
            .unwrap()
        {
            StorageResult::AddProjectTarget => {}
            _ => unreachable!(),
        }
    }
}
