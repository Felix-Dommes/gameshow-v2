use std::sync::atomic::{Ordering, AtomicUsize, AtomicI64};
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use tokio::sync::broadcast;

mod questions;
mod events;

pub use events::Event;
pub use questions::find_question_files;

use questions::{Question, QuestionType};
use events::*;

//standard parameters for the game
const INITIAL_MONEY:i64 = 500; //initial amount of money every player owns
const INITIAL_JOKERS:usize = 3; //number of inital jokers every player gets
const NORMAL_Q_MONEY:i64 = 500; //money to get when answering a normal question correctly
const ESTIMATION_Q_MONEY:i64 = 1000; //money to get when winning a estimation question


//object for one gameshow lobby; includes all necessary data and methods to interact
//lock order to avoid deadlocks: admin -> open -> question_set -> current_question_state -> questions -> player_data -> game_events
pub struct Gameshow
{
    //data related to lobby
    admin: RwLock<(String, String)>, //UUID and name of player that controls the lobby
    open: RwLock<bool>, //whether or not the lobby accepts additional players
    param_initial_money: AtomicI64, //see respective constants
    param_initial_jokers: AtomicUsize, //see respective constants
    param_normal_q_money: AtomicI64, //see respective constants
    param_estimation_q_money: AtomicI64, //see respective constants
    question_set: RwLock<String>, //name of selected questions
    
    //data related to the game
    player_data: RwLock<Vec<PlayerData>>,
    questions: RwLock<Vec<Question>>,
    game_events: RwLock<EventManager>,
    current_question: AtomicUsize,
    current_question_state: RwLock<LobbyState>,
}

impl Gameshow
{
    pub fn new(admin: String, name: String) -> Self
    {
        Gameshow {
            admin: RwLock::new((admin, name)),
            open: RwLock::new(true),
            param_initial_money: AtomicI64::new(INITIAL_MONEY),
            param_initial_jokers: AtomicUsize::new(INITIAL_JOKERS),
            param_normal_q_money: AtomicI64::new(NORMAL_Q_MONEY),
            param_estimation_q_money: AtomicI64::new(ESTIMATION_Q_MONEY),
            question_set: RwLock::new(String::new()),
            
            player_data: RwLock::new(Vec::new()),
            questions: RwLock::new(Vec::new()),
            game_events: RwLock::new(EventManager::new()),
            current_question: AtomicUsize::new(0),
            current_question_state: RwLock::new(LobbyState::Menu(false)),
        }
    }
    
    pub async fn get_admin_uuid(&self) -> String
    {
        let admin_access = self.admin.read().await;
        (*admin_access).0.clone()
    }
    
    pub async fn get_admin_name(&self) -> String
    {
        let admin_access = self.admin.read().await;
        (*admin_access).1.clone()
    }
    
    pub async fn set_admin(&self, admin: String, name: String) -> &Self
    {
        {
            let mut admin_access = self.admin.write().await;
            (*admin_access) = (admin, name);
        }
        self
    }
    
    pub async fn is_open(&self) -> bool
    {
        let open_access = self.open.read().await;
        *open_access
    }
    
    pub fn get_initial_money(&self) -> i64
    {
        self.param_initial_money.load(Ordering::Relaxed)
    }
    
    pub fn get_initial_jokers(&self) -> usize
    {
        self.param_initial_jokers.load(Ordering::Relaxed)
    }
    
    pub fn get_normal_q_money(&self) -> i64
    {
        self.param_normal_q_money.load(Ordering::Relaxed)
    }
    
    pub fn get_estimation_q_money(&self) -> i64
    {
        self.param_estimation_q_money.load(Ordering::Relaxed)
    }
    
    pub async fn get_question_set(&self) -> String
    {
        let question_set_access = self.question_set.read().await;
        (*question_set_access).clone()
    }
    
    pub async fn set_open(&self, open: bool) -> &Self
    {
        { //set new preference
            let mut open_access = self.open.write().await;
            (*open_access) = open;
        }
        
        //send update event to clients
        let event = EventType::LobbySettingsUpdate(EventLobbySettingsUpdate {
            open: self.is_open().await,
            initial_money: self.get_initial_money(),
            initial_jokers: self.get_initial_jokers(),
            normal_q_money: self.get_normal_q_money(),
            estimation_q_money: self.get_estimation_q_money(),
            question_set: self.get_question_set().await,
        });
        self.game_events.write().await.add(event);
        
        self
    }
    
    pub async fn update_preferences(&self, initial_money: i64, initial_jokers: usize, normal_q_money: i64, estimation_q_money: i64) -> &Self
    {
        if initial_money >= 0
        {
            self.param_initial_money.store(initial_money, Ordering::Relaxed);
        }
        
        self.param_initial_jokers.store(initial_jokers, Ordering::Relaxed);
        
        if normal_q_money > 0
        {
            self.param_normal_q_money.store(normal_q_money, Ordering::Relaxed);
        }
        
        if estimation_q_money > 0
        {
            self.param_estimation_q_money.store(estimation_q_money, Ordering::Relaxed);
        }
        
        //send update event to clients
        let event = EventType::LobbySettingsUpdate(EventLobbySettingsUpdate {
            open: self.is_open().await,
            initial_money: self.get_initial_money(),
            initial_jokers: self.get_initial_jokers(),
            normal_q_money: self.get_normal_q_money(),
            estimation_q_money: self.get_estimation_q_money(),
            question_set: self.get_question_set().await,
        });
        self.game_events.write().await.add(event);
        
        self
    }
    
    pub async fn set_question_set(&self, question_set: String) -> &Self
    {
        { //update preference
            let mut question_set_access = self.question_set.write().await;
            (*question_set_access) = question_set;
        }
        
        //send update event to clients
        let event = EventType::LobbySettingsUpdate(EventLobbySettingsUpdate {
            open: self.is_open().await,
            initial_money: self.get_initial_money(),
            initial_jokers: self.get_initial_jokers(),
            normal_q_money: self.get_normal_q_money(),
            estimation_q_money: self.get_estimation_q_money(),
            question_set: self.get_question_set().await,
        });
        self.game_events.write().await.add(event);
        
        self
    }
    
    pub async fn get_events(&self) -> Vec<Event>
    {
        self.game_events.read().await.get()
    }
    
    pub async fn subsribe_events(&self) -> broadcast::Receiver<Event>
    {
        self.game_events.read().await.subscribe()
    }
    
    pub async fn get_event_subscribers(&self) -> usize
    {
        self.game_events.read().await.get_subscribers()
    }
    
    
    pub async fn join(&self, uuid: String, mut name: String) -> (bool, Option<String>)
    {
        //check if already joined and return true if so; also check if name is already in use
        let mut name_in_use = false;
        {
            let players_access = self.player_data.read().await;
            for player in (*players_access).iter()
            {
                if player.uuid == uuid
                {
                    return (true, Some(player.name.clone()));
                }
                else if player.name == name
                {
                    name_in_use = true;
                }
            }
        }
        
        //if not already joined, check if allowed to join
        if self.get_admin_uuid().await == uuid
        { //admin can always join with its name
            let mut players_access = self.player_data.write().await;
            let new_player = PlayerData {
                uuid: uuid,
                name: name.clone(),
                jokers: self.get_initial_jokers(),
                money: self.get_initial_money(),
                
                money_bet: 0,
                vs_player: String::new(),
                answer: 0,
            };
            (*players_access).push(new_player);
            return (true, Some(name));
        }
        else if self.is_open().await
        { //others need to have unique name (from others and from admin)
            let admin_name = self.get_admin_name().await;
            let mut players_access = self.player_data.write().await;
            //make sure the name is unique
            while name_in_use || name == admin_name
            {
                name += "2";
                name_in_use = false;
                for player in (*players_access).iter()
                {
                    if player.name == name
                    {
                        name_in_use = true;
                        break;
                    }
                }
            }
            let new_player = PlayerData {
                uuid: uuid,
                name: name.clone(),
                jokers: self.get_initial_jokers(),
                money: self.get_initial_money(),
                
                money_bet: 0,
                vs_player: String::new(),
                answer: 0,
            };
            (*players_access).push(new_player);
            
            return (true, Some(name));
        }
        else
        {
            return (false, None);
        }
    }
    
    pub async fn leave(&self, uuid: String) -> bool
    {
        let mut player_access = self.player_data.write().await;
        let contained = (*player_access).iter().any(|player| player.uuid == uuid);
        (*player_access).retain(|player| player.uuid != uuid);
        contained
        //in the future when drain_filter is not experimental anymore
        //let removed = (*player_access).drain_filter(|player| player.uuid != uuid);
        //removed.count() != 0
    }
    
    //TODO:
    //make PlayerListUpdate events when joined, leaved, answered, bet, etc.
    //answer, bet functions etc.
    //question selection and loading
    //TODO:
    //allow to create a custom question set from sent json data
}


//struct for player data
#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerData
{
    uuid: String,
    name: String,
    jokers: usize,
    money: i64,
    //could also use Option<>, but easier for frontend to handle without
    money_bet: i64,
    vs_player: String,
    answer: usize,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
enum LobbyState
{ //the bool indicates if it is ready to transition to next state
    Menu(bool),
    Results(bool),
    NormalQAnswering(bool),
    BettingQBetting(bool),
    BettingQAnswering(bool),
    EstimationQAnswering(bool),
    VersusQSelecting(bool),
    VersusQAnswering(bool),
    GameEnding,
}
