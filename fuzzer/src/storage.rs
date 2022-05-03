use std::fmt::Debug;

use tokio::sync::{mpsc, oneshot};

use crate::{
    project::{Project, Target},
    FuzzResult,
};

pub mod sqlite;

/// A Storage-Backend that can be used to store all the Data generated and configured by the Program.
///
/// Having this abstraction allows for a wider variety of possible storage Solutions to fulfill different
/// Goals, like in memory for testing or sqlite for a simple deployment.
pub trait StorageBackend {
    /// This should run the StorageBackend meaning that it will receive Storage Requests over the given mpsc-Queue
    /// and should process them one by one and then send the Result back using the provided oneshot channel alongside
    /// each Request.
    ///
    /// This should not block and instead perform only the setup that needs to be done before being able to potentially
    /// serve requests and then move all the long running tasks (like handling the requests) to a background task/thread
    fn run(self, recv: mpsc::Receiver<(StorageRequest, oneshot::Sender<StorageResult>)>);
}

/// A Request for the Storage Backend
pub enum StorageRequest {
    /// The given result should be stored for the given Project
    StoreResult {
        /// The Project to which this result belongs
        project_name: String,
        /// The Result itself
        result: FuzzResult,
    },
    /// Should load the Results for the given Project
    LoadResults {
        /// The Project name
        project: String,
    },
    /// Should store/update the Project Configuration
    StoreProject(Project),
    /// Should remove the Project
    RemoveProject { name: String },
    /// Should load all configured Projects
    LoadProjects,
    /// Should attempt to load the Project with the given Name
    LoadProject {
        /// The Name of the Project to load
        name: String,
    },
    /// Add a new Target to a Project
    AddProjectTarget {
        project_name: String,
        target: Target,
    },
    /// Should attempt to load the Target with the given Name from the Project
    LoadTarget {
        /// The Name of the Project that the target belongs to
        project_name: String,
        /// The Name of the Target itself
        target_name: String,
    },
}

/// A Result returned by the Storage Backend for a Request
pub enum StorageResult {
    Store,
    LoadResults(Vec<FuzzResult>),
    StoreProject,
    RemoveProject,
    LoadProjects(Vec<Project>),
    LoadProject(Option<Project>),
    AddProjectTarget,
    LoadTarget(Option<Target>),
}

/// The Handle allows for easy interaction with a Storage Backend
pub struct StorageHandle {
    /// The Queue used for communicating with the Backend
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

    pub async fn load_project<N>(&self, name: N) -> Option<Project>
    where
        N: Into<String>,
    {
        match self
            .request(StorageRequest::LoadProject { name: name.into() })
            .await
            .unwrap()
        {
            StorageResult::LoadProject(d) => d,
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
