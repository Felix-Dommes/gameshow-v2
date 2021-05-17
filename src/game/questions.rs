use serde::{Serialize, Deserialize};


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
