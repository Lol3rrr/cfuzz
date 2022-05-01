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

fn storage_handler(
    connection: rusqlite::Connection,
    mut recv: mpsc::Receiver<(StorageRequest, oneshot::Sender<StorageResult>)>,
) {
    loop {
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

                if res_channel.send(StorageResult::Store).is_err() {
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

                if res_channel
                    .send(StorageResult::LoadResults(results.collect()))
                    .is_err()
                {
                    println!("Sending Result");
                }
            }
        };
    }
}

pub fn start() -> StorageHandle {
    let connection = rusqlite::Connection::open("./data.db").expect("");

    connection
        .execute(
            "CREATE TABLE if not exists results (name string, input binary)",
            [],
        )
        .expect("");

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

    pub async fn store_result(&self, data: FuzzResult) {
        self.request(StorageRequest::Store(data)).await.unwrap();
    }

    pub async fn load_results(&self) -> Vec<FuzzResult> {
        match self.request(StorageRequest::LoadResults).await.unwrap() {
            StorageResult::LoadResults(r) => r,
            _ => unreachable!(),
        }
    }
}
