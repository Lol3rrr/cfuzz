use std::fmt::Debug;

use tokio::sync::{mpsc, oneshot};

use crate::{
    project::{Project, Target},
    FuzzResult,
};

pub mod sqlite;

pub trait StorageBackend {
    fn run(self, recv: mpsc::Receiver<(StorageRequest, oneshot::Sender<StorageResult>)>);
}

pub enum StorageRequest {
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

pub enum StorageResult {
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

pub fn start<S>(backend: S) -> StorageHandle
where
    S: StorageBackend,
{
    let (coms, recv) = mpsc::channel::<(StorageRequest, oneshot::Sender<StorageResult>)>(64);

    backend.run(recv);

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
