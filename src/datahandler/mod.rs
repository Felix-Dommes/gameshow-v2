#![allow(dead_code)] //TODO remove?

use actix_web::rt;
use actix_web::rt::task::JoinHandle;
use tokio::sync::{mpsc, oneshot};
use std::sync::Arc;

mod database;

use database::DataBase;


/// Manageable data requests and manipulations
enum DataAccess
{
    CreatePlayer(oneshot::Sender<()>, String),
}

/// Single instance of worker to access the database
struct DataWorker
{
    db: DataBase,
    receiver: mpsc::Receiver<DataAccess>,
}

impl DataWorker
{
    pub fn new() -> (Self, mpsc::Sender<DataAccess>)
    {
        let (sender, receiver) = mpsc::channel(100);
        let worker = DataWorker { db: DataBase::new(), receiver: receiver };
        (worker, sender)
    }
    
    fn spawn_worker(self) -> JoinHandle<()>
    {
        rt::spawn(self.handler())
    }
    
    /// handle received data accesses
    async fn handler(mut self)
    {
        while let Some(access) = self.receiver.recv().await
        {
            match access
            {
                DataAccess::CreatePlayer(result_sender, name) => {
                    let result = self.db.create_player(name);
                    result_sender.send(result).ok();
                },
            }
        }
    }
}


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
    
    pub fn abort(self)
    {
        self.worker.abort()
    }
    
    pub async fn create_player(&self, name: String) -> Result<(), &str>
    {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(DataAccess::CreatePlayer(result_sender, name)).await.map_err(|_err| "db send data access error (dropped channel)")?;
        result_receiver.await.map_err(|_err| "db receive data access result error (dropped channel)")
    }
}
