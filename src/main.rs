use json::{object, JsonValue};
use lobby::Lobby;
use rocket::{fs::NamedFile, State};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process,
    sync::{Arc, Mutex},
    thread::{self}, fmt,
};
use ws::{listen, Handler, Sender};

mod chess;
mod lobby;

type LobbyMap = Arc<Mutex<HashMap<String, Lobby>>>;

#[macro_use]
extern crate rocket;

#[get("/<file..>?<lobbyid>")]
async fn files(
    file: PathBuf,
    lobbyid: Option<&str>,
    lobbys: &State<LobbyMap>,
) -> Result<NamedFile, String> {
    if let Some(key) = lobbyid {
        let lobbys_mutex = lobbys.lock();
        if lobbys_mutex.is_err() {
            return Err("Internal Error (aquiring shared lobbydata)".to_owned());
        }
        let mut lobby_lock_guard = lobbys_mutex.unwrap();
        let lobbys = &mut *lobby_lock_guard;

        let lobby = lobbys.get_mut(key);
        if lobby.is_none() {
            return Err("Lobby not found".to_owned());
        }
    }
    println!("{:?}",file);
    let mut file = file;
    if file.to_str().unwrap().len() == 0{
        file = Path::new("index.html").to_path_buf();
    }
    NamedFile::open(Path::new("HTTP/").join(file))
        .await
        .map_err(|e| e.to_string())
}
struct WebSocketConnection{
    out: Sender,
    lobbys: LobbyMap,
}

impl fmt::Debug for WebSocketConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.out)
    }
}

impl WebSocketConnection{
    fn send(&self, val: JsonValue) -> Result<(), ws::Error>{
        self.out.send(json::stringify(val))
    }
}

impl WebSocketConnection{
    fn new(out: Sender, lobbys: LobbyMap) -> Self{
        WebSocketConnection{out,lobbys}
    }
}

impl Handler for WebSocketConnection {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let msg = msg.as_text();
        if msg.is_err(){
            eprintln!("Error casting websocket message as text{}",msg.err().unwrap());
            return Ok(());
        }
        let msg = msg.unwrap();
        let res = json::parse(msg);
        if res.is_err() {
            eprintln!("Error parsing websocket message to json{}",res.err().unwrap());
            return  Ok(());
        }
        let res = res.unwrap();
        let mut lobby_map = self.lobbys.lock().expect("Error");
        match res["type"].as_str().unwrap() {
            "createLobby" => {
                println!("Creating lobby");
                let mut l = Lobby::default();
                l.add_connection(self.out.clone());
                println!("{:?}",l);
                let lobby_id = l.lobby_id();
                lobby_map.insert(l.lobby_id(), l);
                let _ = self.send(object!(
                    "type": "lobbycreated",
                    "lobbyid": lobby_id
                ));
                
            },
            "joinLobby" => {
                let lobby_id = res["lobbyid"].as_str().unwrap();
                
                let lobby = lobby_map.get_mut(lobby_id);
                if let Some(lobby) = lobby{
                    lobby.join(self.out.clone())
                }

            },
            "doMove" => {

            },
            "chatmessage" => {
                let lobby_id = res["lobbyid"].as_str().unwrap();
                let lobby = lobby_map.get_mut(lobby_id);
                if let Some(lobby) = lobby{
                    lobby.broad_cast(json::stringify(res))
                }
            }
            _=>{
                let _=self.send(object!(
                    error: "unknown packet"
                ));
            }
        }

        Ok(())
    }
}

#[launch]
fn rocket() -> _ {
    let lobbys: LobbyMap = Arc::new(Mutex::new(HashMap::new()));
    let lobbys_c = lobbys.clone();
    thread::spawn(move || {
        println!("Starting Websocket at port 7721");
        let res = listen("localhost:7721", |out| {
            WebSocketConnection::new(out,lobbys_c.clone())
            
        });
        if res.is_err() {
            println!("{}", res.err().unwrap());
            process::exit(1)
        }
    });

    let lock = lobbys.lock();
    lock.unwrap().insert("debug".to_owned(), Lobby::default());
    rocket::build().mount("/", routes![files]).manage(lobbys.clone())
}
