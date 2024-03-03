//This is the complimentary client code for the server code in the server code file
use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127:0:0:1:6000";
const MSG_SIZE: usize = 1000;
fn main(){
    let mut client = TcpStream::connect(LOCAL).expect("Client failed to connect to the server port and IP");
    client.set_nonblocking(true).expect("Failed to initialize non-blocking");

    let (tx, rx) = mpsc::channel::<String>();   

    thread::spawn(move || loop {
        
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff){
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("Message received: {:?}", msg);
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection with the server was severed");
                break;
            }
        }
        
    });

}