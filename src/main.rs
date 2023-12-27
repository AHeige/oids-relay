use std::net::{TcpListener, TcpStream};
use tungstenite::{accept, WebSocket, Message};
use rand::{self, Rng};
use std::sync::{Arc, Mutex};
use serde::{Serialize, ser::SerializeStruct, Deserialize};

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

    #[derive(Debug, Serialize, Deserialize)]
    struct SpaceObject {
        online: bool,
    }

    
   fn generate_random_id() -> u32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..10000)
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

    fn send_message_to_other_clients(sender_id: u32, message: Message, connected_clients: Arc<Mutex<Vec<Arc<Mutex<Client>>>>>) {
        let mut clients = connected_clients.lock().unwrap();
        for client in clients.iter_mut() {
            let msg = message.clone();
            if client.lock().unwrap().id != sender_id {
                
                
                // println!("Sending message to client: {:?}", [client.id, sender_id]);
                
                // let json_string = serde_json::to_string(message).expect("Failed to make JSON");
                let send_object: tungstenite::Message = msg;
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
        let client = Arc::new(Mutex::new(Client::new("Name", cid, ws.clone(), Arc::new(Mutex::new(SpaceObject{online: true})))));
        println!("New client {}", client.lock().unwrap().id);
       
        let cloned_client = client.clone();
        let connected_clients_clone = connected_clients.clone(); // Clone again
        std::thread::spawn(move || {

            let connected_clients_clone_inner = connected_clients_clone.clone(); // Clone again
            
            
            add_new_client(client, connected_clients_clone_inner);


            loop {
                if cloned_client.lock().unwrap().ws.lock().unwrap().can_read() {
                    let msg = cloned_client.lock().unwrap().ws.lock().unwrap().read().expect("Could not read");
                    if msg.is_binary() || msg.is_text() {
                        // println!("{}", msg);
                        let connected_clients_list = connected_clients_clone.clone();
                        let so: SpaceObject = serde_json::from_str(msg.to_text().unwrap()).expect("Failed to parse JSON");

                        cloned_client.lock().unwrap().space_object.lock().unwrap().online = so.online;
                        
                            send_message_to_other_clients(cid,msg, connected_clients_list);
                        
                        //    let send_object: tungstenite::Message = json_string.into();
                        //    websocket.send(send_object).expect("Could not send!")
                        
                    }
                } else {
                    let connected_clients_list = connected_clients_clone.clone();

                    println!("Connection closed by client with id, {}", cid);
                    
                    
                    cloned_client.lock().unwrap().space_object.lock().unwrap().online = false;

                    let space_object_json = serde_json::to_string::<SpaceObject>(&cloned_client.lock().unwrap().space_object.lock().unwrap()).expect("Failed to serialize JSON");

                    let space_object_msg = Message::Text(space_object_json);

                    send_message_to_other_clients(cid, space_object_msg, connected_clients_list);
                    remove_client(cid, connected_clients_clone);
                    break;
                }
            }
        });
        
    }

}
