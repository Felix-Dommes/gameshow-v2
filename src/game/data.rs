use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use std::sync::atomic::AtomicUsize;


//different gameshow question types
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum QuestionType
{
    NormalQuestion,
    BettingQuestion,
    EstimationQuestion,
    VersusQuestion,
}

//struct for question data
#[derive(Serialize, Deserialize, Clone)]
pub struct Question
{
    pub question_type: QuestionType,
    pub category: String,
    pub question: String,
    pub answers: Vec<String>,
    pub correct_answer: usize,
}


//struct for player data
#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerData
{
    pub name: String,
    pub jokers: usize,
    pub money: i64,
    //could also use Option<>, but easier for frontend to handle without
    pub money_bet: i64,
    pub vs_player: String,
    pub answer: usize,
}


//structs for events
#[derive(Serialize, Deserialize, Clone)]
pub struct EventBeginNormalQAnswering
{
    question_type: QuestionType,
    current_question: usize,
    category: String,
    question: String,
    answers: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct EventBeginBettingQBetting
{
    question_type: QuestionType,
    current_question: usize,
    category: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct EventBeginBettingQAnswering
{
    question: String,
    answers: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct EventBeginEstimationQAnswering
{
    question_type: QuestionType,
    current_question: usize,
    category: String,
    question: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct EventBeginVersusQSelecting
{
    question_type: QuestionType,
    current_question: usize,
    category: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct EventBeginVersusQAnswering
{
    question: String,
    answers: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct EventShowResults
{
    correct_answer: usize,
    previous_player_data: Vec<PlayerData>,
    player_data: Vec<PlayerData>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct EventGameEnding
{
    player_data: Vec<PlayerData>,
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
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Event
{
    id: usize,
    event_name: String,
    event: EventType,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum QuestionState
{ //the bool indicates if it is ready to transition to next state
    Results(bool),
    NormalQAnswering(bool),
    BettingQBetting(bool),
    BettingQAnswering(bool),
    EstimationQAnswering(bool),
    VersusQSelecting(bool),
    VersusQAnswering(bool),
    GameEnding,
}
