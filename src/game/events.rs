use std::fmt;
use serde::{Serialize, Deserialize};
use tokio::sync::broadcast;

use super::PublicPlayerData;
use super::questions::*;


//event manager
pub struct EventManager
{
    events: Vec<Event>,
    event_sender: broadcast::Sender<Event>,
}

impl EventManager
{
    pub fn new() -> Self
    {
        let (sender, _receiver) = broadcast::channel(50);
        EventManager {
            events: Vec::new(),
            event_sender: sender,
        }
    }
    
    pub fn get(&self) -> Vec<Event>
    {
        self.events.clone()
    }
    
    pub fn add(&mut self, event: EventType) -> &mut Self
    {
        let last = self.events.last();
        let mut id = 0;
        if last.is_some()
        {
            id = last.unwrap().id + 1;
        }
        let event_type = format!("{}", event);
        
        let new_event = Event {
            id: id,
            event_name: event_type,
            event: event,
        };
        self.events.push(new_event.clone());
        self.event_sender.send(new_event).ok();
        
        self
    }
    
    pub fn subscribe(&self) -> broadcast::Receiver<Event>
    {
        self.event_sender.subscribe()
    }
    
    pub fn get_subscribers(&self) -> usize
    {
        self.event_sender.receiver_count()
    }
}

//structs for events
#[derive(Serialize, Deserialize, Clone)]
pub struct EventBeginNormalQAnswering
{
    pub question_type: QuestionType,
    pub current_question: usize,
    pub category: String,
    pub question: String,
    pub answers: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct EventBeginBettingQBetting
{
    pub question_type: QuestionType,
    pub current_question: usize,
    pub category: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct EventBeginBettingQAnswering
{
    pub question: String,
    pub answers: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct EventBeginEstimationQAnswering
{
    pub question_type: QuestionType,
    pub current_question: usize,
    pub category: String,
    pub question: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct EventBeginVersusQSelecting
{
    pub question_type: QuestionType,
    pub current_question: usize,
    pub category: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct EventBeginVersusQAnswering
{
    pub question: String,
    pub answers: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct EventShowResults
{
    pub correct_answer: usize,
    pub previous_player_data: Vec<PublicPlayerData>,
    pub player_data: Vec<PublicPlayerData>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct EventGameEnding
{
    pub player_data: Vec<PublicPlayerData>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct EventBackToMenu
{
    pub open: bool,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct EventPlayerListUpdate
{
    pub player_data: Vec<PublicPlayerData>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct EventLobbySettingsUpdate
{
    pub open: bool,
    pub initial_money: i64,
    pub initial_jokers: usize,
    pub normal_q_money: i64,
    pub estimation_q_money: i64,
    pub question_set: String,
}
//combining struct for events
#[derive(Serialize, Deserialize, Clone)]
pub enum EventType
{
    BeginNormalQAnswering(EventBeginNormalQAnswering),
    BeginBettingQBetting(EventBeginBettingQBetting),
    BeginBettingQAnswering(EventBeginBettingQAnswering),
    BeginEstimationQAnswering(EventBeginEstimationQAnswering),
    BeginVersusQSelecting(EventBeginVersusQSelecting),
    BeginVersusQAnswering(EventBeginVersusQAnswering),
    ShowResults(EventShowResults),
    GameEnding(EventGameEnding),
    BackToMenu(EventBackToMenu),
    PlayerListUpdate(EventPlayerListUpdate),
    LobbySettingsUpdate(EventLobbySettingsUpdate),
}

impl fmt::Display for EventType
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            EventType::BeginNormalQAnswering(_) => write!(f, "BeginNormalQAnswering"),
            EventType::BeginBettingQBetting(_) => write!(f, "BeginBettingQBetting"),
            EventType::BeginBettingQAnswering(_) => write!(f, "BeginBettingQAnswering"),
            EventType::BeginEstimationQAnswering(_) => write!(f, "BeginEstimationQAnswering"),
            EventType::BeginVersusQSelecting(_) => write!(f, "BeginVersusQSelecting"),
            EventType::BeginVersusQAnswering(_) => write!(f, "BeginVersusQAnswering"),
            EventType::ShowResults(_) => write!(f, "ShowResults"),
            EventType::GameEnding(_) => write!(f, "GameEnding"),
            EventType::BackToMenu(_) => write!(f, "BackToMenu"),
            EventType::PlayerListUpdate(_) => write!(f, "PlayerListUpdate"),
            EventType::LobbySettingsUpdate(_) => write!(f, "LobbySettingsUpdate"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Event
{
    pub id: usize,
    pub event_name: String,
    pub event: EventType,
}
