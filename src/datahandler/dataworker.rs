use std::{path::PathBuf, sync::Arc};

use actix_web::rt;
use tokio::{
	sync::{mpsc, oneshot},
	task::JoinHandle,
};

use super::database::DataBase;
use crate::game::Gameshow;

const DATA_ACCESS_CAPACITY: usize = 50;

/// Manageable data requests and manipulations
pub enum DataAccess {
	CreatePlayer(oneshot::Sender<String>, String),
	SetPlayerName(oneshot::Sender<bool>, String, String),
	GetPlayerName(oneshot::Sender<Option<String>>, String),
	CreateLobby(oneshot::Sender<String>, String, String),
	GetLobby(oneshot::Sender<Option<Arc<Gameshow>>>, String),
	SetQuestionSets(oneshot::Sender<()>, Vec<(String, PathBuf)>),
	GetQuestionSets(oneshot::Sender<Vec<(String, PathBuf)>>),
}

/// Single instance of worker to access the database
pub struct DataWorker {
	db: DataBase,
	receiver: mpsc::Receiver<DataAccess>,
}

impl DataWorker {
	pub fn new() -> (Self, mpsc::Sender<DataAccess>) {
		let (sender, receiver) = mpsc::channel(DATA_ACCESS_CAPACITY);
		let worker = DataWorker { db: DataBase::new(), receiver };
		(worker, sender)
	}

	/// spawn worker to handle all incoming DataAccesses
	pub fn spawn_worker(self) -> JoinHandle<()> {
		rt::spawn(self.handler())
	}

	/// handle received DataAccesses
	async fn handler(mut self) {
		while let Some(access) = self.receiver.recv().await {
			match access {
				DataAccess::CreatePlayer(result_sender, name) => {
					let result = self.db.create_player(name);
					result_sender.send(result).ok();
				}
				DataAccess::SetPlayerName(result_sender, uuid, name) => {
					let result = self.db.set_player_name(uuid, name);
					result_sender.send(result).ok();
				}
				DataAccess::GetPlayerName(result_sender, uuid) => {
					let result = self.db.get_player_name(uuid);
					result_sender.send(result).ok();
				}
				DataAccess::CreateLobby(result_sender, admin_uuid, admin_name) => {
					let result = self.db.create_lobby(admin_uuid, admin_name);
					result_sender.send(result).ok();
				}
				DataAccess::GetLobby(result_sender, uuid) => {
					let result = self.db.get_lobby(uuid);
					result_sender.send(result).ok();
				}
				DataAccess::SetQuestionSets(result_sender, question_sets) => {
					self.db.set_question_sets(question_sets);
					result_sender.send(()).ok();
				}
				DataAccess::GetQuestionSets(result_sender) => {
					let result = self.db.get_question_sets();
					result_sender.send(result).ok();
				}
			}
		}
	}
}
