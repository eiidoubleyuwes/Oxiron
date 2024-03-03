//This is the complimentary client code for the server code in the server code file
use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127:0:0:1:6000";
const MSG_SIZE: usize = 1000;
fn lala(){
    println!("Sleeping to not use all the CPU");
   thread::sleep(Duration::from_millis(100));
}
fn main(){
    let mut client = TcpStream::connect(LOCAL).expect("Client failed to connect to the server port and IP,check if server is on");
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
        match rx.try_recv(){
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("Writing to socket failed");
                println!("Message sent: {:?}", msg);
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }
        lala();
    });
    println!("Write a message:");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("Reading Unable to read message");
        let msg = buff.trim().to_string();
        if msg == ":done" || tx.send(msg).is_err() {break}
    }
    print!("See you soon!");

}