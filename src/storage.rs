use std::fmt::Debug;

use tokio::sync::{mpsc, oneshot};

use crate::FuzzResult;

enum StorageRequest {
    Store(FuzzResult),
    LoadResults,
}

enum StorageResult {
    Store,
    LoadResults(Vec<FuzzResult>),
}

pub struct StorageHandle {
    coms: mpsc::Sender<(StorageRequest, oneshot::Sender<StorageResult>)>,
}

impl Debug for StorageHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StorageHandle").finish()
    }
}

pub fn start() -> StorageHandle {
    let connection = rusqlite::Connection::open("./data.db").unwrap();

    connection
        .execute(
            "CREATE TABLE if not exists results (name string, input binary)",
            [],
        )
        .expect("");

    let (coms, mut recv) = mpsc::channel::<(StorageRequest, oneshot::Sender<StorageResult>)>(64);

    std::thread::spawn(move || loop {
        let (req, res_channel) = match recv.blocking_recv() {
            Some(d) => d,
            None => return,
        };

        match req {
            StorageRequest::Store(data) => {
                connection
                    .execute(
                        "INSERT INTO results (name, input) VALUES (:name, :data)",
                        rusqlite::params![data.name, data.content],
                    )
                    .unwrap();

                if let Err(_) = res_channel.send(StorageResult::Store) {
                    println!("Sending Result");
                }
            }
            StorageRequest::LoadResults => {
                let mut preped = connection
                    .prepare("SELECT name, input FROM results")
                    .unwrap();

                let results = preped
                    .query_map([], |row| {
                        let name: String = row.get("name")?;
                        let input: Vec<u8> = row.get("input")?;

                        Ok(FuzzResult {
                            name,
                            content: input,
                        })
                    })
                    .unwrap()
                    .filter_map(|r| r.ok());

                if let Err(_) = res_channel.send(StorageResult::LoadResults(results.collect())) {
                    println!("Sending Result");
                }
            }
        };
    });

    StorageHandle { coms }
}

impl StorageHandle {
    pub async fn store_result(&self, data: FuzzResult) {
        let (send, recv) = oneshot::channel();

        match self.coms.send((StorageRequest::Store(data), send)).await {
            Ok(_) => {}
            Err(_) => {
                todo!()
            }
        };

        recv.await.unwrap();
    }

    pub async fn load_results(&self) -> Vec<FuzzResult> {
        let (send, recv) = oneshot::channel();

        match self.coms.send((StorageRequest::LoadResults, send)).await {
            Ok(_) => {}
            Err(_) => {
                todo!()
            }
        };

        match recv.await.unwrap() {
            StorageResult::LoadResults(r) => r,
            _ => unreachable!(),
        }
    }
}
