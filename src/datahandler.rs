#![allow(dead_code)] //TODO remove?

use actix_web::rt::task::JoinHandle;
use tokio::sync::{mpsc, oneshot};
use std::sync::Arc;

mod database;
mod dataworker;

use dataworker::{DataAccess, DataWorker};


/// Thread-safe handler to access the database from multiple instances
#[derive(Clone)]
pub struct DataHandler
{
    worker: Arc<JoinHandle<()>>,
    sender: mpsc::Sender<DataAccess>,
}

impl DataHandler
{
    pub fn new() -> Self
    {
        let (worker, sender) = DataWorker::new();
        DataHandler { worker: Arc::new(worker.spawn_worker()), sender: sender }
    }
    
    /// Normally the worker finishes all accesses and then terminates when all data handlers are dropped/closed.
    /// But in case it should be aborted earlier, use this.
    pub fn abort(self)
    {
        self.worker.abort()
    }
    
    /// Create a player in the database
    pub async fn create_player(&self, name: String) -> Result<(), &str>
    {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(DataAccess::CreatePlayer(result_sender, name)).await.map_err(|_err| "db send data access error (dropped channel)")?;
        result_receiver.await.map_err(|_err| "db receive data access result error (dropped channel)")
    }
}
