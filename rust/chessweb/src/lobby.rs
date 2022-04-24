use std::{sync::{Weak, Arc, Mutex}, collections::HashMap, fmt};

use json::object;
use uuid::Uuid;
use ws::Sender;
// use crate::{chess::ChessBoard};
enum SystemMessageType{
    Info,Error,Warning,Success
}

impl SystemMessageType{
    pub fn str(&self) -> &'static str{
        match self{
            SystemMessageType::Info => "INFO",
            SystemMessageType::Error => "ERROR",
            SystemMessageType::Warning => "WARNING",
            SystemMessageType::Success => "SUCCESS",
        }
    }
}

#[derive(Debug)]
pub struct Lobby {
    lobby_id: String,
    board: ChessBoard,
    pub users: u32,
    connections: HashMap<u32,Sender>,
}



impl Lobby {
    pub fn default() -> Self {
        Self {
            lobby_id: Uuid::new_v4().to_hyphenated().to_string(),
            board: ChessBoard::default(),
            users: 0,
            connections: HashMap::new(),
        }
    }


    pub fn lobby_id(&self)->String{
        self.lobby_id.clone()
    }

    pub fn broad_cast(&self, msg: String){
        self.connections.values().for_each(|v|{v.send(msg.clone());});
    }

    pub fn send_system_message(&self, msg: String, smt: SystemMessageType) {
        let res = object! {
            "type": "chatmessage",
            "args": [
                {"msg": msg.clone(),systemmessage: smt.str()}
            ],
        };
        self.broad_cast(json::stringify(res))
    }

    pub fn join(&mut self, sender: Sender){
        let obj = object! {
            "type": "lobbyJoined",
            lobbyid: self.lobby_id(),
            status: "black",
        };
        _=sender.send(json::stringify(obj));
        
        self.send_system_message(format!("{} joined the lobby","black").to_string(), SystemMessageType::Success);
        self.add_connection(sender);
    }

    fn send_board_to_player(&self, sender: &Sender){
        let fen = self.board.to_fen();
        _=sender.send(json::stringify(object!(
            "type": "fenMap",
            "fen": fen
        )));
    }

    pub fn add_connection(&mut self, sender: Sender){
        self.send_board_to_player(&sender);
        self.connections.insert(sender.connection_id(),sender);
    }
}
