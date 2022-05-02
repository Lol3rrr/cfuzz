use std::sync::Arc;

use tokio::sync::oneshot;

use crate::FuzzTarget;

pub mod process;

pub trait Runner {
    /// Runs the given Target
    ///
    /// The Fuzzing should be canceled when there is a message sent over the cancel-oneshot
    fn run(&self, target: FuzzTarget, cancel: oneshot::Receiver<()>) -> Option<Vec<Vec<u8>>>;
}

pub async fn run_completion<R>(runner: Arc<R>, target: FuzzTarget) -> Option<Vec<Vec<u8>>>
where
    R: Runner + Send + Sync + 'static,
{
    let (sender, recv) = oneshot::channel();

    let (res_sender, res_recv) = oneshot::channel();

    std::thread::spawn(move || {
        match runner.run(target, recv) {
            Some(r) => {
                res_sender.send(r).unwrap();
            }
            None => {
                println!("Error running Target")
            }
        };
    });

    let res = res_recv.await.ok();

    drop(sender);

    res
}

pub async fn run_timeout<R>(
    runner: Arc<R>,
    target: FuzzTarget,
    timeout: std::time::Duration,
) -> Option<Vec<Vec<u8>>>
where
    R: Runner + Send + Sync + 'static,
{
    let (sender, recv) = oneshot::channel();

    let (res_sender, res_recv) = oneshot::channel();

    std::thread::spawn(move || {
        match runner.run(target, recv) {
            Some(r) => {
                res_sender.send(r).unwrap();
            }
            None => {
                println!("Error running Target")
            }
        };
    });

    // Spawn a future that should send a cancel signal after the given Timeout
    tokio::spawn(async move {
        tokio::time::sleep(timeout).await;

        let _ = sender.send(());
    });

    res_recv.await.ok()
}
