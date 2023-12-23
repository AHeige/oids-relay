use std::net::{TcpListener, TcpStream};
use tungstenite::{accept, protocol, client, WebSocket, Message};
use rand::{self, Rng};
use std::sync::{Arc, Mutex};
use serde::{Serialize, ser::SerializeStruct};

/// A WebSocket echo server
fn main () {
    println!("Starting ws server");
    let url = "127.0.0.1:9001";

    struct Client {
        id: u32,
        name: String,
    }

    impl Client {
        fn new(name: &str, id: u32) -> Self {
           
            Self {
                id,
                name: name.to_string()
            }
        }
    }

    impl Serialize for Client {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Client", 2)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.end()
    }
}

    
   fn generate_random_id() -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..10000)
    }

    let mut connected_clients: Arc<Mutex<Vec<Client>>> = Arc::new(Mutex::new(Vec::new()));
    
    pub fn add_new_client(c: Client, connected_clients: Arc<Mutex<Vec<Client>>>) {
        connected_clients.lock().unwrap().push(c)
    }

    fn send_message_to_other_clients(sender_id: u32, message: Message, connected_clients: Arc<Mutex<Vec<Client>>>, websocket: &mut WebSocket<TcpStream>) {
        let clients = connected_clients.lock().unwrap();
        for client in clients.iter() {
            let msg = message.clone();
            if client.id != sender_id {
                
                println!("Sending message to client: {}", client.id);
                
                // let json_string = serde_json::to_string(message).expect("Failed to make JSON");
                let send_object: tungstenite::Message = msg;
                
                websocket.send(send_object).expect("Ops")
            }
        }
    }

    let server = TcpListener::bind(url).expect("Could not bind to adr");
    println!("Waiting for connections on url: {}", url);
    for stream in server.incoming() {

        println!("Spawning new thread...");
        let client = Client::new("Name", generate_random_id());
        let connected_clients_clone = connected_clients.clone();
        
        std::thread::spawn(move || {
            let mut websocket = accept(stream.expect("Stream err")).expect("Could not accept new connections");
            let connected_clients_clone_inner = connected_clients_clone.clone(); // Clone again
            
            add_new_client(client, connected_clients_clone_inner);


            loop {
                if websocket.can_read() {
                    let msg = websocket.read().expect("Could not read");
                    if msg.is_binary() || msg.is_text() {
                        println!("{}", msg);
                        let connected_clients_list = connected_clients_clone.clone();
                        let sender_id = connected_clients_clone.lock().unwrap().last().map(|c| c.id).unwrap_or_default();
                        send_message_to_other_clients(sender_id,msg, connected_clients_list,&mut websocket);
                        //    let send_object: tungstenite::Message = json_string.into();
                        //    websocket.send(send_object).expect("Could not send!")
                        
                    }
                } else {
                    println!("Connection closed by client with id");
                    break;
                }
            }
        });
        
    }

}
