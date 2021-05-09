use tokio::sync::RwLock;
use std::sync::atomic::{Ordering, AtomicUsize, AtomicI64};

mod data;

use data::*;

//standard parameters for the game
const INITIAL_MONEY:i64 = 500; //initial amount of money every player owns
const INITIAL_JOKERS:usize = 3; //number of inital jokers every player gets
const NORMAL_Q_MONEY:i64 = 500; //money to get when answering a normal question correctly
const ESTIMATION_Q_MONEY:i64 = 1000; //money to get when winning a estimation question


//object for one gameshow lobby; includes all necessary data
//lock order to avoid deadlocks: admin -> open -> current_question_state -> questions -> player_data -> game_events
pub struct Gameshow
{
    admin: RwLock<String>, //UUID of player that controls the lobby
    open: RwLock<bool>, //whether or not the lobby accepts additional players
    param_initial_money: AtomicI64, //see respective constants
    param_initial_jokers: AtomicUsize, //see respective constants
    param_normal_q_money: AtomicI64, //see respective constants
    param_estimation_q_money: AtomicI64, //see respective constants
    
    //data related to the game
    player_data: RwLock<Vec<PlayerData>>,
    questions: RwLock<Vec<Question>>,
    game_events: RwLock<Vec<Event>>,
    current_question: AtomicUsize,
    current_question_state: RwLock<LobbyState>,
}

impl Gameshow
{
    pub fn new(admin: String) -> Self
    {
        Gameshow {
            admin: RwLock::new(admin),
            open: RwLock::new(false), //initially only admin can join
            param_initial_money: AtomicI64::new(INITIAL_MONEY),
            param_initial_jokers: AtomicUsize::new(INITIAL_JOKERS),
            param_normal_q_money: AtomicI64::new(NORMAL_Q_MONEY),
            param_estimation_q_money: AtomicI64::new(ESTIMATION_Q_MONEY),
            
            player_data: RwLock::new(Vec::new()),
            questions: RwLock::new(Vec::new()),
            game_events: RwLock::new(Vec::new()),
            current_question: AtomicUsize::new(0),
            current_question_state: RwLock::new(LobbyState::Menu(false)),
        }
    }
    
    pub async fn get_admin(&self) -> String
    {
        let admin_access = self.admin.read().await;
        (*admin_access).clone()
    }
    
    pub async fn set_admin(&self, admin: String) -> &Self
    {
        {
            let mut admin_access = self.admin.write().await;
            (*admin_access) = admin;
        }
        self
    }
    
    pub async fn is_open(&self) -> bool
    {
        let open_access = self.open.read().await;
        *open_access
    }
    
    pub async fn set_open(&self, open: bool) -> &Self
    {
        {
            let mut open_access = self.open.write().await;
            (*open_access) = open;
        }
        self
    }
    
    pub fn get_initial_money(&self) -> i64
    {
        self.param_initial_money.load(Ordering::Relaxed)
    }
    
    pub fn set_initial_money(&self, initial_money: i64) -> &Self
    {
        if initial_money >= 0
        {
            self.param_initial_money.store(initial_money, Ordering::Relaxed);
        }
        self
    }
    
    pub fn get_initial_jokers(&self) -> usize
    {
        self.param_initial_jokers.load(Ordering::Relaxed)
    }
    
    pub fn set_initial_jokers(&self, initial_jokers: usize) -> &Self
    {
        self.param_initial_jokers.store(initial_jokers, Ordering::Relaxed);
        self
    }
    
    pub fn get_normal_q_money(&self) -> i64
    {
        self.param_normal_q_money.load(Ordering::Relaxed)
    }
    
    pub fn set_normal_q_money(&self, normal_q_money: i64) -> &Self
    {
        if normal_q_money > 0
        {
            self.param_normal_q_money.store(normal_q_money, Ordering::Relaxed);
        }
        self
    }
    
    pub fn get_estimation_q_money(&self) -> i64
    {
        self.param_estimation_q_money.load(Ordering::Relaxed)
    }
    
    pub fn set_estimation_q_money(&self, estimation_q_money: i64) -> &Self
    {
        if estimation_q_money > 0
        {
            self.param_estimation_q_money.store(estimation_q_money, Ordering::Relaxed);
        }
        self
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
        if self.get_admin().await == uuid || self.is_open().await
        { //join by creating a new player
            let mut players_access = self.player_data.write().await;
            //make sure the name is unique
            while name_in_use
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
}
