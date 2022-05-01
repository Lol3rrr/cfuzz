use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use serde::{Deserialize, Serialize};
use tokio::sync::{oneshot, OnceCell};

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
    #[serde(default)]
    repeating: bool,
}

pub async fn run(req: RunRequest) {
    let name = req.name.clone();
    dbg!(&req);

    loop {
        let (res_sender, res_recv) = tokio::sync::oneshot::channel();

        let running_req = RunRequest {
            name: req.name.clone(),
            runner: req.runner.clone(),
            config: req.config.clone(),
            repeating: req.repeating,
        };

        let target = FuzzTarget::new(running_req.name, running_req.runner, running_req.config);
        let runnable = target.setup().unwrap();

        {
            let state = STATE.get().unwrap();
            let mut running = state.running.try_lock().unwrap();
            running.insert(name.clone());
        }

        std::thread::spawn(move || {
            match runnable.run() {
                Some(r) => {
                    res_sender.send(r).unwrap();
                }
                None => {
                    println!("Error running Runnable");
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

        if !req.repeating {
            break;
        }
    }

    let state = STATE.get().unwrap();
    let mut running = state.running.try_lock().unwrap();

    running.remove(&name);
}
