use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;

mod target;
pub use target::{FuzzTarget, RunableTarget};

mod config;
pub use config::{Runner, TargetConfig};

pub mod project;

pub mod runner;
pub mod storage;

#[derive(Debug)]
pub struct State {
    pub running: Mutex<HashSet<String>>,
    pub store: storage::StorageHandle,
}

pub static STATE: OnceCell<State> = OnceCell::const_new();

#[derive(Debug, Serialize)]
pub struct FuzzResult {
    name: String,
    content: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub struct RunRequest {
    pub pname: String,
    pub name: String,
    runner: Runner,
    config: TargetConfig,
    #[serde(default)]
    repeating: bool,
}

pub async fn run<R>(req: RunRequest, runner: Arc<R>)
where
    R: runner::Runner + Send + Sync + 'static,
{
    let pname = req.pname.clone();
    let name = req.name.clone();
    dbg!(&req);

    loop {
        let running_req = RunRequest {
            pname: req.pname.clone(),
            name: req.name.clone(),
            runner: req.runner.clone(),
            config: req.config.clone(),
            repeating: req.repeating,
        };

        let target = FuzzTarget::new(
            running_req.pname,
            running_req.name,
            running_req.runner,
            running_req.config,
        );
        let runner = runner.clone();

        {
            let state = STATE.get().unwrap();
            let mut running = state.running.try_lock().unwrap();
            running.insert(name.clone());
        }

        match crate::runner::run_completion(runner.clone(), target).await {
            Some(r) => {
                let state = STATE.get().unwrap();

                for res in r {
                    state
                        .store
                        .store_result(
                            pname.clone(),
                            FuzzResult {
                                name: name.clone(),
                                content: res,
                            },
                        )
                        .await;
                }
            }
            None => {
                println!("Target Failed");
            }
        };

        if !req.repeating {
            break;
        }
    }

    let state = STATE.get().unwrap();
    let mut running = state.running.try_lock().unwrap();

    running.remove(&name);
}
