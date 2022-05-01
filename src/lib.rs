use std::{collections::HashSet, sync::Mutex};

use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;

mod target;
pub use target::{FuzzTarget, RunableTarget};

mod config;
pub use config::{Runner, TargetConfig};

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
    pub name: String,
    runner: Runner,
    config: TargetConfig,
}

pub async fn run(req: RunRequest) {
    let name = req.name.clone();
    let (res_sender, res_recv) = tokio::sync::oneshot::channel();

    std::thread::spawn(move || {
        let target = FuzzTarget::new(req.name, req.runner, req.config);
        let runnable = target.setup().unwrap();

        match runnable.run() {
            Some(r) => {
                res_sender.send(r).unwrap();
            }
            None => {
                todo!()
            }
        };
    });

    match res_recv.await {
        Ok(r) => {
            let state = STATE.get().unwrap();

            for res in r {
                state
                    .store
                    .store_result(FuzzResult {
                        name: name.clone(),
                        content: res,
                    })
                    .await;
            }
        }
        Err(e) => {
            dbg!(e);
            todo!()
        }
    };

    let state = STATE.get().unwrap();
    let mut running = state.running.try_lock().unwrap();

    running.remove(&name);
}
