use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;

use crate::constants::{LOCAL, MSG_SIZE};
use crate::utils::sleep;

pub fn start_client(name: String) {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client
        .set_nonblocking(true)
        .expect("failed to initiate non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    tx.send(format!("**{} HAS JOINED**", name))
        .expect("failed to send username");
    match rx.try_recv() {
        Ok(msg) => {
            let mut buff = msg.clone().into_bytes();
            buff.resize(MSG_SIZE, 0);
            client.write_all(&buff).expect("writing to socket failed");
            //        println!("Client: {:?}", client.read_exact(&mut buff));
        }
        Err(_) => (),
    }
    // Loops to check if client has entered message in terminal
    thread::spawn(move || loop {
        // Reads if other clients send a message
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let msg = format!("{}", String::from_utf8_lossy(&msg));
                println!("{}", msg);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let msg = format!("{}: {}", name, msg);
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("writing to socket failed");
                //        println!("Client: {:?}", client.read_exact(&mut buff));
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        sleep();
    });

    println!("----- START OF CHAT SESSION -----");
    loop {
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("reading from stdin failed");
        let msg = buff.trim().to_string();
        if msg == ":q" || tx.send(msg).is_err() {
            break;
        }
    }

    println!("Session terminated");
}
