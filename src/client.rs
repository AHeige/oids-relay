
use tungstenite::{connect, Message};


fn main() {
  println!("Connecting to server...");

  let (mut socket, response) = connect("ws://127.0.0.1:9001").expect("Can't connect");
  println!("Connected to the server");
  println!("Response HTTP code: {}", response.status());
  println!("Response contains the following headers:");
  for (ref header, _value) in response.headers() {
      println!("* {}", header);
  }
  socket.send(Message::Text("from client".into())).expect("Could not send");
  std::thread::sleep(std::time::Duration::from_secs(3));
  socket.close(None).expect("Could not close the socket!");

}

