use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;

use crate::game::Gameshow;


pub struct DataBase
{
    players: HashMap<String, String>,
    lobbies: HashMap<String, Arc<Gameshow>>,
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
    
    pub fn set_player_name(&mut self, uuid: String, name: String) -> bool
    {
        let was_there = self.players.contains_key(&uuid);
        self.players.entry(uuid).and_modify(|n| *n = name.clone()).or_insert(name);
        was_there
    }
    
    pub fn get_player_name(&self, uuid: String) -> Option<String>
    {
        if !self.players.contains_key(&uuid)
        {
            None
        }
        else
        {
            Some(self.players[&uuid].clone())
        }
    }
    
    pub fn create_lobby(&mut self, admin: String) -> String
    {
        //be sure UUID is REALLY unique
        let mut uuid = String::from("");
        while uuid == "" || self.lobbies.contains_key(&uuid)
        {
            uuid = Uuid::new_v4().to_simple().to_string();
        }
        
        //add lobby
        self.lobbies.insert(uuid.clone(), Arc::new(Gameshow::new(admin)));
        //return UUID
        uuid
    }
    
    pub fn get_lobby(&self, uuid: String) -> Option<Arc<Gameshow>>
    {
        if !self.lobbies.contains_key(&uuid)
        {
            None
        }
        else
        {
            Some(self.lobbies[&uuid].clone())
        }
    }
}
