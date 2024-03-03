//This is the complimentary client code for the server code in the server code file
use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127:0:0:1:6000";
const MSG_SIZE: usize = 1000;
fn main(){
    
}