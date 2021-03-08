use actix_web::rt;
use actix_web::rt::task::JoinHandle;
use tokio::sync::{mpsc, oneshot};

use super::database::DataBase;

const DATA_ACCESS_CAPACITY:usize = 50;


/// Manageable data requests and manipulations
pub enum DataAccess
{
    CreatePlayer(oneshot::Sender<()>, String),
}

/// Single instance of worker to access the database
pub struct DataWorker
{
    db: DataBase,
    receiver: mpsc::Receiver<DataAccess>,
}

impl DataWorker
{
    pub fn new() -> (Self, mpsc::Sender<DataAccess>)
    {
        let (sender, receiver) = mpsc::channel(DATA_ACCESS_CAPACITY);
        let worker = DataWorker { db: DataBase::new(), receiver: receiver };
        (worker, sender)
    }
    
    pub fn spawn_worker(self) -> JoinHandle<()>
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
