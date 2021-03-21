use tokio::sync::{mpsc, oneshot};

mod database;
mod dataworker;

use dataworker::{DataAccess, DataWorker};


/// Thread-safe handler to access the database from multiple instances
#[derive(Clone)]
pub struct DataHandler
{
    sender: mpsc::Sender<DataAccess>,
}

impl DataHandler
{
    /// Create a new DataHandler and spawn a worker for it.
    /// Normally the worker finishes all accesses and then terminates when all DataHandlers are dropped/closed.
    pub fn new() -> Self
    {
        let (worker, sender) = DataWorker::new();
        worker.spawn_worker();
        DataHandler { sender: sender }
    }
    
    /// Create a player in the database
    pub async fn create_player(&self, name: String) -> Result<String, &str>
    {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(DataAccess::CreatePlayer(result_sender, name)).await.map_err(|_err| "db send data access error (dropped channel)")?;
        result_receiver.await.map_err(|_err| "db receive data access result error (dropped channel)")
    }
}
