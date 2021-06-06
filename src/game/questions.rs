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

//TODO:
//find question-files and create a list. make it retrievable.
//allow to get the questions, so the lobbies can have a copy of the questions
//allow to create a custom question set from sent json data
