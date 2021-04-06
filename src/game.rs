use tokio::sync::RwLock;
use std::sync::atomic::{Ordering, AtomicUsize};

mod data;

use data::*;


//object for one gameshow lobby; includes all necessary data
//lock order to avoid deadlocks: current_question_state -> questions -> player_data -> game_events
pub struct Gameshow
{
    admin: RwLock<String>, //UUID of player that controls the lobby
    open: RwLock<bool>, //whether or not the lobby accepts additional players
    //data related to the game
    player_data: RwLock<Vec<PlayerData>>,
    questions: RwLock<Vec<Question>>,
    game_events: RwLock<Vec<Event>>,
    current_question: AtomicUsize,
    current_question_state: RwLock<QuestionState>,
}

impl Gameshow
{
    pub fn new(admin: String) -> Self
    {
        Gameshow {
            admin: RwLock::new(admin),
            open: RwLock::new(true),
            player_data: RwLock::new(Vec::new()),
            questions: RwLock::new(Vec::new()),
            game_events: RwLock::new(Vec::new()),
            current_question: AtomicUsize::new(0),
            current_question_state: RwLock::new(QuestionState::Results(false)),
        }
    }
}
