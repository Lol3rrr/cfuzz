use std::{collections::HashSet, sync::Mutex};

use cfuzz::{run, FuzzResult, RunRequest, State, STATE};
use warp::Filter;

async fn load_results() -> String {
    let state = STATE.get().unwrap();

    let results = state.store.load_results().await;

    let content = serde_json::to_string::<Vec<FuzzResult>>(results.as_ref()).unwrap();

    content
}

#[tokio::main]
async fn main() {
    let storage_handle = cfuzz::storage::start();

    STATE
        .set(State {
            running: Mutex::new(HashSet::new()),
            store: storage_handle,
        })
        .expect("");

    let targets_filter = warp::path("targets").and(warp::get()).map(|| {
        let state = STATE.get().unwrap();
        let running = state.running.try_lock().unwrap();

        serde_json::to_string::<HashSet<String>>(&running).unwrap()
    });
    let results_filter = warp::path("results").and(warp::get()).then(load_results);
    let start_filter = warp::path("run")
        .and(warp::post())
        .and(warp::body::json())
        .map(|content: RunRequest| {
            let name = content.name.to_string();
            tokio::spawn(run(content));

            let state = STATE.get().unwrap();
            let mut running = state.running.try_lock().unwrap();
            running.insert(name);

            ""
        });

    let server = targets_filter.or(results_filter).or(start_filter);
    warp::serve(server).run(([0, 0, 0, 0], 8080)).await;
}
