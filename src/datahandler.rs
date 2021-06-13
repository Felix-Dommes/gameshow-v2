use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};

mod database;
mod dataworker;

use crate::game::Gameshow;
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
    pub async fn create_player(&self, name: String) -> Result<String, &'static str>
    {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(DataAccess::CreatePlayer(result_sender, name)).await.map_err(|_err| "db send data access error (dropped channel)")?;
        result_receiver.await.map_err(|_err| "db receive data access result error (dropped channel)")
    }
    
    /// Change name of a player in the database
    pub async fn set_player_name(&self, uuid: String, name: String) -> Result<bool, &'static str>
    {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(DataAccess::SetPlayerName(result_sender, uuid, name)).await.map_err(|_err| "db send data access error (dropped channel)")?;
        result_receiver.await.map_err(|_err| "db receive data access result error (dropped channel)")
    }
    
    /// Get player name for an UUID (if the UUID is valid, else None)
    pub async fn get_player_name(&self, uuid: String) -> Result<Option<String>, &'static str>
    {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(DataAccess::GetPlayerName(result_sender, uuid)).await.map_err(|_err| "db send data access error (dropped channel)")?;
        result_receiver.await.map_err(|_err| "db receive data access result error (dropped channel)")
    }
    
    /// Create a new lobby with the provided UUID as admin
    pub async fn create_lobby(&self, admin_uuid: String, admin_name: String) -> Result<String, &'static str>
    {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(DataAccess::CreateLobby(result_sender, admin_uuid, admin_name)).await.map_err(|_err| "db send data access error (dropped channel)")?;
        result_receiver.await.map_err(|_err| "db receive data access result error (dropped channel)")
    }
    
    /// Get a lobby handle for an UUID (if the UUID is valid, else None)
    pub async fn get_lobby(&self, uuid: String) -> Result<Option<Arc<Gameshow>>, &'static str>
    {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(DataAccess::GetLobby(result_sender, uuid)).await.map_err(|_err| "db send data access error (dropped channel)")?;
        result_receiver.await.map_err(|_err| "db receive data access result error (dropped channel)")
    }
    
    /// Set the available question sets (server side files)
    pub async fn set_question_sets(&self, question_sets: Vec<(String, PathBuf)>) -> Result<(), &'static str>
    {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(DataAccess::SetQuestionSets(result_sender, question_sets)).await.map_err(|_err| "db send data access error (dropped channel)")?;
        result_receiver.await.map_err(|_err| "db receive data access result error (dropped channel)")
    }
    
    /// Get the available question sets (server side files)
    pub async fn get_question_sets(&self) -> Result<Vec<(String, PathBuf)>, &'static str>
    {
        let (result_sender, result_receiver) = oneshot::channel();
        self.sender.send(DataAccess::GetQuestionSets(result_sender)).await.map_err(|_err| "db send data access error (dropped channel)")?;
        result_receiver.await.map_err(|_err| "db receive data access result error (dropped channel)")
    }
}
