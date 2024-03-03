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
}

