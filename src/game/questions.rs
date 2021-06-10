use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};


const QUESTIONS_FOLDER:&'static str = "./questions";


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


//read questions from a JSON file and return them
fn read_questions(filename: impl AsRef<Path>) -> std::io::Result<Vec<Question>>
{
    let json_string = fs::read_to_string(filename)?;
    let questions: Vec<Question> = serde_json::from_str(&json_string)?;
    Ok(questions)
}

pub fn find_question_files() -> std::io::Result<Vec<(String, PathBuf)>>
{
    let mut list = Vec::new();
    
    let questions_folder = fs::read_dir(QUESTIONS_FOLDER)?;
    for entry in questions_folder
    {
        let path = entry?.path();
        if !path.is_dir()
        {
            let file_name = path.file_stem();
            let file_type = path.extension();
            if file_type.is_some()
            {
                let file_name = file_name.unwrap().to_string_lossy().into_owned();
                let file_type = file_type.unwrap().to_string_lossy().to_lowercase();
                if file_type == "json" && file_name != "questions-example"
                {
                    list.push((file_name, path));
                }
            }
        }
    }
    
    Ok(list)
}
