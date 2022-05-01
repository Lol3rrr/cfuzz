use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use cfuzz::{
    project::{Project, Target},
    run, FuzzResult, RunRequest, State, STATE,
};
use serde::Deserialize;
use warp::Filter;

#[derive(Debug, Deserialize)]
struct CancelQuery {
    name: String,
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

    let targets_filter = warp::path!("api" / "targets").and(warp::get()).map(|| {
        let state = STATE.get().unwrap();
        let running = state.running.try_lock().unwrap();

        serde_json::to_string::<HashSet<String>>(&running).unwrap()
    });
    let results_filter = warp::path!("api" / "results")
        .and(warp::get())
        .and(warp::query())
        .then(|params: HashMap<String, String>| async move {
            let pname = match params.get("pname") {
                Some(n) => n,
                None => return "Missing Name".to_string(),
            };

            let state = STATE.get().unwrap();

            let results = state.store.load_results(pname.to_string()).await;

            let content = serde_json::to_string::<Vec<FuzzResult>>(results.as_ref()).unwrap();

            content
        });
    let start_filter = warp::path!("api" / "run")
        .and(warp::post())
        .and(warp::body::json())
        .map(|content: RunRequest| {
            tokio::spawn(run(content));

            ""
        });
    let cancel_filter = warp::path!("api" / "cancel")
        .and(warp::post())
        .and(warp::query())
        .map(|query: CancelQuery| {
            // TODO
            dbg!(&query);

            "TODO"
        });

    let update_project_filter = warp::path!("api" / "projects" / "update")
        .and(warp::post())
        .and(warp::body::json())
        .then(|proj: Project| async move {
            dbg!(&proj);

            let state = STATE.get().unwrap();
            state.store.update_project(proj).await;

            ""
        });
    let remove_project_filter = warp::path!("api" / "projects" / "remove")
        .and(warp::post())
        .and(warp::query())
        .then(|query: HashMap<String, String>| async move {
            let name = match query.get("pname") {
                Some(n) => n,
                None => return "Missing pname",
            };

            let state = STATE.get().unwrap();
            state.store.remove_project(name.to_string()).await;

            ""
        });
    let list_projects_filter = warp::path!("api" / "projects" / "list")
        .and(warp::get())
        .then(|| async move {
            let state = STATE.get().unwrap();
            let projects = state.store.load_projects().await;

            serde_json::to_string(&projects).unwrap()
        });
    let add_project_target = warp::path!("api" / "projects" / "targets" / "add")
        .and(warp::post())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::body::json::<Target>())
        .then(|query: HashMap<String, String>, target| async move {
            let name = match query.get("pname") {
                Some(n) => n,
                None => {
                    return "Missing pname";
                }
            };

            let state = STATE.get().unwrap();
            state
                .store
                .add_project_target(name.to_string(), target)
                .await;

            ""
        });

    let server = targets_filter
        .or(results_filter)
        .or(start_filter)
        .or(cancel_filter)
        .or(update_project_filter)
        .or(remove_project_filter)
        .or(list_projects_filter)
        .or(add_project_target)
        .with(
            warp::cors()
                .allow_origins(["http://192.168.178.22:5000"])
                .allow_methods(["GET", "POST", "FETCH"])
                .allow_credentials(true)
                .allow_headers(["content-type", "content-length"]),
        );
    warp::serve(server).run(([0, 0, 0, 0], 8080)).await;
}
