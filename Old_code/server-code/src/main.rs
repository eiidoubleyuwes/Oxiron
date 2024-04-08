//This is the server side code
//This code is a simple server that listens for incoming connections and messages from clients
use std::net::TcpListener;
use std::io::{Read, Write, ErrorKind};
use std::thread;
use std::sync::mpsc;

//Defining useful constants such as IP and port numbers
//1000 bits is to faciliate long messages
const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 1000;

fn lala(){
    println!("Sleeping to save CPU and other resources");
    thread::sleep(std::time::Duration::from_millis(100));
}

fn main() {
    //Bind the server to the IP address and port number
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server.set_nonblocking(true).expect("Non-blocking not working");
    //Create a vector to store the clients
    let mut clients = vec![];
    //Create a channel to send messages to the clients
    let (tx, rx) = mpsc::channel::<String>();
    loop {
        //If a client connects, print a message and clone the client
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);
            let tx = tx.clone();
            clients.push(socket.try_clone().expect("Failed to clone client"));
            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];
                //Read the message from the client and print it
                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid message");
                        println!("{}: {:?}", addr, msg);
                        tx.send(msg).expect("Failed to send message");
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),//Return a unit type and proceed
                    Err(_) => {
                        println!("Closing connection with: {}", addr);
                        break;
                    }
                }
                lala();
            });
        }
        //If a message is received, send it to all the clients and clear the buffer
        if let Ok(msg) = rx.try_recv() {
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }
        
        
    }
}