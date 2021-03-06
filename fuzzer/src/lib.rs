use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use project::{Source, Target};
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;

mod target;
pub use target::FuzzTarget;

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
}

pub async fn run<R>(req: RunRequest, runner: Arc<R>, target: Target, source: Source)
where
    R: runner::Runner + Send + Sync + 'static,
{
    let pname = req.pname.clone();
    let name = req.name.clone();

    loop {
        let running_req = RunRequest {
            pname: req.pname.clone(),
            name: req.name.clone(),
        };

        let ftarget = FuzzTarget::new(
            running_req.pname,
            running_req.name,
            target.clone(),
            source.clone(),
        );
        let runner = runner.clone();

        {
            let state = STATE.get().unwrap();
            let mut running = state.running.try_lock().unwrap();
            running.insert(name.clone());
        }

        match crate::runner::run_completion(runner.clone(), ftarget).await {
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

        if !target.repeating {
            break;
        }
    }

    let state = STATE.get().unwrap();
    let mut running = state.running.try_lock().unwrap();

    running.remove(&name);
}
