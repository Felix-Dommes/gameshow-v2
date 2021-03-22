use uuid::Uuid;
use std::collections::HashMap;


struct Lobby //move to game module
{}

pub struct DataBase
{
    players: HashMap<String, String>,
    lobbies: HashMap<String, Lobby>,
}

impl DataBase
{
    pub fn new() -> Self
    {
        DataBase { players: HashMap::new(), lobbies: HashMap::new() }
    }
    
    pub fn create_player(&mut self, name: String) -> String
    {
        //be sure UUID is REALLY unique
        let mut uuid = String::from("");
        while uuid == "" || self.players.contains_key(&uuid)
        {
            uuid = Uuid::new_v4().to_simple().to_string();
        }
        
        //add player
        self.players.insert(uuid.clone(), name);
        //return UUID
        uuid
    }
    
    pub fn get_player_name(&self, uuid: String) -> String
    {
        self.players[&uuid].clone()
    }
}
