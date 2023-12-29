use std::net::{TcpListener, TcpStream};
use tungstenite::{accept, WebSocket, Message};
use std::sync::{Arc, Mutex};
use serde::{Serialize, ser::SerializeStruct};
mod types;
mod factory;
mod util;
use types::SpaceObject;

use crate::{factory::create_new_so, util::generate_random_id};

/// A WebSocket echo server
fn main () {
    println!("Starting ws server");
    let url = "127.0.0.1:9001";

    struct Client {
        id: u32,
        name: String,
        ws: Arc<Mutex<WebSocket<TcpStream>>>,
        space_object: Arc<Mutex<SpaceObject>>,
    }

    impl Client {
        fn new(name: &str, id: u32, ws: Arc<Mutex<WebSocket<TcpStream>>>, space_object: Arc<Mutex<SpaceObject>>) -> Self {
           
            Self {
                id,
                name: name.to_string(),
                ws,
                space_object,
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

   

    

    let connected_clients: Arc<Mutex<Vec<Arc<Mutex<Client>>>>> = Arc::new(Mutex::new(Vec::new()));
    
    pub fn add_new_client(c: Arc<Mutex<Client>>, connected_clients: Arc<Mutex<Vec<Arc<Mutex<Client>>>>>) {
        println!("connected_client length before adding new Client: {}", connected_clients.lock().unwrap().len());
        connected_clients.lock().unwrap().push(c)
    }

    pub fn remove_client(cid: u32, connected_clients: Arc<Mutex<Vec<Arc<Mutex<Client>>>>>) {
        println!("removing client");
        let mut clients = connected_clients.lock().expect("No clients in list");

        println!("Amount of clients before removal: {}", clients.len());

        
        if let Some(index) = clients.iter().position(|x| x.lock().unwrap().id == cid) {
            println!("Client removed with id: {}",  cid);
            clients.remove(index);
            println!("Amount of clients after removal: {}", clients.len());
        } else {
            println!("Client with id {} not found", cid);
        }
        
    }

    fn send_message_to_other_clients(sender_id: u32, message: String, connected_clients: Arc<Mutex<Vec<Arc<Mutex<Client>>>>>) {
       
        // println!("{}",message);
        for client in connected_clients.lock().unwrap().iter_mut() {
            let msg = message.clone();
            if client.lock().unwrap().id != sender_id {
                
                
                // println!("Sending message to client: {:?}", [client.id, sender_id]);
                
                // let json_string = serde_json::to_string(message).expect("Failed to make JSON");
                let send_object = tungstenite::Message::Text(msg);
                if client.lock().unwrap().ws.lock().unwrap().can_write() {
                    client.lock().unwrap().ws.lock().unwrap().send(send_object).expect("Ops")
                }
            }
        }
    }



    let server = TcpListener::bind(url).expect("Could not bind to adr");
    println!("Waiting for connections on url: {}", url);
    for stream in server.incoming() {


        let cid = generate_random_id();
        let ws = Arc::new(Mutex::new(accept(stream.expect("Stream err")).expect("Could not accept new connections")));
        let mut client = Arc::new(Mutex::new(Client::new("Name", cid, ws.clone(), Arc::new(Mutex::new(create_new_so())))));
        let mut client_so = create_new_so(); 
        println!("New client {}", client.lock().unwrap().id);
        
        let cloned_client = client.clone();
        let connected_clients_clone = connected_clients.clone(); // Clone again
        std::thread::spawn(move || {
            
            let connected_clients_clone_inner = connected_clients_clone.clone(); // Clone again
            
            
            add_new_client(client, connected_clients_clone_inner);


            loop {
                if cloned_client.lock().unwrap().ws.lock().unwrap().can_read() {
                    let msg = cloned_client.lock().unwrap().ws.lock().unwrap().read().expect("Could not read").to_string();
                        
                    if msg.is_empty() {
                        println!("Returned an empty JSON string");
                        continue;
                    } 

                    let connected_clients_list = connected_clients_clone.clone();
                    let last_space_object = serde_json::from_str::<SpaceObject>(&msg);

                    println!("{:?}", last_space_object);
                    
                    
                    
                    
                    send_message_to_other_clients(cid,msg, connected_clients_list);
                        
                    
                } else {
                    let connected_clients_list = connected_clients_clone.clone();

                    println!("Connection closed by client with id, {}", cid);

                    client_so.online = false;

                    let space_object_json = serde_json::to_string::<SpaceObject>(&client_so).expect("Failed to serialize JSON");

                    
                    send_message_to_other_clients(cid, space_object_json, connected_clients_list);
                    remove_client(cid, connected_clients_clone);
                    break;
                }
            }
        });
        
    }

}
