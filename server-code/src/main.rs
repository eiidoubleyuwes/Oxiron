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
            
    }
}

