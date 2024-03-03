//This is the server side code.
//It will open a socket and listen for clients

use std::net::TcpListener;
use std::io::{Read, Write, ErrorKind};
use std::thread;
use std::sync::mpsc;

//Define constants such ad the IP address and port number
//The message size is also defined as 1000 bits to facilitate long messages
const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 1000;
fn main() {
    //Bind the server to the IP address and port number
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server.set_nonblocking(true).expect("Non-blocking not working");
    //Create a vector to store the clients
    let mut clients = vec![];
    //Create a channel to send messages between threads
    let (tx, rx) = mpsc::channel::<String>();
    loop {
        //If a client connects, accept the connection and store the client
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);
            let tx = tx.clone();
            //Create a new thread to handle the client
            clients.push(socket.try_clone().expect("Failed to clone client"));
            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];
                //Read the message from the client
                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        //Collect everything into a single string
                        let msg = String::from_utf8(msg).expect("Invalid message");
                        println!("{}: {:?}", addr, msg);
                        tx.send(msg).expect("Failed to send message");
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (), //Return a unit type and proceed
                    //If any other kind of error close the connection
                    Err(_) => {
                        println!("Closing connection with: {}", addr);
                        break;
                    }
                }
                //If a message is received, broadcast it to all clients
                if let Ok(msg) = rx.try_recv() {
                    clients = clients.into_iter().filter_map(|mut client| {
                        //Write the message to the client
                        let mut buff = msg.clone().into_bytes();
                        buff.resize(MSG_SIZE, 0);
                        client.write_all(&buff).map(|_| client).ok()
                    }).collect::<Vec<_>>();
                }
                //Sleep for a short time to avoid using too much CPU
                thread::sleep(std::time::Duration::from_millis(100));
            });
        }
    }
}

